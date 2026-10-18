use chrono::{NaiveDate, Utc};
use serde::Deserialize;
use serde_json::Value;
use sqlx::{sqlite::SqliteConnectOptions, Connection, SqliteConnection};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Deserialize)]
struct Snapshot {
    format: String,
    total: usize,
    has_more: bool,
    tags: Vec<Tag>,
    items: Vec<Value>,
}
#[derive(Deserialize)]
struct Tag {
    name: String,
    #[serde(rename = "type")]
    kind: String,
}
#[derive(Deserialize)]
struct Item {
    id: String,
    name: String,
    #[serde(rename = "type")]
    kind: String,
    state: String,
    created: String,
    completed: Option<String>,
    #[serde(default)]
    note: String,
    tags: Vec<String>,
    parent: Option<Parent>,
    starred: bool,
    startdate: Option<String>,
    duedate: Option<String>,
    waitingfor: Option<String>,
    etime: Option<i64>,
    energy: Option<i64>,
}
#[derive(Deserialize)]
struct Parent {
    id: String,
}

fn validate(snapshot: &Snapshot) -> Result<Vec<Item>> {
    if snapshot.format != "nirvana-mcp-v1"
        || snapshot.has_more
        || snapshot.total != snapshot.items.len()
    {
        return Err("Incomplete or unsupported snapshot".into());
    }
    let items: Vec<Item> = snapshot
        .items
        .iter()
        .cloned()
        .map(serde_json::from_value)
        .collect::<std::result::Result<_, _>>()?;
    let mut ids = HashSet::new();
    let mut tags = HashSet::new();
    for tag in &snapshot.tags {
        if tag.name.trim().is_empty()
            || !matches!(tag.kind.as_str(), "area" | "context")
            || !tags.insert(&tag.name)
        {
            return Err("Invalid or duplicate tag".into());
        }
    }
    for item in &items {
        if item.id.is_empty() || !ids.insert(&item.id) || item.name.trim().is_empty() {
            return Err("Invalid or duplicate item identity/name".into());
        }
        let states: &[&str] = match item.kind.as_str() {
            "project" => &["active", "someday", "logbook"],
            "task" => &[
                "inbox",
                "next",
                "waiting",
                "scheduled",
                "someday",
                "later",
                "recurring",
                "logbook",
                "trash",
            ],
            _ => return Err("Unsupported item type; extend mapping before import".into()),
        };
        if !states.contains(&item.state.as_str())
            || (item.state == "logbook" && item.completed.is_none())
        {
            return Err("Unsupported state/completion combination".into());
        }
        for date in [
            Some(&item.created),
            item.completed.as_ref(),
            item.startdate.as_ref(),
            item.duedate.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            if NaiveDate::parse_from_str(date, "%Y-%m-%d")?.to_string() != *date {
                return Err("Invalid source date".into());
            }
        }
        if item.etime.is_some_and(|n| n < 0) || item.energy.is_some_and(|n| !(0..=3).contains(&n)) {
            return Err("Invalid time or energy".into());
        }
        if item.tags.iter().any(|t| !tags.contains(t))
            || item
                .tags
                .iter()
                .filter(|t| {
                    snapshot
                        .tags
                        .iter()
                        .any(|g| g.name == **t && g.kind == "area")
                })
                .count()
                > 1
        {
            return Err("Unknown tag or multiple areas; explicit mapping required".into());
        }
        if let Some(parent) = &item.parent {
            if item.kind != "task"
                || !items
                    .iter()
                    .any(|p| p.id == parent.id && p.kind == "project")
            {
                return Err("Missing or invalid parent project".into());
            }
        }
    }
    Ok(items)
}

