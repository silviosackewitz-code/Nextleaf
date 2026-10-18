//! Small-dataset sync: immutable request receipts and transactional three-way field merging.
//! shortcut: complete snapshots are bounded to 16 MiB; add change cursors before larger datasets.
use super::*;
use axum::http::HeaderMap;
use sqlx::SqliteConnection;
use std::collections::{BTreeMap, BTreeSet};

type Records = BTreeMap<String, BTreeMap<String, Value>>;
impl Default for Snapshot {
    fn default() -> Self {
        Self {
            areas: Records::new(),
            projects: Records::new(),
            tasks: Records::new(),
            tags: Records::new(),
            protocol: 3,
            clock: schedule::Clock::default(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    areas: Records,
    projects: Records,
    tasks: Records,
    tags: Records,
    protocol: u32,
    clock: schedule::Clock,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SyncConflict {
    pub entity: String,
    pub id: String,
    pub field: String,
    pub local: Value,
    pub server: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Envelope {
    server_id: String,
    snapshot: Snapshot,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Exchange {
    server_id: String,
    client_id: String,
    sequence: i64,
    request_id: String,
    base: Snapshot,
    local: Snapshot,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Reply {
    server_id: String,
    snapshot: Snapshot,
    conflicts: Vec<SyncConflict>,
    tag_remaps: BTreeMap<String, String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct ClientState {
    remote_url: Option<String>,
    server_id: Option<String>,
    client_id: Option<String>,
    sequence: i64,
    baseline: Snapshot,
    inflight: Option<Exchange>,
    conflicts: Vec<SyncConflict>,
    conflict_base: Option<Snapshot>,
    conflict_server: Option<Snapshot>,
    last_sync: Option<String>,
    error: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
pub struct SyncStatus {
    pub initialized: bool,
    pub pending: bool,
    pub conflicts: Vec<SyncConflict>,
    pub error: Option<String>,
    pub last_sync: Option<String>,
    pub remote_url: Option<String>,
}

const AREA: &str = "id,name,sort_order,created_at,updated_at,archived";
const TAG: &str = "id,name,kind,archived";
const PROJECT: &str = "id,area_id,name,notes,project_type,status,sort_order,created_at,updated_at,completed_at,due_date,starred";
const TASK: &str = "id,project_id,area_id,title,notes,status,sort_order,due_date,defer_until,completed_at,created_at,updated_at,starred,waiting_for,estimated_minutes,energy,recurrence";

async fn identity(conn: &mut SqliteConnection) -> Result<String, String> {
    sqlx::query("INSERT OR IGNORE INTO sync_identity(singleton,id) VALUES(1,?)")
        .bind(uid())
        .execute(&mut *conn)
        .await
        .map_err(err)?;
    sqlx::query_scalar("SELECT id FROM sync_identity WHERE singleton=1")
        .fetch_one(conn)
        .await
        .map_err(err)
}
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
async fn snapshot(conn: &mut SqliteConnection) -> Result<Snapshot, String> {
    let mut result = Snapshot {
        clock: schedule::read_clock(conn).await.map_err(err)?,
        ..Snapshot::default()
    };
    for (table, columns, target) in [
        ("tags", TAG, &mut result.tags),
        ("areas", AREA, &mut result.areas),
        ("projects", PROJECT, &mut result.projects),
        ("tasks", TASK, &mut result.tasks),
    ] {
        let fields = columns
            .split(',')
            .map(|col| format!("'{col}',{col}"))
            .collect::<Vec<_>>()
            .join(",");
        let rows: Vec<String> = sqlx::query_scalar(&format!(
            "SELECT json_object({fields}) FROM {table} ORDER BY id"
        ))
        .fetch_all(&mut *conn)
        .await
        .map_err(err)?;
        for row in rows {
            let mut row: BTreeMap<String, Value> = serde_json::from_str(&row).map_err(err)?;
            let id = row["id"].as_str().ok_or("Invalid database ID")?.to_owned();
            if matches!(table, "tasks" | "projects") {
                let (links, owner) = if table == "tasks" {
                    ("task_tags", "task_id")
                } else {
                    ("project_tags", "project_id")
                };
                let names:Vec<String> = sqlx::query_scalar(&format!("SELECT t.id FROM {links} x JOIN tags t ON t.id=x.tag_id WHERE {owner}=? ORDER BY t.id")).bind(&id).fetch_all(&mut *conn).await.map_err(err)?;
                row.insert("tags".into(), json!(names));
            }
            target.insert(id, row);
        }
    }
    Ok(result)
}

fn merge(
    base: &Snapshot,
    local: &Snapshot,
    remote: &Snapshot,
    prefer: Option<&str>,
) -> Result<(Snapshot, Vec<SyncConflict>), String> {
    let mut base = base.clone();
    let mut local = local.clone();
    for (old, new) in tag_remaps(&base, &local, remote) {
        remap_tag(&mut local, &old, &new);
        remap_tag(&mut base, &old, &new);
    }
    let mut merged = remote.clone();
    if local.clock.newer_than(&remote.clock) {
        merged.clock = local.clock.clone();
    }
    let mut conflicts = vec![];
    for (kind, b, l, r, out) in [
        (
            "tag",
            &base.tags,
            &local.tags,
            &remote.tags,
            &mut merged.tags,
        ),
        (
            "area",
            &base.areas,
            &local.areas,
            &remote.areas,
            &mut merged.areas,
        ),
        (
            "project",
            &base.projects,
            &local.projects,
            &remote.projects,
            &mut merged.projects,
        ),
        (
            "task",
            &base.tasks,
            &local.tasks,
            &remote.tasks,
            &mut merged.tasks,
        ),
    ] {
        for id in b.keys() {
            if !l.contains_key(id) || !r.contains_key(id) {
                return Err(
                    "A record was removed outside Nextleaf. Sync stopped without deleting data."
                        .into(),
                );
            }
        }
        for (id, fields) in l {
            let Some(remote_fields) = r.get(id) else {
                out.insert(id.clone(), fields.clone());
                continue;
            };
            let target = out.get_mut(id).unwrap();
            for (field, value) in fields {
                let previous = b.get(id).and_then(|v| v.get(field));
                let other = remote_fields.get(field);
                if field == "completed_at" {
                    continue;
                }
                if field == "updated_at" {
                    if other.is_none_or(|o| value.as_str() > o.as_str()) {
                        target.insert(field.clone(), value.clone());
                    }
                    continue;
                }
                if Some(value) == previous || Some(value) == other {
                    continue;
                }
                if other == previous {
                    target.insert(field.clone(), value.clone());
                    continue;
                }
                if prefer == Some("local") {
                    target.insert(field.clone(), value.clone());
                } else if prefer != Some("server") {
                    conflicts.push(SyncConflict {
                        entity: kind.into(),
                        id: id.clone(),
                        field: field.clone(),
                        local: value.clone(),
                        server: other.cloned().unwrap_or(Value::Null),
                    });
                }
            }
            if matches!(kind, "task" | "project") {
                // Completion time follows its status, never an independently merged field.
                let source = if target["status"] != remote_fields["status"]
                    || (fields["status"] == remote_fields["status"]
                        && b.get(id).and_then(|v| v.get("completed_at"))
                            == remote_fields.get("completed_at"))
                {
                    fields
                } else {
                    remote_fields
                };
                target.insert("completed_at".into(), source["completed_at"].clone());
            }
        }
    }
    Ok((merged, conflicts))
}
// Only independent creations converge. Established IDs retain their identity on rename.
fn tag_remaps(base: &Snapshot, local: &Snapshot, remote: &Snapshot) -> BTreeMap<String, String> {
    local
        .tags
        .iter()
        .filter_map(|(id, fields)| {
            if remote.tags.contains_key(id) || base.tags.contains_key(id) {
                return None;
            }
            remote
                .tags
                .iter()
                .find(|(_, f)| f["name"] == fields["name"])
                .map(|(other, _)| (id.clone(), other.clone()))
        })
        .collect()
}
async fn rekey_tag(conn: &mut SqliteConnection, old: &str, new: &str) -> Result<(), String> {
    if old == new {
        return Ok(());
    }
    let existing: Option<String> = sqlx::query_scalar("SELECT name FROM tags WHERE id=?")
        .bind(new)
        .fetch_optional(&mut *conn)
        .await
        .map_err(err)?;
    if existing.is_some() {
        return Err("Tag identity collision. Sync stopped without changing data.".into());
    }
    // Foreign keys are checked at transaction commit, after both junction tables have followed the ID.
    sqlx::query("PRAGMA defer_foreign_keys=ON")
        .execute(&mut *conn)
        .await
        .map_err(err)?;
    sqlx::query("UPDATE tags SET id=? WHERE id=?")
        .bind(new)
        .bind(old)
        .execute(&mut *conn)
        .await
        .map_err(err)?;
    for table in ["task_tags", "project_tags"] {
        sqlx::query(&format!("UPDATE {table} SET tag_id=? WHERE tag_id=?"))
            .bind(new)
            .bind(old)
            .execute(&mut *conn)
            .await
            .map_err(err)?;
    }
    Ok(())
}
fn remap_tag(snapshot: &mut Snapshot, old: &str, new: &str) {
    if let Some(mut tag) = snapshot.tags.remove(old) {
        tag.insert("id".into(), json!(new));
        snapshot.tags.insert(new.to_owned(), tag);
    }
    for records in [&mut snapshot.tasks, &mut snapshot.projects] {
        for fields in records.values_mut() {
            if let Some(tags) = fields.get_mut("tags").and_then(Value::as_array_mut) {
                for tag in tags.iter_mut() {
                    if tag == old {
                        *tag = json!(new);
                    }
                }
                tags.sort_by(|a, b| a.as_str().cmp(&b.as_str()));
                tags.dedup();
            }
        }
    }
}
fn validate(s: &Snapshot) -> Result<(), String> {
    if s.protocol != 3 {
        return Err(
            "Update both the server and desktop app to support scheduling sync (version 3).".into(),
        );
    }
    s.clock.validate()?;
    let mut names = BTreeSet::new();
    for tag in s.tags.values() {
        if !names.insert(tag.get("name").and_then(Value::as_str)) {
            return Err("Duplicate tag name. Rename one of the tags before syncing.".into());
        }
    }
    if s.areas.len() + s.projects.len() + s.tasks.len() + s.tags.len() > 50_000 {
        return Err("Sync record limit exceeded".into());
    }
    for (table, cols, records) in [
        ("tags", TAG, &s.tags),
        ("areas", AREA, &s.areas),
        ("projects", PROJECT, &s.projects),
        ("tasks", TASK, &s.tasks),
    ] {
        let mut expected: BTreeSet<&str> = cols.split(',').collect();
        if matches!(table, "tasks" | "projects") {
            expected.insert("tags");
        }
        for (id, fields) in records {
            if id.is_empty()
                || id.len() > 128
                || fields.get("id") != Some(&json!(id))
                || fields.keys().map(String::as_str).collect::<BTreeSet<_>>() != expected
            {
                return Err("Invalid sync record fields".into());
            }
            let name = fields[if table == "tasks" { "title" } else { "name" }]
                .as_str()
                .ok_or("Invalid name")?;
            required_name(name).map_err(|_| "Invalid name")?;
            for (key, value) in fields {
                match key.as_str() {
                    "sort_order" => {
                        if value.as_i64().is_none() {
                            return Err("Invalid order".into());
                        }
                    }
                    "starred" | "archived" => {
                        if !matches!(value.as_i64(), Some(0 | 1)) {
                            return Err("Invalid Focus value".into());
                        }
                    }
                    "estimated_minutes" | "energy" => {
                        if !value.is_null()
                            && (value.as_i64().is_none_or(|n| n < 0)
                                || key == "energy" && value.as_i64().is_some_and(|n| n > 3))
                        {
                            return Err("Invalid effort value".into());
                        }
                    }
                    "tags" => {
                        let a = value.as_array().ok_or("Invalid tags")?;
                        for tag in a {
                            if !s.tags.contains_key(tag.as_str().ok_or("Invalid tag")?) {
                                return Err("Unknown tag".into());
                            }
                        }
                    }
                    "area_id" | "project_id" => {
                        if !value.is_null() {
                            let parent = value.as_str().ok_or("Invalid relation")?;
                            if !(if key == "area_id" {
                                s.areas.contains_key(parent)
                            } else {
                                s.projects.contains_key(parent)
                            }) {
                                return Err("Unknown relation".into());
                            }
                        }
                    }
                    "due_date" | "defer_until" => {
                        if !value.is_null() {
                            let date = value.as_str().ok_or("Invalid date")?;
                            if chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_err()
                                && chrono::DateTime::parse_from_rfc3339(date).is_err()
                            {
                                return Err("Invalid date".into());
                            }
                        }
                    }
                    "recurrence" => {
                        if !value.is_null() {
                            let rule: schedule::RepeatRule =
                                serde_json::from_str(value.as_str().ok_or("Invalid repeat rule")?)
                                    .map_err(err)?;
                            rule.validate().map_err(err)?;
                            if fields["status"] != "recurring" {
                                return Err("A repeat rule requires Recurring state".into());
                            }
                        }
                    }
                    "completed_at" | "waiting_for" => {
                        if !value.is_null() && value.as_str().is_none() {
                            return Err("Invalid optional text".into());
                        }
                    }
                    _ => {
                        if value.as_str().is_none_or(|v| v.len() > 1_000_000) {
                            return Err("Invalid text".into());
                        }
                    }
                }
            }
            if table == "tags" && !matches!(fields["kind"].as_str(), Some("tag" | "person")) {
                return Err("Invalid tag kind".into());
            }
            if table == "tasks"
                && !valid_task_status(fields["status"].as_str().unwrap_or(""))
                && fields["status"] != "recurring"
            {
                return Err("Invalid task status".into());
            }
            if table == "projects"
                && (!matches!(
                    fields["status"].as_str(),
                    Some("active" | "someday" | "completed")
                ) || !matches!(
                    fields["project_type"].as_str(),
                    Some("parallel" | "sequential")
                ))
            {
                return Err("Invalid project state".into());
            }
        }
    }
    Ok(())
}
async fn apply(conn: &mut SqliteConnection, s: &Snapshot) -> Result<(), String> {
    validate(s)?;
    schedule::write_clock(conn, &s.clock).await.map_err(err)?;
    // Upsert, never replace/delete: original import audit records and future metadata survive.
    // Avoid transient UNIQUE(name) failures during simultaneous renames. Never delete a tag.
    // Same-name concurrent creations use one canonical ID, preserving local foreign keys.
    for (id, fields) in &s.tags {
        let existing: Option<String> =
            sqlx::query_scalar("SELECT id FROM tags WHERE name=? AND id!=?")
                .bind(fields["name"].as_str().unwrap())
                .bind(id)
                .fetch_optional(&mut *conn)
                .await
                .map_err(err)?;
        if let Some(old) = existing.filter(|old| !s.tags.contains_key(old)) {
            rekey_tag(conn, &old, id).await?;
        }
    }
    for id in s.tags.keys() {
        sqlx::query("UPDATE tags SET name=? WHERE id=?")
            .bind(format!("__sync_{}", uid()))
            .bind(id)
            .execute(&mut *conn)
            .await
            .map_err(err)?;
    }
    for (table, cols, records) in [
        ("tags", TAG, &s.tags),
        ("areas", AREA, &s.areas),
        ("projects", PROJECT, &s.projects),
        ("tasks", TASK, &s.tasks),
    ] {
        let columns: Vec<&str> = cols.split(',').collect();
        let setters = columns
            .iter()
            .filter(|c| **c != "id")
            .map(|c| format!("{c}=excluded.{c}"))
            .collect::<Vec<_>>()
            .join(",");
        let sql = format!(
            "INSERT INTO {table}({cols}) VALUES({}) ON CONFLICT(id) DO UPDATE SET {setters}",
            vec!["?"; columns.len()].join(",")
        );
        for (id, fields) in records {
            let mut q = sqlx::query(&sql);
            for col in &columns {
                q = match &fields[*col] {
                    Value::Null => q.bind(None::<String>),
                    Value::Number(n) => q.bind(n.as_i64().ok_or("Invalid number")?),
                    Value::String(s) => q.bind(s),
                    _ => return Err("Invalid SQL value".into()),
                };
            }
            q.execute(&mut *conn).await.map_err(err)?;
            if matches!(table, "tasks" | "projects") {
                let (links, owner) = if table == "tasks" {
                    ("task_tags", "task_id")
                } else {
                    ("project_tags", "project_id")
                };
                sqlx::query(&format!("DELETE FROM {links} WHERE {owner}=?"))
                    .bind(id)
                    .execute(&mut *conn)
                    .await
                    .map_err(err)?;
                for tag in fields["tags"].as_array().unwrap() {
                    sqlx::query(&format!("INSERT OR IGNORE INTO {links}({owner},tag_id) SELECT ?,id FROM tags WHERE id=?")).bind(id).bind(tag.as_str().unwrap()).execute(&mut *conn).await.map_err(err)?;
                }
            }
        }
    }
    Ok(())
}
fn no_browser(headers: &HeaderMap) -> Result<(), (StatusCode, Json<Value>)> {
    if headers.contains_key(header::ORIGIN) {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error":"Sync is available to native clients only"})),
        ));
    }
    Ok(())
}
fn sync_error(message: String) -> (StatusCode, Json<Value>) {
    (StatusCode::BAD_REQUEST, Json(json!({"error":message})))
}
pub(super) async fn get_snapshot(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> ApiResult<Envelope> {
    no_browser(&headers)?;
    let mut tx = state
        .pool
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(internal)?;
    let id = identity(&mut tx).await.map_err(sync_error)?;
    schedule::advance(&mut tx, Utc::now())
        .await
        .map_err(internal)?;
    let data = snapshot(&mut tx).await.map_err(sync_error)?;
    tx.commit().await.map_err(internal)?;
    Ok(Json(Envelope {
        server_id: id,
        snapshot: data,
    }))
}
pub(super) async fn exchange(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<Exchange>,
) -> ApiResult<Reply> {
    no_browser(&headers)?;
    let result = exchange_db(&state.pool, request)
        .await
        .map_err(sync_error)?;
    Ok(Json(result))
}
async fn exchange_db(pool: &SqlitePool, request: Exchange) -> Result<Reply, String> {
    if Uuid::parse_str(&request.request_id).is_err()
        || Uuid::parse_str(&request.client_id).is_err()
        || request.sequence < 1
    {
        return Err("Invalid request identity".into());
    }
    validate(&request.base)?;
    validate(&request.local)?;
    let request_json = serde_json::to_string(&request).map_err(err)?;
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await.map_err(err)?;
    let server_id = identity(&mut tx).await?;
    if request.server_id != server_id {
        return Err("Server identity changed. Sync stopped to protect both databases.".into());
    }
    if let Some((previous, response)) = sqlx::query_as::<_, (String, String)>(
        "SELECT request_json,response_json FROM sync_receipts WHERE id=?",
    )
    .bind(&request.request_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(err)?
    {
        if previous != request_json {
            return Err("Request identity was reused with different data".into());
        }
        return serde_json::from_str(&response).map_err(err);
    }
    let latest: Option<i64> =
        sqlx::query_scalar("SELECT sequence FROM sync_receipts WHERE client_id=?")
            .bind(&request.client_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(err)?;
    if latest.is_some_and(|n| request.sequence <= n) {
        return Err("Stale sync request rejected".into());
    }
    let remote = snapshot(&mut tx).await?;
    let (merged, conflicts) = merge(&request.base, &request.local, &remote, None)?;
    let tag_remaps = tag_remaps(&request.base, &request.local, &remote);
    let reply = if conflicts.is_empty() {
        apply(&mut tx, &merged).await?;
        schedule::advance(&mut tx, Utc::now()).await.map_err(err)?;
        Reply {
            server_id,
            snapshot: snapshot(&mut tx).await?,
            conflicts,
            tag_remaps,
        }
    } else {
        Reply {
            server_id,
            snapshot: remote,
            conflicts,
            tag_remaps,
        }
    };
    sqlx::query("INSERT INTO sync_receipts(id,client_id,sequence,request_json,response_json) VALUES(?,?,?,?,?) ON CONFLICT(client_id) DO UPDATE SET id=excluded.id,sequence=excluded.sequence,request_json=excluded.request_json,response_json=excluded.response_json").bind(&request.request_id).bind(&request.client_id).bind(request.sequence).bind(request_json).bind(serde_json::to_string(&reply).map_err(err)?).execute(&mut *tx).await.map_err(err)?;
    tx.commit().await.map_err(err)?;
    Ok(reply)
}

#[derive(Clone)]
pub struct SyncEngine {
    pool: SqlitePool,
    http: reqwest::Client,
}
impl SyncEngine {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .expect("TLS client configuration"),
        }
    }
    pub async fn status(&self) -> Result<SyncStatus, String> {
        let mut tx = self.pool.begin().await.map_err(err)?;
        let state = load(&mut tx).await?;
        let current = snapshot(&mut tx).await?;
        Ok(SyncStatus {
            initialized: state.last_sync.is_some(),
            pending: state.inflight.is_some() || !equivalent(&current, &state.baseline),
            conflicts: state.conflicts,
            error: state.error,
            last_sync: state.last_sync,
            remote_url: state.remote_url,
        })
    }
    pub async fn sync_once(&self, remote: &str) -> Result<SyncStatus, String> {
        if let Err(error) = self.sync_inner(remote).await {
            let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await.map_err(err)?;
            let mut state = load(&mut tx).await?;
            state.error = Some(error.clone());
            save(&mut tx, &state).await?;
            tx.commit().await.map_err(err)?;
            return Err(error);
        }
        self.status().await
    }
    async fn sync_inner(&self, remote: &str) -> Result<(), String> {
        let url = reqwest::Url::parse(remote.trim()).map_err(|_| "Invalid server URL")?;
        if !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || url.path() != "/"
        {
            return Err("Use a server origin without credentials, path or query".into());
        }
        if url.scheme() != "https"
            && !(url.scheme() == "http"
                && matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "[::1]")))
        {
            return Err("Sync requires HTTPS, except for loopback development".into());
        }
        let remote = url.as_str().trim_end_matches('/').to_owned();
        {
            let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await.map_err(err)?;
            let mut state = load(&mut tx).await?;
            if state.server_id.is_some() && state.remote_url.as_ref().is_some_and(|v| v != &remote)
            {
                return Err("This database is linked to a different server URL. Keep that connection to protect local data.".into());
            }
            state.remote_url = Some(remote.clone());
            state.error = None;
            save(&mut tx, &state).await?;
            tx.commit().await.map_err(err)?;
        }
        let mut tx = self.pool.begin().await.map_err(err)?;
        let state = load(&mut tx).await?;
        tx.commit().await.map_err(err)?;
        if !state.conflicts.is_empty() {
            return Err("Resolve the sync conflicts before retrying".into());
        }
        if state.server_id.is_none() {
            let envelope: Envelope = self
                .response(
                    self.http
                        .get(format!("{remote}/api/sync"))
                        .send()
                        .await
                        .map_err(|_| "Server is unreachable. Local changes are saved.")?,
                )
                .await?;
            validate(&envelope.snapshot)?;
            let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await.map_err(err)?;
            let mut state = load(&mut tx).await?;
            if state.remote_url.as_deref() != Some(&remote) {
                return Err(
                    "Connection changed while connecting. Retry using the current address.".into(),
                );
            }
            if state.server_id.is_none() {
                state.server_id = Some(envelope.server_id);
                save(&mut tx, &state).await?;
            }
            tx.commit().await.map_err(err)?;
        }
        let request = {
            let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await.map_err(err)?;
            let mut state = load(&mut tx).await?;
            if state.remote_url.as_deref() != Some(&remote) {
                return Err(
                    "Connection changed while connecting. Retry using the current address.".into(),
                );
            }
            if state.inflight.is_none() {
                if state.client_id.is_none() {
                    state.client_id = Some(uid());
                }
                state.sequence += 1;
                state.inflight = Some(Exchange {
                    client_id: state.client_id.clone().unwrap(),
                    sequence: state.sequence,
                    server_id: state.server_id.clone().ok_or("No server identity")?,
                    request_id: uid(),
                    base: state.baseline.clone(),
                    local: snapshot(&mut tx).await?,
                });
                save(&mut tx, &state).await?;
            }
            let request = state.inflight.unwrap();
            tx.commit().await.map_err(err)?;
            request
        };
        let reply: Reply = self
            .response(
                self.http
                    .post(format!("{remote}/api/sync"))
                    .json(&request)
                    .send()
                    .await
                    .map_err(|_| "Server is unreachable. Local changes are saved.")?,
            )
            .await?;
        self.finish(&request, reply).await
    }
    async fn response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, String> {
        if !response.status().is_success() {
            return Err(format!(
                "Server sync failed (HTTP {}). Check that the server supports offline sync.",
                response.status().as_u16()
            ));
        }
        if response
            .content_length()
            .is_some_and(|n| n > 16 * 1024 * 1024)
        {
            return Err("Sync response exceeds size limit".into());
        }
        let mut response = response;
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(err)? {
            if bytes.len() + chunk.len() > 16 * 1024 * 1024 {
                return Err("Sync response exceeds size limit".into());
            }
            bytes.extend_from_slice(&chunk);
        }
        serde_json::from_slice(&bytes).map_err(|_| "Invalid sync response".into())
    }
    async fn finish(&self, request: &Exchange, reply: Reply) -> Result<(), String> {
        if reply.server_id != request.server_id {
            return Err("Server identity changed. Sync stopped.".into());
        }
        validate(&reply.snapshot)?;
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await.map_err(err)?;
        let mut state = load(&mut tx).await?;
        if state.inflight.as_ref().map(|r| &r.request_id) != Some(&request.request_id) {
            return Ok(());
        }
        let mut current = snapshot(&mut tx).await?;
        let mut base = if reply.conflicts.is_empty() {
            request.local.clone()
        } else {
            request.base.clone()
        };
        for (old, new) in &reply.tag_remaps {
            if !request.local.tags.contains_key(old)
                || request.base.tags.contains_key(old)
                || !reply.snapshot.tags.contains_key(new)
                || reply.snapshot.tags.contains_key(old)
            {
                return Err("Invalid tag identity mapping".into());
            }
            rekey_tag(&mut tx, old, new).await?;
            remap_tag(&mut current, old, new);
            remap_tag(&mut base, old, new);
        }
        let (merged, conflicts) = merge(&base, &current, &reply.snapshot, None)?;
        if conflicts.is_empty() && reply.conflicts.is_empty() {
            apply(&mut tx, &merged).await?;
            state.baseline = reply.snapshot;
            state.last_sync = Some(stamp());
            state.error = None;
        } else {
            state.conflicts = if conflicts.is_empty() {
                reply.conflicts
            } else {
                conflicts
            };
            state.conflict_base = Some(base.clone());
            state.conflict_server = Some(reply.snapshot);
            state.error = Some("Changes conflict. Your local data has been kept.".into());
        }
        state.inflight = None;
        save(&mut tx, &state).await?;
        tx.commit().await.map_err(err)?;
        Ok(())
    }
    /// Explicitly choose the winning value for conflicting fields only. Other changes merge.
    pub async fn resolve_conflicts(&self, choice: &str) -> Result<SyncStatus, String> {
        if !matches!(choice, "local" | "server") {
            return Err("Choose local or server".into());
        }
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await.map_err(err)?;
        let mut state = load(&mut tx).await?;
        let base = state
            .conflict_base
            .as_ref()
            .ok_or("No conflicts to resolve")?;
        let remote = state
            .conflict_server
            .as_ref()
            .ok_or("No conflict server snapshot")?;
        let local = snapshot(&mut tx).await?;
        let (_, latest_conflicts) = merge(base, &local, remote, None)?;
        if !latest_conflicts.is_empty() && latest_conflicts != state.conflicts {
            state.conflicts = latest_conflicts;
            state.error = Some(
                "Conflict values changed. Review the updated conflicts before choosing again."
                    .into(),
            );
            save(&mut tx, &state).await?;
            tx.commit().await.map_err(err)?;
            return self.status().await;
        }
        let (merged, _) = merge(base, &local, remote, Some(choice))?;
        apply(&mut tx, &merged).await?;
        state.baseline = remote.clone();
        state.conflicts.clear();
        state.conflict_base = None;
        state.conflict_server = None;
        state.error = None;
        save(&mut tx, &state).await?;
        tx.commit().await.map_err(err)?;
        self.status().await
    }
}
/// Upgrade only persisted v1 state. Old peers cannot write v1 snapshots into a v2 database.
pub(crate) async fn migrate_client_state(pool: &SqlitePool) -> Result<(), String> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await.map_err(err)?;
    let raw: Option<String> =
        sqlx::query_scalar("SELECT state_json FROM sync_client WHERE singleton=1")
            .fetch_optional(&mut *tx)
            .await
            .map_err(err)?;
    let Some(raw) = raw else {
        return Ok(());
    };
    let mut state: Value = serde_json::from_str(&raw).map_err(err)?;
    if state["baseline"]["protocol"] == 3 {
        return Ok(());
    }
    if state["baseline"]["protocol"] == 2 {
        for key in ["baseline", "conflict_base", "conflict_server"] {
            if !state[key].is_null() {
                upgrade_schedule_snapshot(&mut state[key])?;
            }
        }
        state["inflight"] = Value::Null;
        let state: ClientState = serde_json::from_value(state).map_err(err)?;
        save(&mut tx, &state).await?;
        tx.commit().await.map_err(err)?;
        return Ok(());
    }
    if !state["baseline"]["protocol"].is_null() {
        return Err("Unsupported saved sync protocol".into());
    }
    let mut canonical = BTreeMap::<String, String>::new();
    // The last acknowledged server baseline is authoritative for IDs; v1 used only names in links.
    for source in [&state["baseline"], &state["conflict_server"]] {
        if let Some(tags) = source["tags"].as_object() {
            for (name, id) in tags {
                canonical
                    .entry(name.clone())
                    .or_insert(id.as_str().ok_or("Invalid legacy tag ID")?.into());
            }
        }
    }
    let local: Vec<(String, String)> = sqlx::query_as("SELECT name,id FROM tags")
        .fetch_all(&mut *tx)
        .await
        .map_err(err)?;
    for (name, id) in &local {
        canonical.entry(name.clone()).or_insert(id.clone());
    }
    for (name, id) in local {
        if let Some(new) = canonical.get(&name) {
            rekey_tag(&mut tx, &id, new).await?;
        }
    }
    for key in ["baseline", "conflict_base", "conflict_server"] {
        if !state[key].is_null() {
            upgrade_snapshot(&mut state[key], &canonical)?;
        }
    }
    // A v1 request may already be committed remotely. Keep its original baseline and all current
    // local edits, then send a fresh higher sequence: three-way merging detects any divergence.
    state["inflight"] = Value::Null;
    // Recompute conflicts using ID relations after upgrading; never trust name-based field values.
    if !state["conflict_base"].is_null() && !state["conflict_server"].is_null() {
        let base: Snapshot = serde_json::from_value(state["conflict_base"].clone()).map_err(err)?;
        let remote: Snapshot =
            serde_json::from_value(state["conflict_server"].clone()).map_err(err)?;
        let (_, conflicts) = merge(&base, &snapshot(&mut tx).await?, &remote, None)?;
        state["conflicts"] = serde_json::to_value(conflicts).map_err(err)?;
    }
    let state: ClientState = serde_json::from_value(state).map_err(err)?;
    save(&mut tx, &state).await?;
    tx.commit().await.map_err(err)?;
    Ok(())
}
fn upgrade_snapshot(value: &mut Value, canonical: &BTreeMap<String, String>) -> Result<(), String> {
    let old = value["tags"]
        .as_object()
        .ok_or("Invalid legacy tags")?
        .clone();
    let mut ids = BTreeMap::new();
    let mut tags = serde_json::Map::new();
    for (name, id) in old {
        let id = canonical
            .get(&name)
            .map(String::as_str)
            .or_else(|| id.as_str())
            .ok_or("Invalid legacy tag ID")?
            .to_owned();
        tags.insert(
            id.clone(),
            json!({"id":id,"name":name,"kind":"tag","archived":0}),
        );
        ids.insert(name, id);
    }
    for table in ["tasks", "projects"] {
        for fields in value[table]
            .as_object_mut()
            .ok_or("Invalid legacy records")?
            .values_mut()
        {
            let mut links = fields["tags"]
                .as_array()
                .ok_or("Invalid legacy links")?
                .iter()
                .map(|name| {
                    ids.get(name.as_str().ok_or("Invalid legacy tag name")?)
                        .cloned()
                        .ok_or("Unknown legacy tag")
                })
                .collect::<Result<Vec<_>, _>>()?;
            links.sort();
            links.dedup();
            fields["tags"] = json!(links);
        }
    }
    for fields in value["areas"]
        .as_object_mut()
        .ok_or("Invalid legacy areas")?
        .values_mut()
    {
        fields["archived"] = json!(0);
    }
    value["tags"] = Value::Object(tags);
    upgrade_schedule_snapshot(value)?;
    validate(&serde_json::from_value(value.clone()).map_err(err)?)
}
fn upgrade_schedule_snapshot(value: &mut Value) -> Result<(), String> {
    for task in value["tasks"]
        .as_object_mut()
        .ok_or("Invalid saved tasks")?
        .values_mut()
    {
        task["recurrence"] = Value::Null;
    }
    value["clock"] = serde_json::to_value(schedule::Clock::default()).map_err(err)?;
    value["protocol"] = json!(3);
    validate(&serde_json::from_value(value.clone()).map_err(err)?)
}
async fn load(conn: &mut SqliteConnection) -> Result<ClientState, String> {
    let json: Option<String> =
        sqlx::query_scalar("SELECT state_json FROM sync_client WHERE singleton=1")
            .fetch_optional(conn)
            .await
            .map_err(err)?;
    json.map(|j| serde_json::from_str(&j).map_err(err))
        .unwrap_or_else(|| Ok(ClientState::default()))
}
async fn save(conn: &mut SqliteConnection, state: &ClientState) -> Result<(), String> {
    sqlx::query("INSERT INTO sync_client(singleton,state_json) VALUES(1,?) ON CONFLICT(singleton) DO UPDATE SET state_json=excluded.state_json").bind(serde_json::to_string(state).map_err(err)?).execute(conn).await.map_err(err)?;
    Ok(())
}

fn equivalent(left: &Snapshot, right: &Snapshot) -> bool {
    left.areas == right.areas
        && left.projects == right.projects
        && left.tasks == right.tasks
        && left.tags == right.tags
        && left.clock == right.clock
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::IntoFuture;
    async fn pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .in_memory(true)
                    .foreign_keys(true),
            )
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }
    async fn task(pool: &SqlitePool, title: &str) -> String {
        create_task(
            State(AppState { pool: pool.clone() }),
            Json(serde_json::from_value(json!({"title":title,"status":"next"})).unwrap()),
        )
        .await
        .unwrap()
        .0
        .id
    }
    async fn edit(pool: &SqlitePool, id: &str, changes: Value) {
        let _ = update_task(
            State(AppState { pool: pool.clone() }),
            Path(id.into()),
            Json(serde_json::from_value(changes).unwrap()),
        )
        .await
        .unwrap();
    }
    async fn data(pool: &SqlitePool) -> Snapshot {
        let mut conn = pool.acquire().await.unwrap();
        snapshot(&mut conn).await.unwrap()
    }
    async fn roundtrip(engine: &SyncEngine, server: &SqlitePool) {
        let server_id = {
            let mut conn = server.acquire().await.unwrap();
            identity(&mut conn).await.unwrap()
        };
        let request = {
            let mut tx = engine.pool.begin_with("BEGIN IMMEDIATE").await.unwrap();
            let mut state = load(&mut tx).await.unwrap();
            if state.server_id.is_none() {
                state.server_id = Some(server_id.clone());
            }
            state.sequence += 1;
            if state.client_id.is_none() {
                state.client_id = Some(uid());
            }
            let request = Exchange {
                server_id: state.server_id.clone().unwrap(),
                client_id: state.client_id.clone().unwrap(),
                sequence: state.sequence,
                request_id: uid(),
                base: state.baseline.clone(),
                local: snapshot(&mut tx).await.unwrap(),
            };
            state.inflight = Some(request.clone());
            save(&mut tx, &state).await.unwrap();
            tx.commit().await.unwrap();
            request
        };
        let reply = exchange_db(server, request.clone()).await.unwrap();
        engine.finish(&request, reply).await.unwrap();
    }
    #[tokio::test]
    async fn initial_sync_disjoint_changes_tags_and_trash_restore() {
        let server = pool().await;
        let a = SyncEngine::new(pool().await);
        let b = SyncEngine::new(pool().await);
        let id = task(&server, "Server task").await;
        let offline = task(&a.pool, "Created offline").await;
        roundtrip(&a, &server).await;
        roundtrip(&b, &server).await;
        assert_eq!(data(&a.pool).await.tasks.len(), 2);
        assert!(data(&server).await.tasks.contains_key(&offline));
        edit(&a.pool, &id, json!({"title":"From A","tags":["Laptop"]})).await;
        edit(&b.pool, &id, json!({"notes":"From B"})).await;
        let _ = create_tag(
            State(AppState {
                pool: b.pool.clone(),
            }),
            Json(NewTag {
                name: "Laptop".into(),
                kind: None,
            }),
        )
        .await
        .unwrap();
        roundtrip(&a, &server).await;
        roundtrip(&b, &server).await;
        roundtrip(&a, &server).await;
        let d = data(&a.pool).await;
        assert_eq!(d.tasks[&id]["title"], "From A");
        assert_eq!(d.tasks[&id]["notes"], "From B");
        assert!(!a.status().await.unwrap().pending);
        assert!(!b.status().await.unwrap().pending);
        edit(&a.pool, &id, json!({"status":"trash"})).await;
        roundtrip(&a, &server).await;
        roundtrip(&b, &server).await;
        assert_eq!(data(&b.pool).await.tasks[&id]["status"], "trash");
        edit(&b.pool, &id, json!({"status":"next"})).await;
        roundtrip(&b, &server).await;
        roundtrip(&a, &server).await;
        assert_eq!(data(&a.pool).await.tasks[&id]["status"], "next");
    }
    #[tokio::test]
    async fn conflict_preserves_local_then_explicit_choice_merges_other_fields() {
        let server = pool().await;
        let a = SyncEngine::new(pool().await);
        let b = SyncEngine::new(pool().await);
        let id = task(&server, "Original").await;
        roundtrip(&a, &server).await;
        roundtrip(&b, &server).await;
        edit(&a.pool, &id, json!({"title":"A","notes":"Keep notes"})).await;
        edit(&b.pool, &id, json!({"title":"B","energy":2})).await;
        roundtrip(&a, &server).await;
        roundtrip(&b, &server).await;
        assert_eq!(b.status().await.unwrap().conflicts.len(), 1);
        assert_eq!(data(&b.pool).await.tasks[&id]["title"], "B");
        assert_eq!(data(&server).await.tasks[&id]["title"], "A");
        b.resolve_conflicts("local").await.unwrap();
        roundtrip(&b, &server).await;
        roundtrip(&a, &server).await;
        let d = data(&a.pool).await;
        assert_eq!(d.tasks[&id]["title"], "B");
        assert_eq!(d.tasks[&id]["notes"], "Keep notes");
        assert_eq!(d.tasks[&id]["energy"], 2);
        edit(&a.pool, &id, json!({"title":"A again"})).await;
        edit(&b.pool, &id, json!({"title":"B again"})).await;
        roundtrip(&a, &server).await;
        roundtrip(&b, &server).await;
        b.resolve_conflicts("server").await.unwrap();
        assert_eq!(data(&b.pool).await.tasks[&id]["title"], "A again");
    }
    #[tokio::test]
    async fn restart_retry_concurrent_local_edit_and_identity_rejection() {
        let dir = std::env::temp_dir().join(format!("nextleaf-sync-test-{}", uid()));
        let path = dir.join("local.db");
        let local = create_pool(&path).await.unwrap();
        let server = pool().await;
        let engine = SyncEngine::new(local.clone());
        let id = task(&local, "Offline persistent").await;
        roundtrip(&engine, &server).await;
        edit(&local, &id, json!({"notes":"Offline after sync"})).await;
        drop(engine);
        local.close().await;
        let local = create_pool(&path).await.unwrap();
        let engine = SyncEngine::new(local.clone());
        assert!(engine.status().await.unwrap().pending);
        let request = {
            let mut tx = local.begin_with("BEGIN IMMEDIATE").await.unwrap();
            let mut state = load(&mut tx).await.unwrap();
            state.sequence += 1;
            let r = Exchange {
                server_id: state.server_id.clone().unwrap(),
                client_id: state.client_id.clone().unwrap(),
                sequence: state.sequence,
                request_id: uid(),
                base: state.baseline.clone(),
                local: snapshot(&mut tx).await.unwrap(),
            };
            state.inflight = Some(r.clone());
            save(&mut tx, &state).await.unwrap();
            tx.commit().await.unwrap();
            r
        };
        let reply = exchange_db(&server, request.clone()).await.unwrap();
        edit(&local, &id, json!({"energy":3})).await; // MCP writes while network request is in flight.
        let retry = exchange_db(&server, request.clone()).await.unwrap();
        assert_eq!(reply.snapshot, retry.snapshot);
        engine.finish(&request, retry).await.unwrap();
        assert_eq!(data(&local).await.tasks[&id]["energy"], 3);
        assert!(engine.status().await.unwrap().pending);
        roundtrip(&engine, &server).await;
        assert_eq!(data(&server).await.tasks[&id]["energy"], 3);
        let other = pool().await;
        assert!(exchange_db(&other, request)
            .await
            .unwrap_err()
            .contains("identity changed"));
        local.close().await;
        std::fs::remove_dir_all(dir).unwrap();
    }
    #[tokio::test]
    async fn invalid_snapshot_is_atomic_and_browser_origin_rejected() {
        let server = pool().await;
        let id = task(&server, "Original").await;
        let base = data(&server).await;
        let mut invalid = base.clone();
        invalid
            .tasks
            .get_mut(&id)
            .unwrap()
            .insert("energy".into(), json!(99));
        let server_id = {
            let mut c = server.acquire().await.unwrap();
            identity(&mut c).await.unwrap()
        };
        let r = Exchange {
            server_id,
            client_id: uid(),
            sequence: 1,
            request_id: uid(),
            base: base.clone(),
            local: invalid,
        };
        assert!(exchange_db(&server, r).await.is_err());
        assert_eq!(data(&server).await, base);
        let mut headers = HeaderMap::new();
        headers.insert(
            header::ORIGIN,
            HeaderValue::from_static("https://evil.example"),
        );
        assert!(no_browser(&headers).is_err());
    }
    #[tokio::test]
    #[ignore = "requires localhost networking"]
    async fn real_http_sync_and_unreachable_server_preserve_local_data() {
        let server = pool().await;
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let handle = tokio::spawn(axum::serve(listener, api_router(server.clone())).into_future());
        let engine = SyncEngine::new(pool().await);
        let offline = task(&engine.pool, "Before first connection").await;
        let closed = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let wrong = format!("http://{}", closed.local_addr().unwrap());
        drop(closed);
        assert!(engine.sync_once(&wrong).await.is_err());
        assert_eq!(engine.status().await.unwrap().remote_url, Some(wrong));
        let id = task(&server, "From server").await;
        assert!(engine.sync_once(&url).await.unwrap().initialized);
        assert!(data(&server).await.tasks.contains_key(&offline));
        edit(&engine.pool, &id, json!({"title":"Offline later"})).await;
        handle.abort();
        let _ = handle.await;
        assert!(engine.sync_once(&url).await.is_err());
        let status = engine.status().await.unwrap();
        assert!(status.pending);
        assert!(status.error.is_some());
        assert_eq!(status.remote_url, Some(url));
        assert_eq!(
            data(&engine.pool).await.tasks[&id]["title"],
            "Offline later"
        );
    }
    #[tokio::test]
    async fn offline_area_project_relations_and_completion_stay_consistent() {
        let server = pool().await;
        let engine = SyncEngine::new(pool().await);
        let state = AppState {
            pool: engine.pool.clone(),
        };
        let area = create_area(
            State(state.clone()),
            Json(NewArea {
                name: "Work".into(),
            }),
        )
        .await
        .unwrap()
        .0;
        let project = create_project(
            State(state.clone()),
            Json(NewProject {
                name: "Project".into(),
                area_id: Some(area.id.clone()),
                project_type: Some("sequential".into()),
                notes: None,
            }),
        )
        .await
        .unwrap()
        .0;
        let first = create_task(
            State(state.clone()),
            Json(
                serde_json::from_value(
                    json!({"title":"First","project_id":project.id,"status":"next"}),
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap()
        .0;
        let second = create_task(
            State(state.clone()),
            Json(
                serde_json::from_value(
                    json!({"title":"Second","project_id":project.id,"status":"next"}),
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap()
        .0;
        let _ = update_project(
            State(state.clone()),
            Path(project.id.clone()),
            Json(
                serde_json::from_value(
                    json!({"tags":["Office"],"due_date":"2026-12-01","starred":true}),
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap();
        roundtrip(&engine, &server).await;
        let remote = AppState {
            pool: server.clone(),
        };
        let tasks = fetch_tasks(&server).await.unwrap();
        assert!(tasks.iter().find(|t| t.id == first.id).unwrap().available);
        assert!(
            tasks
                .iter()
                .find(|t| t.id == second.id)
                .unwrap()
                .blocked_by_sequence
        );
        let projects = list_projects(
            State(remote.clone()),
            Query(ProjectFilter { area_id: None }),
        )
        .await
        .unwrap()
        .0;
        assert_eq!(projects[0].area_id, Some(area.id));
        assert_eq!(projects[0].tags, vec!["Office"]);
        edit(&engine.pool, &first.id, json!({"status":"completed"})).await;
        let _ = update_project(
            State(remote),
            Path(project.id),
            Json(serde_json::from_value(json!({"notes":"Remote notes"})).unwrap()),
        )
        .await
        .unwrap();
        roundtrip(&engine, &server).await;
        let first_row = &data(&engine.pool).await.tasks[&first.id];
        assert_eq!(first_row["status"], "completed");
        assert!(!first_row["completed_at"].is_null());
        assert!(
            fetch_tasks(&server)
                .await
                .unwrap()
                .iter()
                .find(|t| t.id == second.id)
                .unwrap()
                .available
        );
        edit(&engine.pool, &first.id, json!({"status":"next"})).await;
        roundtrip(&engine, &server).await;
        assert!(data(&server).await.tasks[&first.id]["completed_at"].is_null());
    }
    #[tokio::test]
    async fn conflict_choice_never_overwrites_new_unseen_conflicts() {
        let server = pool().await;
        let engine = SyncEngine::new(pool().await);
        let a = task(&server, "A").await;
        let b = task(&server, "B").await;
        roundtrip(&engine, &server).await;
        edit(&server, &a, json!({"title":"Server A"})).await;
        edit(&server, &b, json!({"notes":"Server B notes"})).await;
        edit(&engine.pool, &a, json!({"title":"Local A"})).await;
        roundtrip(&engine, &server).await;
        assert_eq!(engine.status().await.unwrap().conflicts.len(), 1);
        edit(&engine.pool, &b, json!({"notes":"New local B notes"})).await;
        let status = engine.resolve_conflicts("server").await.unwrap();
        assert_eq!(status.conflicts.len(), 2);
        assert!(status.error.unwrap().contains("Review"));
        let local = data(&engine.pool).await;
        assert_eq!(local.tasks[&a]["title"], "Local A");
        assert_eq!(local.tasks[&b]["notes"], "New local B notes");
        engine.resolve_conflicts("server").await.unwrap();
        assert_eq!(
            data(&engine.pool).await.tasks[&b]["notes"],
            "Server B notes"
        );
    }
    #[tokio::test]
    async fn stale_editor_preconditions_protect_task_and_project_fields() {
        let pool = pool().await;
        let state = AppState { pool: pool.clone() };
        let id = task(&pool, "Original").await;
        edit(
            &pool,
            &id,
            json!({"title":"External update","notes":"Keep"}),
        )
        .await;
        let error = update_task(
            State(state.clone()),
            Path(id.clone()),
            Json(
                serde_json::from_value(
                    json!({"title":"Stale draft","expected":{"title":"Original"}}),
                )
                .unwrap(),
            ),
        )
        .await
        .unwrap_err();
        assert_eq!(error.0, StatusCode::CONFLICT);
        assert_eq!(data(&pool).await.tasks[&id]["title"], "External update");
        let _=update_task(State(state.clone()),Path(id),Json(serde_json::from_value(json!({"notes":"Separate edit","starred":true,"expected":{"notes":"Keep","starred":false,"tags":[]}})).unwrap())).await.unwrap();
        let project = create_project(
            State(state.clone()),
            Json(NewProject {
                name: "Project".into(),
                area_id: None,
                project_type: None,
                notes: None,
            }),
        )
        .await
        .unwrap()
        .0;
        let _ = update_project(
            State(state.clone()),
            Path(project.id.clone()),
            Json(serde_json::from_value(json!({"name":"Server rename"})).unwrap()),
        )
        .await
        .unwrap();
        let error = update_project(
            State(state.clone()),
            Path(project.id.clone()),
            Json(
                serde_json::from_value(json!({"name":"Lost update","expected":{"name":"Project"}}))
                    .unwrap(),
            ),
        )
        .await
        .unwrap_err();
        assert_eq!(error.0, StatusCode::CONFLICT);
        let error = update_project(
            State(state),
            Path(project.id),
            Json(
                serde_json::from_value(json!({"name":"Invalid guard","expected":{"unexpected":0}}))
                    .unwrap(),
            ),
        )
        .await
        .unwrap_err();
        assert_eq!(error.0, StatusCode::BAD_REQUEST);
    }
    async fn tag_edit(pool: &SqlitePool, id: &str, changes: Value) -> Tag {
        update_tag(
            State(AppState { pool: pool.clone() }),
            Path(id.into()),
            Json(serde_json::from_value(changes).unwrap()),
        )
        .await
        .unwrap()
        .0
    }
    async fn make_tag(pool: &SqlitePool, name: &str, kind: &str) -> Tag {
        create_tag(
            State(AppState { pool: pool.clone() }),
            Json(NewTag {
                name: name.into(),
                kind: Some(kind.into()),
            }),
        )
        .await
        .unwrap()
        .0
    }
    fn legacy(s: &Snapshot) -> Value {
        let mut v = serde_json::to_value(s).unwrap();
        v.as_object_mut().unwrap().remove("protocol");
        for row in v["areas"].as_object_mut().unwrap().values_mut() {
            row.as_object_mut().unwrap().remove("archived");
        }
        for table in ["tasks", "projects"] {
            for row in v[table].as_object_mut().unwrap().values_mut() {
                row["tags"] = json!(row["tags"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|id| s.tags[id.as_str().unwrap()]["name"].as_str().unwrap())
                    .collect::<Vec<_>>());
            }
        }
        v["tags"] = json!(s
            .tags
            .iter()
            .map(|(id, t)| (t["name"].as_str().unwrap(), id))
            .collect::<BTreeMap<_, _>>());
        v
    }
    #[tokio::test]
    async fn tag_rename_type_and_archive_preserve_ids_and_offline_assignments() {
        let server = pool().await;
        let a = SyncEngine::new(pool().await);
        let b = SyncEngine::new(pool().await);
        let person = make_tag(&server, "Alice", "person").await;
        let task_id = task(&server, "Waiting").await;
        edit(
            &server,
            &task_id,
            json!({"status":"waiting","waiting_for":"Alice","tags":["Alice"]}),
        )
        .await;
        roundtrip(&a, &server).await;
        roundtrip(&b, &server).await;
        tag_edit(
            &a.pool,
            &person.id,
            json!({"name":"Alice Smith","archived":true}),
        )
        .await;
        let second = task(&b.pool, "New offline task").await;
        edit(&b.pool, &second, json!({"tags":["Alice"]})).await;
        edit(&b.pool, &task_id, json!({"notes":"Offline notes"})).await;
        roundtrip(&a, &server).await;
        roundtrip(&b, &server).await;
        roundtrip(&a, &server).await;
        let d = data(&a.pool).await;
        assert_eq!(d.tags.len(), 1);
        assert_eq!(d.tags[&person.id]["name"], "Alice Smith");
        assert_eq!(d.tags[&person.id]["archived"], 1);
        assert_eq!(d.tasks[&second]["tags"], json!([person.id]));
        assert_eq!(d.tasks[&task_id]["waiting_for"], "Alice Smith");
        assert_eq!(d.tasks[&task_id]["notes"], "Offline notes");
        tag_edit(&b.pool, &person.id, json!({"kind":"tag","archived":false})).await;
        roundtrip(&b, &server).await;
        roundtrip(&a, &server).await;
        assert_eq!(data(&a.pool).await.tags[&person.id]["kind"], "tag");
        assert!(!a.status().await.unwrap().pending);
    }
    #[tokio::test]
    async fn canonical_tag_remap_preserves_rename_during_inflight_request() {
        let server = pool().await;
        let a = SyncEngine::new(pool().await);
        let remote = make_tag(&server, "Laptop", "tag").await;
        let local = make_tag(&a.pool, "Laptop", "tag").await;
        let id = task(&a.pool, "Local").await;
        edit(&a.pool, &id, json!({"tags":["Laptop"]})).await;
        let request = {
            let mut tx = a.pool.begin_with("BEGIN IMMEDIATE").await.unwrap();
            let mut s = load(&mut tx).await.unwrap();
            let mut remote_conn = server.acquire().await.unwrap();
            let server_id = identity(&mut remote_conn).await.unwrap();
            let r = Exchange {
                server_id: server_id.clone(),
                client_id: uid(),
                sequence: 1,
                request_id: uid(),
                base: s.baseline.clone(),
                local: snapshot(&mut tx).await.unwrap(),
            };
            s.server_id = Some(server_id);
            s.client_id = Some(r.client_id.clone());
            s.sequence = 1;
            s.inflight = Some(r.clone());
            save(&mut tx, &s).await.unwrap();
            tx.commit().await.unwrap();
            r
        };
        let response = exchange_db(&server, request.clone()).await.unwrap();
        assert_eq!(response.tag_remaps.get(&local.id), Some(&remote.id));
        tag_edit(&a.pool, &local.id, json!({"name":"Notebook"})).await;
        a.finish(&request, response).await.unwrap();
        let d = data(&a.pool).await;
        assert_eq!(d.tags.len(), 1);
        assert_eq!(d.tags[&remote.id]["name"], "Notebook");
        assert_eq!(d.tasks[&id]["tags"], json!([remote.id]));
        assert!(a.status().await.unwrap().pending);
        roundtrip(&a, &server).await;
        assert_eq!(data(&server).await, d);
        let mut conn = a.pool.acquire().await.unwrap();
        let violations: Vec<sqlx::sqlite::SqliteRow> = sqlx::query("PRAGMA foreign_key_check")
            .fetch_all(&mut *conn)
            .await
            .unwrap();
        assert!(violations.is_empty());
    }
    #[tokio::test]
    async fn v1_state_upgrade_rekeys_baseline_and_keeps_pending_edits_and_receipts() {
        for already_committed in [false, true] {
            let server = pool().await;
            let a = SyncEngine::new(pool().await);
            let tag = make_tag(&server, "Legacy", "tag").await;
            let id = task(&server, "Original").await;
            edit(&server, &id, json!({"tags":["Legacy"]})).await;
            roundtrip(&a, &server).await;
            let baseline = data(&server).await;
            edit(&a.pool, &id, json!({"title":"Pending before upgrade"})).await;
            let pending = data(&a.pool).await;
            let old_local = uid();
            let mut tx = a.pool.begin_with("BEGIN IMMEDIATE").await.unwrap();
            rekey_tag(&mut tx, &tag.id, &old_local).await.unwrap();
            let state = load(&mut tx).await.unwrap();
            let mut raw = serde_json::to_value(&state).unwrap();
            raw["baseline"] = legacy(&baseline);
            let mut local_legacy = legacy(&pending);
            local_legacy["tags"]["Legacy"] = json!(old_local);
            raw["inflight"] = json!({"server_id":state.server_id,"client_id":state.client_id,"sequence":state.sequence+1,"request_id":uid(),"base":legacy(&baseline),"local":local_legacy});
            raw["sequence"] = json!(state.sequence + 1);
            sqlx::query("UPDATE sync_client SET state_json=? WHERE singleton=1")
                .bind(raw.to_string())
                .execute(&mut *tx)
                .await
                .unwrap();
            tx.commit().await.unwrap();
            if already_committed {
                edit(&server, &id, json!({"title":"Pending before upgrade"})).await;
            }
            migrate_client_state(&a.pool).await.unwrap();
            migrate_client_state(&a.pool).await.unwrap();
            let migrated = data(&a.pool).await;
            assert!(migrated.tags.contains_key(&tag.id));
            assert!(!migrated.tags.contains_key(&old_local));
            assert_eq!(migrated.tasks[&id]["title"], "Pending before upgrade");
            assert_eq!(migrated.tasks[&id]["tags"], json!([tag.id]));
            let mut conn = a.pool.acquire().await.unwrap();
            let state = load(&mut conn).await.unwrap();
            drop(conn);
            assert!(state.inflight.is_none());
            assert_eq!(state.baseline, baseline);
            roundtrip(&a, &server).await;
            assert!(!a.status().await.unwrap().pending);
            assert_eq!(
                data(&server).await.tasks[&id]["title"],
                "Pending before upgrade"
            );
        }
    }
    #[tokio::test]
    async fn v1_requests_and_unknown_protocol_cannot_write_v2_database() {
        use tower::ServiceExt;
        let p = pool().await;
        let id = task(&p, "Safe").await;
        let before = data(&p).await;
        let request = json!({"server_id":"old","client_id":uid(),"sequence":1,"request_id":uid(),"base":legacy(&before),"local":legacy(&before)});
        let response = api_router(p.clone())
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/sync")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(request.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let mut invalid = before.clone();
        invalid.protocol = 99;
        assert!(validate(&invalid).is_err());
        assert_eq!(data(&p).await, before);
        assert_eq!(data(&p).await.tasks[&id]["title"], "Safe");
    }
    #[tokio::test]
    async fn concurrent_tag_renames_require_explicit_choice_and_keep_assignments() {
        let server = pool().await;
        let a = SyncEngine::new(pool().await);
        let b = SyncEngine::new(pool().await);
        let tag = make_tag(&server, "Original", "person").await;
        let id = task(&server, "Linked").await;
        edit(&server, &id, json!({"tags":["Original"]})).await;
        roundtrip(&a, &server).await;
        roundtrip(&b, &server).await;
        tag_edit(&a.pool, &tag.id, json!({"name":"Alice"})).await;
        tag_edit(&b.pool, &tag.id, json!({"name":"Bob"})).await;
        roundtrip(&a, &server).await;
        roundtrip(&b, &server).await;
        assert_eq!(b.status().await.unwrap().conflicts[0].entity, "tag");
        assert_eq!(data(&b.pool).await.tags[&tag.id]["name"], "Bob");
        b.resolve_conflicts("local").await.unwrap();
        roundtrip(&b, &server).await;
        roundtrip(&a, &server).await;
        assert_eq!(data(&a.pool).await.tags[&tag.id]["name"], "Bob");
        assert_eq!(data(&server).await.tasks[&id]["tags"], json!([tag.id]));
    }
    #[tokio::test]
    async fn archive_schema_migration_preserves_legacy_data_and_relationships() {
        let p = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .in_memory(true)
                    .foreign_keys(true),
            )
            .await
            .unwrap();
        let migrations = sqlx::migrate!("./migrations");
        for migration in migrations.iter().filter(|m| m.version < 20261018010000) {
            sqlx::raw_sql(&migration.sql).execute(&p).await.unwrap();
        }
        sqlx::query(
            "INSERT INTO areas(id,name,created_at,updated_at) VALUES('a','Work','now','now')",
        )
        .execute(&p)
        .await
        .unwrap();
        sqlx::query("INSERT INTO projects(id,name,area_id,created_at,updated_at) VALUES('p','Project','a','now','now')").execute(&p).await.unwrap();
        sqlx::query("INSERT INTO tags(id,name) VALUES('t','Alice')")
            .execute(&p)
            .await
            .unwrap();
        sqlx::query("INSERT INTO tasks(id,title,project_id,waiting_for,created_at,updated_at) VALUES('x','Existing','p','Alice','now','now')").execute(&p).await.unwrap();
        sqlx::query("INSERT INTO task_tags VALUES('x','t')")
            .execute(&p)
            .await
            .unwrap();
        sqlx::query("INSERT INTO project_tags VALUES('p','t')")
            .execute(&p)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!(
            "../migrations/20261018010000_tag_management.sql"
        ))
        .execute(&p)
        .await
        .unwrap();
        sqlx::raw_sql(include_str!("../migrations/20261018020000_scheduling.sql"))
            .execute(&p)
            .await
            .unwrap();
        let d = data(&p).await;
        assert_eq!(d.tasks["x"]["waiting_for"], "Alice");
        assert_eq!(d.tasks["x"]["tags"], json!(["t"]));
        assert_eq!(d.projects["p"]["tags"], json!(["t"]));
        assert_eq!(d.tags["t"]["kind"], "tag");
        assert_eq!(d.tags["t"]["archived"], 0);
        assert_eq!(d.areas["a"]["archived"], 0);
    }
    #[tokio::test]
    async fn malformed_legacy_upgrade_rolls_back_rekeying_and_keeps_saved_state() {
        let p = pool().await;
        let tag = make_tag(&p, "Legacy", "tag").await;
        let id = task(&p, "Keep").await;
        edit(&p, &id, json!({"tags":["Legacy"]})).await;
        let before = data(&p).await;
        let mut baseline = legacy(&before);
        baseline["tags"]["Legacy"] = json!(uid());
        baseline["tasks"][&id]["tags"] = json!(["Unknown"]);
        let mut raw = serde_json::to_value(ClientState::default()).unwrap();
        raw["baseline"] = baseline;
        sqlx::query("INSERT INTO sync_client(singleton,state_json) VALUES(1,?)")
            .bind(raw.to_string())
            .execute(&p)
            .await
            .unwrap();
        assert!(migrate_client_state(&p).await.is_err());
        assert_eq!(data(&p).await, before);
        let saved: String = sqlx::query_scalar("SELECT state_json FROM sync_client")
            .fetch_one(&p)
            .await
            .unwrap();
        assert_eq!(saved, raw.to_string());
        assert!(data(&p).await.tags.contains_key(&tag.id));
    }
    #[tokio::test]
    async fn v2_upgrade_keeps_pending_edits_and_rejects_old_peers() {
        let p = pool().await;
        let id = task(&p, "Preserved before upgrade").await;
        let baseline = data(&p).await;
        let state = ClientState {
            baseline: baseline.clone(),
            sequence: 7,
            remote_url: Some("https://server.invalid".into()),
            ..ClientState::default()
        };
        let mut legacy = serde_json::to_value(state).unwrap();
        legacy["baseline"]["protocol"] = json!(2);
        legacy["baseline"].as_object_mut().unwrap().remove("clock");
        for task in legacy["baseline"]["tasks"]
            .as_object_mut()
            .unwrap()
            .values_mut()
        {
            task.as_object_mut().unwrap().remove("recurrence");
        }
        legacy["inflight"] = json!({"previous":"request retained in server receipts"});
        sqlx::query("INSERT INTO sync_client VALUES(1,?)")
            .bind(legacy.to_string())
            .execute(&p)
            .await
            .unwrap();
        sqlx::query("UPDATE tasks SET notes='Pending local edit' WHERE id=?")
            .bind(&id)
            .execute(&p)
            .await
            .unwrap();
        migrate_client_state(&p).await.unwrap();
        migrate_client_state(&p).await.unwrap();
        let state = load(&mut *p.acquire().await.unwrap()).await.unwrap();
        assert_eq!(state.baseline.protocol, 3);
        assert_eq!(state.sequence, 7);
        assert!(state.inflight.is_none());
        assert_eq!(state.baseline.tasks[&id]["notes"], "");
        assert_eq!(data(&p).await.tasks[&id]["notes"], "Pending local edit");
        let mut old = baseline;
        old.protocol = 2;
        assert!(validate(&old).is_err());
    }
    #[tokio::test]
    async fn repeat_and_travel_sync_generate_once_on_server_and_merge_unrelated_edits() {
        let server = pool().await;
        let client = SyncEngine::new(pool().await);
        let id = task(&server, "Repeat source").await;
        roundtrip(&client, &server).await;
        let mut tx = client.pool.begin_with("BEGIN IMMEDIATE").await.unwrap();
        let mut state = load(&mut tx).await.unwrap();
        state.remote_url = Some("https://server.invalid".into());
        save(&mut tx, &state).await.unwrap();
        schedule::write_clock(
            &mut tx,
            &schedule::Clock {
                timezone: "America/Hermosillo".into(),
                updated_at: stamp(),
            },
        )
        .await
        .unwrap();
        tx.commit().await.unwrap();
        let today = Utc::now()
            .with_timezone(&chrono_tz::America::Hermosillo)
            .date_naive()
            .to_string();
        edit(&client.pool,&id,json!({"recurrence":{"id":uid(),"frequency":"daily","interval":1,"start":today,"until":today,"due_days":null,"paused":false}})).await;
        assert_eq!(data(&client.pool).await.tasks.len(), 1);
        edit(&server, &id, json!({"notes":"Independent server edit"})).await;
        roundtrip(&client, &server).await;
        assert!(client.status().await.unwrap().conflicts.is_empty());
        assert_eq!(data(&server).await.clock.timezone, "America/Hermosillo");
        let first = data(&server).await;
        assert_eq!(first.tasks.len(), 2);
        assert!(first
            .tasks
            .values()
            .all(|t| t["notes"] == "Independent server edit"));
        roundtrip(&client, &server).await;
        schedule::tick(&server).await.unwrap();
        assert_eq!(data(&server).await, first);
        assert_eq!(data(&client.pool).await, first);
    }
}
