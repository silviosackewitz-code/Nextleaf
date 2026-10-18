use axum::{
    body::{to_bytes, Body},
    http::{Method, Request},
};
use nextleaf_server::sync::{SyncEngine, SyncStatus};
use serde::Serialize;
use sqlx::SqlitePool;
use std::{path::PathBuf, sync::Arc};
use tauri::{Emitter, Manager};
use tokio::sync::Mutex;
use tower::ServiceExt;

struct DesktopState {
    pool: SqlitePool,
    sync_lock: Arc<Mutex<()>>,
    data_dir: PathBuf,
}

fn data_directory() -> Result<PathBuf, String> {
    // An explicit override keeps integration tests away from the user's data.
    if let Some(path) = std::env::var_os("NEXTLEAF_DESKTOP_DATA_DIR") {
        return Ok(PathBuf::from(path));
    }
    dirs::data_dir()
        .map(|path| path.join("dev.taskmanager.desktop"))
        .ok_or_else(|| "Could not locate the application data directory.".into())
}

#[derive(Serialize)]
struct ApiResponse {
    status: u16,
    body: serde_json::Value,
}

#[tauri::command]
async fn desktop_api(
    path: String,
    method: String,
    body: Option<String>,
    state: tauri::State<'_, DesktopState>,
) -> Result<ApiResponse, String> {
    if !path.starts_with('/') || path.starts_with("//") || path.contains('#') {
        return Err("Invalid local API path.".into());
    }
    let method = match method.as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PATCH" => Method::PATCH,
        _ => return Err("Unsupported local API method.".into()),
    };
    if body
        .as_ref()
        .is_some_and(|value| value.len() > 2 * 1024 * 1024)
    {
        return Err("Request is too large.".into());
    }
    let request = Request::builder()
        .method(method)
        .uri(format!("/api{path}"))
        .header("Content-Type", "application/json")
        .body(Body::from(body.unwrap_or_default()))
        .map_err(|e| e.to_string())?;
    let response = nextleaf_server::api_router(state.pool.clone())
        .oneshot(request)
        .await
        .map_err(|e| e.to_string())?;
    let status = response.status().as_u16();
    let bytes = to_bytes(response.into_body(), 32 * 1024 * 1024)
        .await
        .map_err(|e| e.to_string())?;
    let body = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| serde_json::json!({"error": "Invalid local API response."}));
    Ok(ApiResponse { status, body })
}

#[tauri::command]
async fn desktop_status(state: tauri::State<'_, DesktopState>) -> Result<SyncStatus, String> {
    SyncEngine::new(state.pool.clone()).status().await
}

#[tauri::command]
async fn desktop_sync(
    remote: String,
    state: tauri::State<'_, DesktopState>,
    app: tauri::AppHandle,
) -> Result<SyncStatus, String> {
    let _guard = state.sync_lock.lock().await;
    let result = SyncEngine::new(state.pool.clone()).sync_once(&remote).await;
    let _ = app.emit("nextleaf-data-changed", ());
    result
}

#[tauri::command]
async fn desktop_resolve(
    choice: String,
    state: tauri::State<'_, DesktopState>,
    app: tauri::AppHandle,
) -> Result<SyncStatus, String> {
    if !matches!(choice.as_str(), "local" | "server") {
        return Err("Invalid conflict resolution.".into());
    }
    let _guard = state.sync_lock.lock().await;
    let result = SyncEngine::new(state.pool.clone())
        .resolve_conflicts(&choice)
        .await;
    let _ = app.emit("nextleaf-data-changed", ());
    result
}

#[derive(Serialize)]
struct DesktopInfo {
    data_directory: String,
    mcp_command: String,
    mcp_args: Vec<String>,
}

#[tauri::command]
fn desktop_info(state: tauri::State<'_, DesktopState>) -> Result<DesktopInfo, String> {
    Ok(DesktopInfo {
        data_directory: state.data_dir.to_string_lossy().into_owned(),
        mcp_command: std::env::current_exe()
            .map_err(|e| e.to_string())?
            .to_string_lossy()
            .into_owned(),
        mcp_args: vec!["--mcp".into()],
    })
}

async fn background_sync(pool: SqlitePool, lock: Arc<Mutex<()>>, app: Option<tauri::AppHandle>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));
    loop {
        interval.tick().await;
        let _guard = lock.lock().await;
        let engine = SyncEngine::new(pool.clone());
        if let Ok(status) = engine.status().await {
            if let Some(remote) = status.remote_url.filter(|url| !url.is_empty()) {
                let _ = engine.sync_once(&remote).await;
                if let Some(app) = &app {
                    let _ = app.emit("nextleaf-data-changed", ());
                }
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if std::env::args().any(|arg| arg == "--mcp") {
        let result: Result<(), String> = (|| {
            let directory = data_directory()?;
            let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
            runtime.block_on(async {
                let pool = nextleaf_server::create_pool(&directory.join("nextleaf.db"))
                    .await
                    .map_err(|e| e.to_string())?;
                tokio::spawn(nextleaf_server::schedule::run(pool.clone()));
                tokio::spawn(background_sync(
                    pool.clone(),
                    Arc::new(Mutex::new(())),
                    None,
                ));
                nextleaf_server::serve_stdio(pool)
                    .await
                    .map_err(|e| e.to_string())
            })
        })();
        if let Err(error) = result {
            eprintln!("Nextleaf MCP: {error}");
            std::process::exit(1);
        }
        return;
    }
    tauri::Builder::default()
        .setup(|app| {
            let directory = data_directory().map_err(std::io::Error::other)?;
            let pool = tauri::async_runtime::block_on(nextleaf_server::create_pool(
                &directory.join("nextleaf.db"),
            ))
            .map_err(|e| std::io::Error::other(e.to_string()))?;
            let lock = Arc::new(Mutex::new(()));
            app.manage(DesktopState {
                pool: pool.clone(),
                sync_lock: lock.clone(),
                data_dir: directory,
            });
            tauri::async_runtime::spawn(nextleaf_server::schedule::run(pool.clone()));
            tauri::async_runtime::spawn(background_sync(pool, lock, Some(app.handle().clone())));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            desktop_api,
            desktop_status,
            desktop_sync,
            desktop_resolve,
            desktop_info
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Nextleaf Desktop");
}
