use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderValue, Method, StatusCode},
    routing::{get, patch},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{sqlite::SqlitePoolOptions, FromRow, SqlitePool};
use std::{collections::HashMap, env, net::SocketAddr, path::PathBuf};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}
type ApiResult<T> = Result<Json<T>, (StatusCode, Json<Value>)>;

fn bad_request(message: &str) -> (StatusCode, Json<Value>) {
    (StatusCode::BAD_REQUEST, Json(json!({"error": message})))
}
fn internal(error: sqlx::Error) -> (StatusCode, Json<Value>) {
    tracing::error!("Database error: {error}");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error":"database error"})),
    )
}
fn not_found() -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_FOUND, Json(json!({"error":"not found"})))
}
fn stamp() -> String {
    Utc::now().to_rfc3339()
}
fn uid() -> String {
    Uuid::new_v4().to_string()
}
fn required_name(input: &str) -> Result<String, (StatusCode, Json<Value>)> {
    let name = input.trim();
    if name.is_empty() || name.len() > 200 {
        Err(bad_request("Name must be 1–200 characters"))
    } else {
        Ok(name.to_owned())
    }
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct Area {
    id: String,
    name: String,
    sort_order: i64,
}
#[derive(Debug, Deserialize)]
struct NewArea {
    name: String,
}
async fn list_areas(State(state): State<AppState>) -> ApiResult<Vec<Area>> {
    let rows = sqlx::query_as::<_, Area>(
        "SELECT id, name, sort_order FROM areas ORDER BY sort_order, name",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(internal)?;
    Ok(Json(rows))
}
async fn create_area(State(state): State<AppState>, Json(input): Json<NewArea>) -> ApiResult<Area> {
    let name = required_name(&input.name)?;
    let id = uid();
    let now = stamp();
    let order: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(sort_order), 0) + 100 FROM areas")
        .fetch_one(&state.pool)
        .await
        .map_err(internal)?;
    sqlx::query(
        "INSERT INTO areas (id, name, sort_order, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&name)
    .bind(order)
    .bind(&now)
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(internal)?;
    Ok(Json(Area {
        id,
        name,
        sort_order: order,
    }))
}

#[derive(Debug, Clone, Serialize, FromRow)]
struct Project {
    id: String,
    area_id: Option<String>,
    name: String,
    notes: String,
    project_type: String,
    status: String,
    sort_order: i64,
}
#[derive(Debug, Deserialize)]
struct NewProject {
    name: String,
    area_id: Option<String>,
    project_type: Option<String>,
    notes: Option<String>,
}
#[derive(Debug, Deserialize)]
struct ProjectFilter {
    area_id: Option<String>,
}
async fn list_projects(
    State(state): State<AppState>,
    Query(filter): Query<ProjectFilter>,
) -> ApiResult<Vec<Project>> {
    let rows = sqlx::query_as::<_, Project>(
        "SELECT id,area_id,name,notes,project_type,status,sort_order FROM projects WHERE (? IS NULL OR area_id = ?) ORDER BY sort_order,name"
    ).bind(&filter.area_id).bind(&filter.area_id).fetch_all(&state.pool).await.map_err(internal)?;
    Ok(Json(rows))
}
async fn create_project(
    State(state): State<AppState>,
    Json(input): Json<NewProject>,
) -> ApiResult<Project> {
    let name = required_name(&input.name)?;
    let project_type = input.project_type.unwrap_or_else(|| "parallel".into());
    if project_type != "sequential" && project_type != "parallel" {
        return Err(bad_request("Invalid project_type"));
    }
    if let Some(area) = &input.area_id {
        ensure_area(&state.pool, area).await?;
    }
    let now = stamp();
    let id = uid();
    let notes = input.notes.unwrap_or_default();
    let order: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(sort_order), 0) + 100 FROM projects")
        .fetch_one(&state.pool)
        .await
        .map_err(internal)?;
    sqlx::query("INSERT INTO projects (id,area_id,name,notes,project_type,status,sort_order,created_at,updated_at) VALUES (?,?,?,?,?,'active',?,?,?)")
        .bind(&id).bind(&input.area_id).bind(&name).bind(&notes).bind(&project_type).bind(order).bind(&now).bind(&now)
        .execute(&state.pool).await.map_err(internal)?;
    Ok(Json(Project {
        id,
        area_id: input.area_id,
        name,
        notes,
        project_type,
        status: "active".into(),
        sort_order: order,
    }))
}
async fn ensure_area(pool: &SqlitePool, id: &str) -> Result<(), (StatusCode, Json<Value>)> {
    let found: Option<i64> = sqlx::query_scalar("SELECT 1 FROM areas WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(internal)?;
    if found.is_none() {
        return Err(bad_request("Unknown area_id"));
    }
    Ok(())
}
async fn ensure_project(pool: &SqlitePool, id: &str) -> Result<(), (StatusCode, Json<Value>)> {
    let found: Option<i64> = sqlx::query_scalar("SELECT 1 FROM projects WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(internal)?;
    if found.is_none() {
        return Err(bad_request("Unknown project_id"));
    }
    Ok(())
}

#[derive(Debug, Clone, FromRow)]
struct TaskRow {
    id: String,
    project_id: Option<String>,
    area_id: Option<String>,
    title: String,
    notes: String,
    status: String,
    sort_order: i64,
    due_date: Option<String>,
    defer_until: Option<String>,
    completed_at: Option<String>,
    created_at: String,
    updated_at: String,
    project_type: Option<String>,
    project_status: Option<String>,
    project_area_id: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
struct Task {
    id: String,
    project_id: Option<String>,
    area_id: Option<String>,
    title: String,
    notes: String,
    status: String,
    sort_order: i64,
    due_date: Option<String>,
    defer_until: Option<String>,
    completed_at: Option<String>,
    created_at: String,
    updated_at: String,
    available: bool,
    blocked_by_sequence: bool,
}
#[derive(Debug, Default, Deserialize)]
struct TaskFilter {
    view: Option<String>,
    area_id: Option<String>,
    project_id: Option<String>,
}
#[derive(Debug, Deserialize)]
struct NewTask {
    title: String,
    project_id: Option<String>,
    area_id: Option<String>,
    status: Option<String>,
    notes: Option<String>,
}
#[derive(Debug, Deserialize)]
struct UpdateTask {
    title: Option<String>,
    status: Option<String>,
}
fn valid_task_status(status: &str) -> bool {
    matches!(
        status,
        "inbox" | "next" | "waiting" | "scheduled" | "someday" | "completed"
    )
}

// Centralized rule: the first unfinished task of each sequential project is its only
// candidate for Next. If that first task is Waiting/Scheduled/Someday, don't skip it.
fn decorate_tasks(rows: Vec<TaskRow>) -> Vec<Task> {
    let mut first_open: HashMap<String, String> = HashMap::new();
    for task in &rows {
        if let (Some(project_id), Some(kind)) = (&task.project_id, &task.project_type) {
            if kind == "sequential"
                && task.status != "completed"
                && task.project_status.as_deref() == Some("active")
            {
                first_open
                    .entry(project_id.clone())
                    .or_insert_with(|| task.id.clone());
            }
        }
    }
    let today = Utc::now().date_naive().to_string();
    rows.into_iter()
        .map(|task| {
            let in_sequence = task.project_type.as_deref() == Some("sequential");
            let blocked = in_sequence
                && task.status != "completed"
                && first_open
                    .get(task.project_id.as_deref().unwrap_or(""))
                    .map(String::as_str)
                    != Some(task.id.as_str());
            let deferred = task
                .defer_until
                .as_deref()
                .is_some_and(|date| date > today.as_str());
            let project_active = task.project_status.as_deref().is_none_or(|s| s == "active");
            let available = task.status == "next" && project_active && !deferred && !blocked;
            Task {
                id: task.id,
                project_id: task.project_id,
                area_id: task.project_area_id.or(task.area_id),
                title: task.title,
                notes: task.notes,
                status: task.status,
                sort_order: task.sort_order,
                due_date: task.due_date,
                defer_until: task.defer_until,
                completed_at: task.completed_at,
                created_at: task.created_at,
                updated_at: task.updated_at,
                available,
                blocked_by_sequence: blocked,
            }
        })
        .collect()
}
async fn fetch_tasks(pool: &SqlitePool) -> Result<Vec<Task>, (StatusCode, Json<Value>)> {
    let rows = sqlx::query_as::<_,TaskRow>(
        "SELECT t.id,t.project_id,t.area_id,t.title,t.notes,t.status,t.sort_order,t.due_date,t.defer_until,t.completed_at,t.created_at,t.updated_at, p.project_type,p.status AS project_status,p.area_id AS project_area_id FROM tasks t LEFT JOIN projects p ON t.project_id=p.id ORDER BY t.sort_order,t.created_at,t.id"
    ).fetch_all(pool).await.map_err(internal)?;
    Ok(decorate_tasks(rows))
}
async fn list_tasks(
    State(state): State<AppState>,
    Query(filter): Query<TaskFilter>,
) -> ApiResult<Vec<Task>> {
    let view = filter.view.as_deref().unwrap_or("all");
    if !matches!(
        view,
        "all" | "inbox" | "next" | "waiting" | "scheduled" | "someday" | "completed"
    ) {
        return Err(bad_request("Invalid view"));
    }
    let rows = fetch_tasks(&state.pool).await?;
    Ok(Json(
        rows.into_iter()
            .filter(|task| {
                let matches_view = if view == "next" {
                    task.available
                } else if view == "all" {
                    task.status != "completed"
                } else {
                    task.status == view
                };
                matches_view
                    && filter
                        .area_id
                        .as_ref()
                        .is_none_or(|id| task.area_id.as_ref() == Some(id))
                    && filter
                        .project_id
                        .as_ref()
                        .is_none_or(|id| task.project_id.as_ref() == Some(id))
            })
            .collect(),
    ))
}
async fn create_task(State(state): State<AppState>, Json(input): Json<NewTask>) -> ApiResult<Task> {
    let title = required_name(&input.title)?;
    let status = input.status.unwrap_or_else(|| "inbox".into());
    if !valid_task_status(&status) {
        return Err(bad_request("Invalid task status"));
    }
    if let Some(id) = &input.project_id {
        ensure_project(&state.pool, id).await?;
    }
    if let Some(id) = &input.area_id {
        ensure_area(&state.pool, id).await?;
    }
    let id = uid();
    let now = stamp();
    let order: i64 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(sort_order),0)+100 FROM tasks WHERE project_id IS ?",
    )
    .bind(&input.project_id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal)?;
    let completed_at = if status == "completed" {
        Some(now.clone())
    } else {
        None
    };
    sqlx::query("INSERT INTO tasks (id,project_id,area_id,title,notes,status,sort_order,completed_at,created_at,updated_at) VALUES (?,?,?,?,?,?,?,?,?,?)")
        .bind(&id).bind(&input.project_id).bind(&input.area_id).bind(&title).bind(input.notes.unwrap_or_default())
        .bind(&status).bind(order).bind(completed_at).bind(&now).bind(&now)
        .execute(&state.pool).await.map_err(internal)?;
    fetch_tasks(&state.pool)
        .await?
        .into_iter()
        .find(|task| task.id == id)
        .map(Json)
        .ok_or_else(not_found)
}
async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(input): Json<UpdateTask>,
) -> ApiResult<Task> {
    if input.title.is_none() && input.status.is_none() {
        return Err(bad_request("No changes"));
    }
    let title = match input.title {
        Some(s) => Some(required_name(&s)?),
        None => None,
    };
    if input.status.as_ref().is_some_and(|s| !valid_task_status(s)) {
        return Err(bad_request("Invalid task status"));
    }
    let previous: Option<String> = sqlx::query_scalar("SELECT status FROM tasks WHERE id=?")
        .bind(&id)
        .fetch_optional(&state.pool)
        .await
        .map_err(internal)?;
    let Some(old_status) = previous else {
        return Err(not_found());
    };
    let completed_at = match input.status.as_deref() {
        Some("completed") if old_status != "completed" => Some(stamp()),
        Some("completed") => sqlx::query_scalar("SELECT completed_at FROM tasks WHERE id=?")
            .bind(&id)
            .fetch_one(&state.pool)
            .await
            .map_err(internal)?,
        Some(_) => None,
        None => sqlx::query_scalar("SELECT completed_at FROM tasks WHERE id=?")
            .bind(&id)
            .fetch_one(&state.pool)
            .await
            .map_err(internal)?,
    };
    sqlx::query("UPDATE tasks SET title=COALESCE(?,title), status=COALESCE(?,status),completed_at=?,updated_at=? WHERE id=?")
        .bind(title).bind(input.status).bind(completed_at).bind(stamp()).bind(&id)
        .execute(&state.pool).await.map_err(internal)?;
    fetch_tasks(&state.pool)
        .await?
        .into_iter()
        .find(|task| task.id == id)
        .map(Json)
        .ok_or_else(not_found)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "taskmanager_server=info,tower_http=info".into()),
        )
        .init();
    let bind: SocketAddr = env::var("TASKMANAGER_BIND")
        .unwrap_or_else(|_| "127.0.0.1:8787".into())
        .parse()?;
    let database_url = env::var("TASKMANAGER_DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://../data/taskmanager.db".into());
    let db_path = database_url.trim_start_matches("sqlite://");
    if db_path != ":memory:" {
        if let Some(parent) = std::path::Path::new(db_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            database_url
                .parse::<sqlx::sqlite::SqliteConnectOptions>()?
                .create_if_missing(true)
                .foreign_keys(true),
        )
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let api = Router::new()
        .route("/health", get(|| async { Json(json!({"status":"ok"})) }))
        .route("/areas", get(list_areas).post(create_area))
        .route("/projects", get(list_projects).post(create_project))
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/{id}", patch(update_task))
        .with_state(AppState { pool });
    let web_dir = PathBuf::from(
        env::var("TASKMANAGER_WEB_DIR").unwrap_or_else(|_| "../frontend/dist".into()),
    );
    let mut app = Router::new().nest("/api", api);
    if web_dir.join("index.html").exists() {
        app = app.fallback_service(
            ServeDir::new(&web_dir).not_found_service(ServeFile::new(web_dir.join("index.html"))),
        );
        tracing::info!("Serving web build from {}", web_dir.display());
    }
    // Local development UI and packaged desktop origin only; never open unrestricted CORS.
    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://127.0.0.1:1420"),
            HeaderValue::from_static("http://localhost:1420"),
            HeaderValue::from_static("tauri://localhost"),
            HeaderValue::from_static("http://tauri.localhost"),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PATCH])
        .allow_headers([header::CONTENT_TYPE]);
    app = app.layer(cors).layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(bind).await?;
    tracing::info!("TaskManager listening on http://{bind}");
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn example(id: &str, project: &str, status: &str, order: i64) -> TaskRow {
        TaskRow {
            id: id.into(),
            project_id: Some(project.into()),
            area_id: None,
            title: id.into(),
            notes: String::new(),
            status: status.into(),
            sort_order: order,
            due_date: None,
            defer_until: None,
            completed_at: None,
            created_at: "2026-10-17T10:00:00Z".into(),
            updated_at: "2026-10-17T10:00:00Z".into(),
            project_type: Some("sequential".into()),
            project_status: Some("active".into()),
            project_area_id: None,
        }
    }
    #[test]
    fn sequential_exposes_only_first_unfinished() {
        let tasks = decorate_tasks(vec![
            example("one", "project", "next", 100),
            example("two", "project", "next", 200),
        ]);
        assert!(tasks[0].available);
        assert!(!tasks[1].available);
        assert!(tasks[1].blocked_by_sequence);
    }
    #[test]
    fn waiting_first_blocks_second() {
        let tasks = decorate_tasks(vec![
            example("one", "project", "waiting", 100),
            example("two", "project", "next", 200),
        ]);
        assert!(!tasks[0].available);
        assert!(!tasks[1].available);
    }
    #[test]
    fn completed_first_releases_second() {
        let tasks = decorate_tasks(vec![
            example("one", "project", "completed", 100),
            example("two", "project", "next", 200),
        ]);
        assert!(tasks[1].available);
    }
}