async fn import(conn: &mut SqliteConnection, snapshot: Snapshot) -> Result<(usize, usize)> {
    let items = validate(&snapshot)?;
    let now = Utc::now().to_rfc3339();
    let mut tx = conn.begin().await?;
    let mut targets = HashMap::new();
    let mut existing = HashSet::new();
    for (item, raw) in items.iter().zip(&snapshot.items) {
        let previous: Option<(String, String)> =
            sqlx::query_as("SELECT target_id,raw_json FROM nirvana_imports WHERE source_id=?")
                .bind(&item.id)
                .fetch_optional(&mut *tx)
                .await?;
        let target = if let Some((id, json)) = previous {
            if serde_json::from_str::<Value>(&json)? != *raw {
                return Err(
                    "Source record changed since import; refusing to overwrite local edits".into(),
                );
            }
            let table = if item.kind == "project" {
                "projects"
            } else {
                "tasks"
            };
            let present: i64 =
                sqlx::query_scalar(&format!("SELECT COUNT(*) FROM {table} WHERE id=?"))
                    .bind(&id)
                    .fetch_one(&mut *tx)
                    .await?;
            if present != 1 {
                return Err("Previously imported target is missing".into());
            }
            existing.insert(item.id.clone());
            id
        } else {
            Uuid::new_v4().to_string()
        };
        targets.insert(item.id.clone(), target);
    }
    let mut area_ids = HashMap::new();
    let mut tag_ids = HashMap::new();
    for tag in &snapshot.tags {
        let table = if tag.kind == "area" { "areas" } else { "tags" };
        let found: Vec<String> =
            sqlx::query_scalar(&format!("SELECT id FROM {table} WHERE name=?"))
                .bind(&tag.name)
                .fetch_all(&mut *tx)
                .await?;
        if found.len() > 1 {
            return Err("Ambiguous existing area name".into());
        }
        let id = match found.first() {
            Some(id) => id.clone(),
            None => {
                let id = Uuid::new_v4().to_string();
                if tag.kind == "area" {
                    sqlx::query("INSERT INTO areas(id,name,sort_order,created_at,updated_at) VALUES(?,?,(SELECT COALESCE(MAX(sort_order),0)+100 FROM areas),?,?)")
                        .bind(&id).bind(&tag.name).bind(&now).bind(&now).execute(&mut *tx).await?;
                } else {
                    sqlx::query("INSERT INTO tags(id,name) VALUES(?,?)")
                        .bind(&id)
                        .bind(&tag.name)
                        .execute(&mut *tx)
                        .await?;
                }
                id
            }
        };
        if tag.kind == "area" {
            area_ids.insert(tag.name.clone(), id);
        } else {
            tag_ids.insert(tag.name.clone(), id);
        }
    }
    // Parents first; returned order is retained only as a provisional display order.
    for kind in ["project", "task"] {
        for (item, raw) in items
            .iter()
            .zip(&snapshot.items)
            .filter(|(i, _)| i.kind == kind && !existing.contains(&i.id))
        {
            let id = &targets[&item.id];
            let area = item.tags.iter().find_map(|t| area_ids.get(t));
            let completed = item.completed.as_deref();
            let status = if item.state == "trash" {
                "trash"
            } else if completed.is_some() {
                "completed"
            } else {
                &item.state
            };
            if kind == "project" {
                sqlx::query("INSERT INTO projects(id,area_id,name,notes,project_type,status,sort_order,created_at,updated_at,completed_at) VALUES(?,?,?,?,'parallel',?,(SELECT COALESCE(MAX(sort_order),0)+100 FROM projects),?,?,?)")
                    .bind(id).bind(area).bind(&item.name).bind(&item.note).bind(status).bind(&item.created).bind(&now).bind(completed).execute(&mut *tx).await?;
            } else {
                let parent = item.parent.as_ref().map(|p| &targets[&p.id]);
                sqlx::query("INSERT INTO tasks(id,project_id,area_id,title,notes,status,sort_order,due_date,defer_until,completed_at,created_at,updated_at,starred,waiting_for,estimated_minutes,energy) VALUES(?,?,?,?,?,?,(SELECT COALESCE(MAX(sort_order),0)+100 FROM tasks),?,?,?,?,?,?,?,?,?)")
                    .bind(id).bind(parent).bind(area).bind(&item.name).bind(&item.note).bind(status).bind(&item.duedate).bind(&item.startdate).bind(completed).bind(&item.created).bind(&now).bind(item.starred).bind(&item.waitingfor).bind(item.etime).bind(item.energy).execute(&mut *tx).await?;
                for tag in &item.tags {
                    if let Some(tag_id) = tag_ids.get(tag) {
                        sqlx::query("INSERT INTO task_tags(task_id,tag_id) VALUES(?,?)")
                            .bind(id)
                            .bind(tag_id)
                            .execute(&mut *tx)
                            .await?;
                    }
                }
            }
            sqlx::query("INSERT INTO nirvana_imports(source_id,target_id,item_type,raw_json) VALUES(?,?,?,?)")
                .bind(&item.id).bind(id).bind(kind).bind(raw.to_string()).execute(&mut *tx).await?;
        }
    }
    tx.commit().await?;
    Ok((items.len() - existing.len(), existing.len()))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err(
            "Usage: import_nirvana SNAPSHOT.json EXISTING_DATABASE.db (back up first)".into(),
        );
    }
    let snapshot: Snapshot = serde_json::from_slice(&std::fs::read(&args[1])?)?;
    validate(&snapshot)?;
    let options = SqliteConnectOptions::new()
        .filename(&args[2])
        .create_if_missing(false)
        .foreign_keys(true);
    let mut conn = SqliteConnection::connect_with(&options).await?;
    sqlx::migrate!("./migrations").run(&mut conn).await?;
    let (inserted, skipped) = import(&mut conn, snapshot).await?;
    println!("Imported {inserted} items; skipped {skipped} already imported items. All imported projects use Parallel as requested. Manual order, recurrence rules and exact timestamps are not supplied by MCP.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn migration_preserves_existing_links_and_failed_import_rolls_back() -> Result<()> {
        let mut db = SqliteConnection::connect_with(
            &SqliteConnectOptions::new()
                .in_memory(true)
                .foreign_keys(true),
        )
        .await?;
        sqlx::raw_sql(include_str!("../../migrations/20261017000000_initial.sql"))
            .execute(&mut db)
            .await?;
        sqlx::raw_sql("INSERT INTO tasks(id,title,created_at,updated_at) VALUES('local','Existing','today','today'); INSERT INTO tags(id,name) VALUES('tag','Existing tag'); INSERT INTO task_tags VALUES('local','tag');").execute(&mut db).await?;
        sqlx::migrate!("./migrations").run(&mut db).await?;
        assert_eq!(
            sqlx::query_scalar::<_, String>("SELECT tag_id FROM task_tags WHERE task_id='local'")
                .fetch_one(&mut db)
                .await?,
            "tag"
        );
        sqlx::raw_sql("CREATE TRIGGER fail_import BEFORE INSERT ON tasks BEGIN SELECT RAISE(ABORT, 'test failure'); END;").execute(&mut db).await?;
        assert!(import(&mut db, snapshot()).await.is_err());
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM projects")
                .fetch_one(&mut db)
                .await?,
            0
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM nirvana_imports")
                .fetch_one(&mut db)
                .await?,
            0
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tags")
                .fetch_one(&mut db)
                .await?,
            1
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM areas")
                .fetch_one(&mut db)
                .await?,
            0
        );
        Ok(())
    }
    fn snapshot() -> Snapshot {
        serde_json::from_value(serde_json::json!({"format":"nirvana-mcp-v1","total":3,"has_more":false,"tags":[{"name":"Personal","type":"area"},{"name":"Home","type":"context"}],"items":[
            {"id":"p","name":"Project","type":"project","state":"active","created":"2026-10-01","tags":["Personal"],"starred":false},
            {"id":"t","name":"Task","type":"task","state":"later","created":"2026-10-01","tags":["Home"],"parent":{"id":"p"},"starred":true,"note":"Keep\nall notes","duedate":"2026-10-31"},
            {"id":"d","name":"Deleted","type":"task","state":"trash","created":"2026-10-01","tags":[],"starred":false}
        ]})).unwrap()
    }
    #[tokio::test]
    async fn import_preserves_data_and_repeats_without_overwriting() -> Result<()> {
        let mut db = SqliteConnection::connect("sqlite::memory:").await?;
        sqlx::migrate!("./migrations").run(&mut db).await?;
        sqlx::query("INSERT INTO tasks(id,title,status,created_at,updated_at) VALUES('local','Existing','inbox','today','today')").execute(&mut db).await?;
        assert_eq!(import(&mut db, snapshot()).await?, (3, 0));
        let row: (String,String,bool,String) = sqlx::query_as("SELECT t.status,t.notes,t.starred,p.name FROM tasks t JOIN projects p ON p.id=t.project_id").fetch_one(&mut db).await?;
        assert_eq!(
            row,
            (
                "later".into(),
                "Keep\nall notes".into(),
                true,
                "Project".into()
            )
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM task_tags")
                .fetch_one(&mut db)
                .await?,
            1
        );
        sqlx::query("UPDATE tasks SET title='Edited locally' WHERE status='later'")
            .execute(&mut db)
            .await?;
        assert_eq!(import(&mut db, snapshot()).await?, (0, 3));
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tasks")
                .fetch_one(&mut db)
                .await?,
            3
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tasks WHERE title='Edited locally'")
                .fetch_one(&mut db)
                .await?,
            1
        );
        let mut changed = snapshot();
        changed.items[0]["name"] = "Changed source".into();
        assert!(import(&mut db, changed).await.is_err());
        let mut invalid = snapshot();
        invalid.items[1]["parent"]["id"] = "missing".into();
        assert!(import(&mut db, invalid).await.is_err());
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM projects")
                .fetch_one(&mut db)
                .await?,
            1
        );
        Ok(())
    }
}
