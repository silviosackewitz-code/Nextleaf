use super::*;
use chrono::{DateTime, Datelike, Days, Months, NaiveDate};
use sqlx::SqliteConnection;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RepeatRule {
    pub id: String,
    pub frequency: String,
    pub interval: u32,
    pub start: String,
    pub until: Option<String>,
    pub due_days: Option<u32>,
    pub paused: bool,
}
pub fn date(value: &str) -> Result<NaiveDate, &'static str> {
    let parsed =
        NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|_| "Use a valid YYYY-MM-DD date")?;
    if parsed.to_string() != value || parsed.year() < 1900 {
        return Err("Dates must be on or after 1900-01-01");
    }
    Ok(parsed)
}
impl RepeatRule {
    pub fn validate(&self) -> Result<(), &'static str> {
        if Uuid::parse_str(&self.id).is_err() {
            return Err("Invalid repeat identity");
        }
        if !matches!(
            self.frequency.as_str(),
            "daily" | "weekly" | "monthly" | "yearly"
        ) || !(1..=1000).contains(&self.interval)
        {
            return Err("Choose a repeat interval from 1 to 1000");
        }
        let start = date(&self.start)?;
        if self
            .until
            .as_deref()
            .map(date)
            .transpose()?
            .is_some_and(|end| end < start)
        {
            return Err("The repeat end cannot precede its start");
        }
        if self.due_days.is_some_and(|days| days > 365) {
            return Err("Due dates support up to 365 days of advance notice");
        }
        Ok(())
    }
    fn occurrence(&self, index: u32) -> Option<NaiveDate> {
        let start = date(&self.start).ok()?;
        let count = index.checked_mul(self.interval)?;
        // Anchor every occurrence to the original day, so Jan 31 -> Feb 28 -> Mar 31.
        let result = match self.frequency.as_str() {
            "daily" => start.checked_add_days(Days::new(count.into())),
            "weekly" => start.checked_add_days(Days::new(u64::from(count) * 7)),
            "monthly" => start.checked_add_months(Months::new(count)),
            "yearly" => start.checked_add_months(Months::new(count.checked_mul(12)?)),
            _ => None,
        }?;
        (self
            .until
            .as_deref()
            .is_none_or(|end| result.to_string().as_str() <= end)
            && result.year() <= 9999)
            .then_some(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Clock {
    pub timezone: String,
    pub updated_at: String,
}
impl Default for Clock {
    fn default() -> Self {
        Self {
            timezone: "UTC".into(),
            updated_at: "1970-01-01T00:00:00Z".into(),
        }
    }
}
impl Clock {
    pub fn validate(&self) -> Result<(), String> {
        self.timezone
            .parse::<chrono_tz::Tz>()
            .map_err(|_| "Unknown time zone")?;
        DateTime::parse_from_rfc3339(&self.updated_at)
            .map_err(|_| "Invalid time zone timestamp")?;
        Ok(())
    }
    pub fn today(&self, now: DateTime<Utc>) -> NaiveDate {
        now.with_timezone(
            &self
                .timezone
                .parse::<chrono_tz::Tz>()
                .unwrap_or(chrono_tz::UTC),
        )
        .date_naive()
    }
    pub fn newer_than(&self, other: &Self) -> bool {
        (
            DateTime::parse_from_rfc3339(&self.updated_at).ok(),
            &self.timezone,
        ) > (
            DateTime::parse_from_rfc3339(&other.updated_at).ok(),
            &other.timezone,
        )
    }
}
pub(crate) async fn read_clock(conn: &mut SqliteConnection) -> Result<Clock, sqlx::Error> {
    let (timezone, updated_at) =
        sqlx::query_as("SELECT timezone,updated_at FROM schedule_clock WHERE singleton=1")
            .fetch_one(conn)
            .await?;
    Ok(Clock {
        timezone,
        updated_at,
    })
}
pub(crate) async fn write_clock(
    conn: &mut SqliteConnection,
    clock: &Clock,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE schedule_clock SET timezone=?,updated_at=? WHERE singleton=1")
        .bind(&clock.timezone)
        .bind(&clock.updated_at)
        .execute(conn)
        .await?;
    Ok(())
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Timezone {
    timezone: String,
}
pub(super) async fn report_timezone(
    State(state): State<AppState>,
    Json(input): Json<Timezone>,
) -> ApiResult<Value> {
    let clock = Clock {
        timezone: input.timezone,
        updated_at: stamp(),
    };
    clock
        .validate()
        .map_err(|_| bad_request("Unknown time zone"))?;
    let mut tx = state
        .pool
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(internal)?;
    write_clock(&mut tx, &clock).await.map_err(internal)?;
    tx.commit().await.map_err(internal)?;
    tick(&state.pool).await.map_err(internal)?;
    Ok(Json(json!({"timezone":clock.timezone})))
}

pub async fn tick(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
    advance(&mut tx, Utc::now()).await?;
    tx.commit().await
}
pub async fn run(pool: SqlitePool) {
    let mut timer = tokio::time::interval(std::time::Duration::from_secs(30));
    loop {
        timer.tick().await;
        if pool.is_closed() {
            return;
        }
        if let Err(error) = tick(&pool).await {
            tracing::error!("Scheduling failed: {error}");
        }
    }
}
pub(crate) async fn advance(
    conn: &mut SqliteConnection,
    now: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    // Connected desktops leave generation to the server; offline edits still remain local.
    let remote: Option<String> = sqlx::query_scalar(
        "SELECT json_extract(state_json,'$.remote_url') FROM sync_client WHERE singleton=1",
    )
    .fetch_optional(&mut *conn)
    .await?
    .flatten();
    if remote.is_some_and(|s| !s.is_empty()) {
        return Ok(());
    }
    let clock = read_clock(conn).await?;
    if clock.updated_at == Clock::default().updated_at {
        return Ok(());
    }
    let today = clock.today(now);
    sqlx::query("UPDATE tasks SET status='next',starred=1,defer_until=NULL,updated_at=? WHERE status='scheduled' AND defer_until IS NOT NULL AND substr(defer_until,1,10)<=?")
        .bind(now.to_rfc3339()).bind(today.to_string()).execute(&mut *conn).await?;
    let templates: Vec<(String, String)> = sqlx::query_as("SELECT t.id,t.recurrence FROM tasks t LEFT JOIN projects p ON p.id=t.project_id WHERE t.status='recurring' AND t.recurrence IS NOT NULL AND (p.id IS NULL OR p.status='active') ORDER BY t.id").fetch_all(&mut *conn).await?;
    let existing: std::collections::HashSet<String> =
        sqlx::query_scalar::<_, String>("SELECT id FROM tasks")
            .fetch_all(&mut *conn)
            .await?
            .into_iter()
            .collect();
    let mut remaining = 100;
    for (template, raw) in templates {
        let rule: RepeatRule =
            serde_json::from_str(&raw).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        rule.validate()
            .map_err(|e| sqlx::Error::Protocol(e.into()))?;
        if rule.paused {
            continue;
        }
        // shortcut: scan the bounded date history; add an indexed cursor for large repeat archives.
        for index in 0..100_000 {
            let Some(day) = rule.occurrence(index) else {
                break;
            };
            let Some(appears) = day.checked_sub_days(Days::new(rule.due_days.unwrap_or(0).into()))
            else {
                break;
            };
            if appears > today {
                break;
            }
            let id = Uuid::new_v5(
                &Uuid::NAMESPACE_URL,
                format!("nextleaf:{template}:{}:{day}", rule.id).as_bytes(),
            )
            .to_string();
            if existing.contains(&id) {
                continue;
            }
            let result = sqlx::query("INSERT OR IGNORE INTO tasks(id,project_id,area_id,title,notes,status,sort_order,due_date,defer_until,completed_at,created_at,updated_at,starred,waiting_for,estimated_minutes,energy,recurrence) SELECT ?,project_id,area_id,title,notes,'next',(SELECT COALESCE(MAX(sort_order),0)+100 FROM tasks WHERE project_id IS source.project_id),?,NULL,NULL,?,?,1,waiting_for,estimated_minutes,energy,NULL FROM tasks source WHERE id=?")
                .bind(&id).bind(rule.due_days.map(|_| day.to_string())).bind(now.to_rfc3339()).bind(now.to_rfc3339()).bind(&template).execute(&mut *conn).await?;
            if result.rows_affected() > 0 {
                sqlx::query("INSERT INTO task_tags(task_id,tag_id) SELECT ?,tag_id FROM task_tags WHERE task_id=?").bind(&id).bind(&template).execute(&mut *conn).await?;
                remaining -= 1;
                if remaining == 0 {
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn rule(start: &str, frequency: &str) -> RepeatRule {
        RepeatRule {
            id: uid(),
            start: start.into(),
            frequency: frequency.into(),
            interval: 1,
            until: None,
            due_days: None,
            paused: false,
        }
    }
    fn instant(s: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
    }
    async fn pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }
    async fn advance_at(pool: &SqlitePool, now: &str) {
        let mut tx = pool.begin_with("BEGIN IMMEDIATE").await.unwrap();
        advance(&mut tx, instant(now)).await.unwrap();
        tx.commit().await.unwrap();
    }
    async fn zone(pool: &SqlitePool, timezone: &str) {
        write_clock(
            &mut *pool.acquire().await.unwrap(),
            &Clock {
                timezone: timezone.into(),
                updated_at: "2026-10-18T12:00:00Z".into(),
            },
        )
        .await
        .unwrap();
    }
    async fn insert(
        pool: &SqlitePool,
        id: &str,
        status: &str,
        day: Option<&str>,
        repeat: Option<RepeatRule>,
    ) {
        sqlx::query("INSERT INTO tasks(id,title,notes,status,sort_order,defer_until,created_at,updated_at,recurrence) VALUES(?,?,'Keep notes',?,100,?,'2026-10-18T12:00:00Z','2026-10-18T12:00:00Z',?)").bind(id).bind(id).bind(status).bind(day).bind(repeat.map(sqlx::types::Json)).execute(pool).await.unwrap();
    }
    #[test]
    fn anchored_month_year_end_and_validation() {
        let mut monthly = rule("2027-01-31", "monthly");
        assert_eq!(monthly.occurrence(1).unwrap().to_string(), "2027-02-28");
        assert_eq!(monthly.occurrence(2).unwrap().to_string(), "2027-03-31");
        monthly.until = Some("2027-02-28".into());
        assert!(monthly.occurrence(2).is_none());
        assert_eq!(
            rule("2024-02-29", "yearly")
                .occurrence(4)
                .unwrap()
                .to_string(),
            "2028-02-29"
        );
        monthly.interval = 0;
        assert!(monthly.validate().is_err());
        assert!(date("2026-2-1").is_err());
        assert!(date("2026-02-30").is_err());
        assert!(Clock {
            timezone: "Unknown/Zone".into(),
            ..Clock::default()
        }
        .validate()
        .is_err());
    }
    #[test]
    fn travel_and_daylight_saving_use_calendar_days() {
        let berlin = Clock {
            timezone: "Europe/Berlin".into(),
            ..Clock::default()
        };
        let mexico = Clock {
            timezone: "America/Hermosillo".into(),
            ..Clock::default()
        };
        assert_eq!(
            berlin.today(instant("2026-10-18T22:30:00Z")).to_string(),
            "2026-10-19"
        );
        assert_eq!(
            mexico.today(instant("2026-10-18T22:30:00Z")).to_string(),
            "2026-10-18"
        );
        assert_eq!(
            berlin.today(instant("2026-03-28T23:30:00Z")).to_string(),
            "2026-03-29"
        );
        assert_eq!(
            berlin.today(instant("2026-10-25T23:30:00Z")).to_string(),
            "2026-10-26"
        );
    }
    #[tokio::test]
    async fn dates_activate_once_and_preserve_other_states() {
        let p = pool().await;
        zone(&p, "Europe/Berlin").await;
        for state in ["scheduled", "waiting", "completed", "trash", "recurring"] {
            insert(&p, state, state, Some("2026-10-19"), None).await;
        }
        advance_at(&p, "2026-10-18T21:59:59Z").await;
        assert_eq!(
            sqlx::query_scalar::<_, String>("SELECT status FROM tasks WHERE id='scheduled'")
                .fetch_one(&p)
                .await
                .unwrap(),
            "scheduled"
        );
        advance_at(&p, "2026-10-18T22:00:00Z").await;
        let row: (String, bool, Option<String>, String) = sqlx::query_as(
            "SELECT status,starred,defer_until,updated_at FROM tasks WHERE id='scheduled'",
        )
        .fetch_one(&p)
        .await
        .unwrap();
        assert_eq!((&*row.0, row.1, row.2), ("next", true, None));
        advance_at(&p, "2026-10-20T00:00:00Z").await;
        assert_eq!(
            sqlx::query_scalar::<_, String>("SELECT updated_at FROM tasks WHERE id='scheduled'")
                .fetch_one(&p)
                .await
                .unwrap(),
            row.3
        );
        for state in ["waiting", "completed", "trash", "recurring"] {
            assert_eq!(
                sqlx::query_scalar::<_, String>("SELECT status FROM tasks WHERE id=?")
                    .bind(state)
                    .fetch_one(&p)
                    .await
                    .unwrap(),
                state
            );
        }
    }
    #[tokio::test]
    async fn repeat_copies_keep_metadata_and_never_resurrect_deleted_occurrences() {
        let p = pool().await;
        zone(&p, "Europe/Berlin").await;
        let mut repeat = rule("2026-10-20", "daily");
        repeat.due_days = Some(1);
        repeat.until = Some("2026-10-21".into());
        insert(&p, "template", "recurring", None, Some(repeat.clone())).await;
        sqlx::query("INSERT INTO tags(id,name,kind) VALUES('contact','Contact','person')")
            .execute(&p)
            .await
            .unwrap();
        sqlx::query("INSERT INTO task_tags VALUES('template','contact')")
            .execute(&p)
            .await
            .unwrap();
        advance_at(&p, "2026-10-18T22:00:00Z").await;
        let rows: Vec<(String, String, String, bool, Option<String>)> = sqlx::query_as(
            "SELECT id,notes,due_date,starred,recurrence FROM tasks WHERE id!='template'",
        )
        .fetch_all(&p)
        .await
        .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(
            (&*rows[0].1, &*rows[0].2, rows[0].3, rows[0].4.clone()),
            ("Keep notes", "2026-10-20", true, None)
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM task_tags WHERE task_id=?")
                .bind(&rows[0].0)
                .fetch_one(&p)
                .await
                .unwrap(),
            1
        );
        sqlx::query("UPDATE tasks SET status='trash' WHERE id=?")
            .bind(&rows[0].0)
            .execute(&p)
            .await
            .unwrap();
        advance_at(&p, "2026-10-22T12:00:00Z").await;
        advance_at(&p, "2026-10-22T12:00:00Z").await;
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tasks")
                .fetch_one(&p)
                .await
                .unwrap(),
            3
        );
        assert_eq!(
            sqlx::query_scalar::<_, String>("SELECT status FROM tasks WHERE id=?")
                .bind(&rows[0].0)
                .fetch_one(&p)
                .await
                .unwrap(),
            "trash"
        );
        repeat.paused = true;
        insert(&p, "paused", "recurring", None, Some(repeat)).await;
        insert(&p, "unknown-import", "recurring", None, None).await;
        advance_at(&p, "2026-10-22T12:00:00Z").await;
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tasks")
                .fetch_one(&p)
                .await
                .unwrap(),
            5
        );
    }
    #[tokio::test]
    async fn connected_desktop_does_not_execute_server_schedule() {
        let p = pool().await;
        zone(&p, "Europe/Berlin").await;
        insert(&p, "deferred", "scheduled", Some("2026-10-19"), None).await;
        insert(
            &p,
            "template",
            "recurring",
            None,
            Some(rule("2026-10-19", "daily")),
        )
        .await;
        sqlx::query("INSERT INTO sync_client VALUES(1,?)")
            .bind(r#"{"remote_url":"https://example.invalid"}"#)
            .execute(&p)
            .await
            .unwrap();
        advance_at(&p, "2026-10-20T12:00:00Z").await;
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tasks")
                .fetch_one(&p)
                .await
                .unwrap(),
            2
        );
        assert_eq!(
            sqlx::query_scalar::<_, String>("SELECT status FROM tasks WHERE id='deferred'")
                .fetch_one(&p)
                .await
                .unwrap(),
            "scheduled"
        );
    }
}
