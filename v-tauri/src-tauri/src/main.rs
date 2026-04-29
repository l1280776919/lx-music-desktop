#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::{Map, Value};
use std::path::PathBuf;

fn config_root_dir() -> Result<PathBuf, String> {
  let base = tauri::api::path::config_dir().ok_or_else(|| "config_dir not found".to_string())?;
  Ok(base.join("lx-music-tauri"))
}

fn app_setting_path() -> Result<PathBuf, String> {
  Ok(config_root_dir()?.join("app_setting.json"))
}

fn load_app_setting() -> Result<Value, String> {
  let path = app_setting_path()?;
  if !path.exists() {
    return Ok(Value::Object(Map::new()));
  }
  let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
  serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}

fn save_app_setting(setting: &Value) -> Result<(), String> {
  let root = config_root_dir()?;
  std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
  let path = app_setting_path()?;
  let bytes = serde_json::to_vec_pretty(setting).map_err(|e| e.to_string())?;
  std::fs::write(path, bytes).map_err(|e| e.to_string())?;
  Ok(())
}

fn merge_object(base: &mut Map<String, Value>, patch: &Map<String, Value>) {
  for (k, v) in patch {
    base.insert(k.clone(), v.clone());
  }
}

#[tauri::command]
fn lx_ipc_invoke(channel: String, params: Option<Value>) -> Result<Value, String> {
  match channel.as_str() {
    "get_app_setting" => load_app_setting(),
    "set_app_setting" => {
      let mut base = load_app_setting()?;
      let patch = params.unwrap_or(Value::Object(Map::new()));
      match (&mut base, patch) {
        (Value::Object(base_obj), Value::Object(patch_obj)) => {
          merge_object(base_obj, &patch_obj);
          save_app_setting(&Value::Object(base_obj.clone()))?;
          Ok(Value::Null)
        }
        _ => Err("set_app_setting params must be an object".to_string()),
      }
    }
    "get_env_params" => Ok(serde_json::json!({
      "platform": std::env::consts::OS,
      "arch": std::env::consts::ARCH,
      "version": env!("CARGO_PKG_VERSION"),
      "configDir": config_root_dir()?.to_string_lossy().to_string()
    })),
    _ => Err(format!("unsupported channel: {channel}")),
  }
}

#[tauri::command]
fn lx_ipc_send(channel: String, _params: Option<Value>) -> Result<(), String> {
  match channel.as_str() {
    _ => Ok(()),
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![lx_ipc_invoke, lx_ipc_send])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

