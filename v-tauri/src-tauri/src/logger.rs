use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use tauri::{AppHandle, Manager};
use time::macros::format_description;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, fmt::time::LocalTime, prelude::*, EnvFilter};

const LOG_DIR_NAME: &str = "logs";
const LOG_FILE_NAME: &str = "app.log";

static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppLogEntry {
    pub level: String,
    pub target: Option<String>,
    pub message: String,
    pub context: Option<Value>,
}

fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|err| err.to_string())?;
    fs::create_dir_all(&dir).map_err(|err| err.to_string())?;
    Ok(dir)
}

fn log_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app_data_dir(app)?.join(LOG_DIR_NAME);
    fs::create_dir_all(&dir).map_err(|err| err.to_string())?;
    Ok(dir)
}

pub fn log_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(log_dir(app)?.join(LOG_FILE_NAME))
}

pub fn init(app: &AppHandle) -> Result<(), String> {
    let dir = log_dir(app)?;
    let file_appender = tracing_appender::rolling::never(&dir, LOG_FILE_NAME);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    
    let timer = LocalTime::new(format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"));
    
    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_ansi(false)
                .with_target(true)
                .with_timer(timer)
                .with_writer(non_blocking),
        );

    tracing::subscriber::set_global_default(subscriber).map_err(|err| err.to_string())?;
    let _ = LOG_GUARD.set(guard);
    tracing::info!(target: "backend.app", log_file = %log_file_path(app)?.display(), "logger initialized");
    Ok(())
}

fn format_log_message(message: String, context: Option<Value>) -> String {
    match context {
        Some(context) if !context.is_null() => format!("{message} | context={context}"),
        _ => message,
    }
}

fn write_entry(entry: AppLogEntry) {
    let target = entry.target.unwrap_or_else(|| "frontend.app".to_string());
    let message = format_log_message(entry.message, entry.context);
    match entry.level.to_ascii_lowercase().as_str() {
        "trace" => tracing::trace!(target: "frontend.app", log_target = %target, "{message}"),
        "debug" => tracing::debug!(target: "frontend.app", log_target = %target, "{message}"),
        "warn" => tracing::warn!(target: "frontend.app", log_target = %target, "{message}"),
        "error" => tracing::error!(target: "frontend.app", log_target = %target, "{message}"),
        _ => tracing::info!(target: "frontend.app", log_target = %target, "{message}"),
    }
}

#[tauri::command]
pub fn app_log_write(entry: AppLogEntry) {
    write_entry(entry);
}

#[tauri::command]
pub fn app_log_write_batch(entries: Vec<AppLogEntry>) {
    for entry in entries {
        write_entry(entry);
    }
}

#[tauri::command]
pub fn app_log_read(app: AppHandle, offset: Option<usize>, limit: Option<usize>) -> Result<Vec<String>, String> {
    let log_path = log_file_path(&app)?;
    if !log_path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(log_path).map_err(|err| err.to_string())?;
    let mut lines: Vec<&str> = content.lines().filter(|l| !l.is_empty()).collect();
    lines.reverse();

    let offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(500);

    let chunk = lines.into_iter().skip(offset).take(limit).map(|s| s.to_string()).collect();
    Ok(chunk)
}

#[tauri::command]
pub fn app_log_clear(app: AppHandle) -> Result<(), String> {
    let log_path = log_file_path(&app)?;
    fs::write(log_path, "").map_err(|err| err.to_string())
}

#[tauri::command]
pub fn app_log_path(app: AppHandle) -> Result<String, String> {
    Ok(log_file_path(&app)?.to_string_lossy().to_string())
}
