#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::{Map, Value};
use std::path::PathBuf;

const MODULES: [&str; 6] = ["common", "player", "dislike", "winMain", "winLyric", "hotKey"];

fn config_root_dir() -> Result<PathBuf, String> {
  let base = tauri::api::path::config_dir().ok_or_else(|| "config_dir not found".to_string())?;
  Ok(base.join("lx-music-tauri"))
}

fn parse_channel(channel: &str) -> (Option<&str>, &str) {
  if let Some((module, rest)) = channel.split_once('_') {
    if MODULES.contains(&module) {
      return (Some(module), rest);
    }
  }
  (None, channel)
}

fn data_store_path() -> Result<PathBuf, String> {
  Ok(config_root_dir()?.join("data.json"))
}

fn app_setting_path() -> Result<PathBuf, String> {
  Ok(config_root_dir()?.join("app_setting.json"))
}

fn load_data_store() -> Result<Map<String, Value>, String> {
  let path = data_store_path()?;
  if !path.exists() {
    return Ok(Map::new());
  }
  let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
  let v: Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
  match v {
    Value::Object(obj) => Ok(obj),
    _ => Ok(Map::new()),
  }
}

fn save_data_store(store: &Map<String, Value>) -> Result<(), String> {
  let root = config_root_dir()?;
  std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
  let path = data_store_path()?;
  let bytes = serde_json::to_vec_pretty(&Value::Object(store.clone())).map_err(|e| e.to_string())?;
  std::fs::write(path, bytes).map_err(|e| e.to_string())?;
  Ok(())
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

fn default_hotkey_config() -> Value {
  serde_json::json!({
    "local": { "enable": false, "keys": {} },
    "global": { "enable": false, "keys": {} }
  })
}

#[tauri::command]
fn lx_ipc_invoke(window: tauri::Window, channel: String, params: Option<Value>) -> Result<Value, String> {
  let (module, name) = parse_channel(&channel);
  match (module, name) {
    (Some("common"), "get_app_setting") | (None, "get_app_setting") => load_app_setting(),
    (Some("common"), "set_app_setting") | (None, "set_app_setting") => {
      let mut base = load_app_setting()?;
      let patch = params.unwrap_or(Value::Object(Map::new()));
      match (&mut base, patch) {
        (Value::Object(base_obj), Value::Object(patch_obj)) => {
          merge_object(base_obj, &patch_obj);
          save_app_setting(&Value::Object(base_obj.clone()))?;
          Ok(Value::Null)
        }
        _ => Ok(Value::Null),
      }
    }
    (Some("common"), "get_env_params") | (None, "get_env_params") => Ok(serde_json::json!({
      "platform": std::env::consts::OS,
      "arch": std::env::consts::ARCH,
      "version": env!("CARGO_PKG_VERSION"),
      "configDir": config_root_dir()?.to_string_lossy().to_string()
    })),
    (Some("common"), "get_system_fonts") | (None, "get_system_fonts") => Ok(Value::Array(vec![])),

    (Some("winMain"), "get_data") | (None, "get_data") => {
      let key = match params {
        Some(Value::String(s)) => s,
        _ => return Ok(Value::Null),
      };
      let store = load_data_store()?;
      Ok(store.get(&key).cloned().unwrap_or(Value::Null))
    }
    (Some("winMain"), "get_hot_key") | (None, "get_hot_key") => Ok(default_hotkey_config()),
    (Some("winMain"), "download_list_get") | (None, "download_list_get") => Ok(Value::Array(vec![])),
    (Some("winMain"), "get_user_api_list") | (None, "get_user_api_list") => Ok(Value::Array(vec![])),
    (Some("winMain"), "get_other_source") | (None, "get_other_source") => Ok(Value::Array(vec![])),
    (Some("winMain"), "get_other_source_count") | (None, "get_other_source_count") => Ok(Value::from(0)),
    (Some("winMain"), "get_music_url_count") | (None, "get_music_url_count") => Ok(Value::from(0)),
    (Some("winMain"), "get_lyric_raw_count") | (None, "get_lyric_raw_count") => Ok(Value::from(0)),
    (Some("winMain"), "get_lyric_edited_count") | (None, "get_lyric_edited_count") => Ok(Value::from(0)),
    (Some("winMain"), "get_sound_effect_eq_preset") | (None, "get_sound_effect_eq_preset") => Ok(Value::Array(vec![])),
    (Some("winMain"), "get_sound_effect_convolution_preset") | (None, "get_sound_effect_convolution_preset") => Ok(Value::Array(vec![])),
    (Some("winMain"), "show_select_dialog") | (None, "show_select_dialog") => Ok(serde_json::json!({ "canceled": true, "filePaths": [] })),
    (Some("winMain"), "show_save_dialog") | (None, "show_save_dialog") => Ok(serde_json::json!({ "canceled": true, "filePath": null })),
    (Some("winMain"), "show_dialog") | (None, "show_dialog") => Ok(Value::Null),
    (Some("winMain"), "get_themes") | (None, "get_themes") => Ok(Value::Array(vec![])),
    (Some("winMain"), "fullscreen") | (None, "fullscreen") => {
      let enable = params.and_then(|v| v.as_bool()).unwrap_or(false);
      window.set_fullscreen(enable).map_err(|e| e.to_string())?;
      Ok(Value::from(enable))
    }

    (Some("player"), "list_get") => Ok(Value::Array(vec![])),
    (Some("player"), "list_music_get") => Ok(Value::Array(vec![])),
    (Some("player"), "list_music_check_exist") => Ok(Value::from(false)),
    (Some("player"), "list_music_get_list_ids") => Ok(Value::Array(vec![])),

    (Some("dislike"), "get_dislike_music_infos") => {
      let store = load_data_store()?;
      let rules = store.get("dislike_rules").and_then(|v| v.as_str()).unwrap_or("").to_string();
      Ok(serde_json::json!({
        "rules": rules,
        "names": [],
        "musicNames": [],
        "singerNames": []
      }))
    }

    (Some("hotKey"), "status") => Ok(Value::Object(Map::new())),
    _ => Ok(Value::Null),
  }
}

#[tauri::command]
fn lx_ipc_send(window: tauri::Window, channel: String, params: Option<Value>) -> Result<(), String> {
  let (module, name) = parse_channel(&channel);
  match (module, name) {
    (Some("winMain"), "save_data") | (None, "save_data") => {
      let (key, value) = match params {
        Some(Value::Object(obj)) => {
          let key = obj.get("path").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          let value = obj.get("data").cloned().unwrap_or(Value::Null);
          (key, value)
        }
        _ => (String::new(), Value::Null),
      };
      if key.is_empty() {
        return Ok(());
      }
      let mut store = load_data_store()?;
      store.insert(key, value);
      save_data_store(&store)?;
      Ok(())
    }
    (Some("dislike"), "overwrite_dislike_music_infos") => {
      let rules = params.and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
      let mut store = load_data_store()?;
      store.insert("dislike_rules".to_string(), Value::String(rules));
      save_data_store(&store)?;
      Ok(())
    }
    (Some("dislike"), "clear_dislike_music_infos") => {
      let mut store = load_data_store()?;
      store.insert("dislike_rules".to_string(), Value::String(String::new()));
      save_data_store(&store)?;
      Ok(())
    }
    (Some("winMain"), "focus") | (None, "focus") => window.set_focus().map_err(|e| e.to_string()),
    (Some("winMain"), "min") | (None, "min") => window.minimize().map_err(|e| e.to_string()),
    (Some("winMain"), "max") | (None, "max") => window.maximize().map_err(|e| e.to_string()),
    (Some("winMain"), "close") | (None, "close") => window.close().map_err(|e| e.to_string()),
    (Some("winMain"), "min_toggle") | (None, "min_toggle") => {
      let is_minimized = window.is_minimized().map_err(|e| e.to_string())?.unwrap_or(false);
      if is_minimized {
        window.unminimize().map_err(|e| e.to_string())
      } else {
        window.minimize().map_err(|e| e.to_string())
      }
    }
    (Some("winMain"), "hide_toggle") | (None, "hide_toggle") => {
      let visible = window.is_visible().map_err(|e| e.to_string())?;
      if visible {
        window.hide().map_err(|e| e.to_string())
      } else {
        window.show().map_err(|e| e.to_string())
      }
    }
    (Some("winMain"), "open_dev_tools") | (None, "open_dev_tools") => {
      window.open_devtools();
      Ok(())
    }
    (Some("winMain"), "open_dir_in_explorer") | (None, "open_dir_in_explorer") => {
      let p = params.and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
      if p.is_empty() {
        return Ok(());
      }
      let url = format!("file://{p}");
      tauri::api::shell::open(&window.shell_scope(), &url, None).map_err(|e| e.to_string())
    }
    (Some("winMain"), "quit") | (None, "quit") => {
      window.app_handle().exit(0);
      Ok(())
    }
    _ => Ok(()),
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![lx_ipc_invoke, lx_ipc_send])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
