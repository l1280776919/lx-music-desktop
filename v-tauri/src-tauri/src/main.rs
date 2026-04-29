#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
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

fn list_store_path() -> Result<PathBuf, String> {
  Ok(config_root_dir()?.join("my_list.json"))
}

fn theme_store_path() -> Result<PathBuf, String> {
  Ok(config_root_dir()?.join("themes.json"))
}

fn download_store_path() -> Result<PathBuf, String> {
  Ok(config_root_dir()?.join("download_list.json"))
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

fn ensure_object(value: Value) -> Map<String, Value> {
  match value {
    Value::Object(obj) => obj,
    _ => Map::new(),
  }
}

fn ensure_array(value: Value) -> Vec<Value> {
  match value {
    Value::Array(arr) => arr,
    _ => vec![],
  }
}

fn default_lyric_info() -> Value {
  serde_json::json!({
    "lyric": "",
    "tlyric": null,
    "rlyric": null,
    "lxlyric": null
  })
}

#[derive(Clone, Serialize, Deserialize, Default)]
struct ThemeStore {
  #[serde(default)]
  userThemes: Vec<Value>,
}

fn load_theme_store() -> Result<ThemeStore, String> {
  let path = theme_store_path()?;
  if !path.exists() {
    return Ok(ThemeStore::default());
  }
  let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
  serde_json::from_slice::<ThemeStore>(&bytes).map_err(|e| e.to_string())
}

fn save_theme_store(store: &ThemeStore) -> Result<(), String> {
  let root = config_root_dir()?;
  std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
  let path = theme_store_path()?;
  let bytes = serde_json::to_vec_pretty(store).map_err(|e| e.to_string())?;
  std::fs::write(path, bytes).map_err(|e| e.to_string())?;
  Ok(())
}

fn load_download_list() -> Result<Vec<Value>, String> {
  let path = download_store_path()?;
  if !path.exists() {
    return Ok(vec![]);
  }
  let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
  serde_json::from_slice::<Vec<Value>>(&bytes).map_err(|e| e.to_string())
}

fn save_download_list(list: &[Value]) -> Result<(), String> {
  let root = config_root_dir()?;
  std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
  let path = download_store_path()?;
  let bytes = serde_json::to_vec_pretty(list).map_err(|e| e.to_string())?;
  std::fs::write(path, bytes).map_err(|e| e.to_string())?;
  Ok(())
}

#[derive(Clone, Serialize, Deserialize, Default)]
struct ListStore {
  #[serde(default)]
  defaultList: Vec<Value>,
  #[serde(default)]
  loveList: Vec<Value>,
  #[serde(default)]
  tempList: Vec<Value>,
  #[serde(default)]
  userList: Vec<UserListFull>,
}

#[derive(Clone, Serialize, Deserialize)]
struct UserListFull {
  id: String,
  name: String,
  #[serde(default)]
  source: Option<String>,
  #[serde(default)]
  sourceListId: Option<String>,
  #[serde(default)]
  locationUpdateTime: Option<i64>,
  #[serde(default)]
  list: Vec<Value>,
}

fn load_list_store() -> Result<ListStore, String> {
  let path = list_store_path()?;
  if !path.exists() {
    return Ok(ListStore::default());
  }
  let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
  serde_json::from_slice::<ListStore>(&bytes).map_err(|e| e.to_string())
}

fn save_list_store(store: &ListStore) -> Result<(), String> {
  let root = config_root_dir()?;
  std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
  let path = list_store_path()?;
  let bytes = serde_json::to_vec_pretty(store).map_err(|e| e.to_string())?;
  std::fs::write(path, bytes).map_err(|e| e.to_string())?;
  Ok(())
}

fn get_list_mut<'a>(store: &'a mut ListStore, id: &str) -> Option<&'a mut Vec<Value>> {
  match id {
    "default" => Some(&mut store.defaultList),
    "love" => Some(&mut store.loveList),
    "temp" => Some(&mut store.tempList),
    _ => store.userList.iter_mut().find(|l| l.id == id).map(|l| &mut l.list),
  }
}

fn get_list_ref<'a>(store: &'a ListStore, id: &str) -> Option<&'a Vec<Value>> {
  match id {
    "default" => Some(&store.defaultList),
    "love" => Some(&store.loveList),
    "temp" => Some(&store.tempList),
    _ => store.userList.iter().find(|l| l.id == id).map(|l| &l.list),
  }
}

fn music_id(v: &Value) -> Option<String> {
  v.get("id").and_then(|id| id.as_str()).map(|s| s.to_string())
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
async fn lx_ipc_invoke(window: tauri::Window, channel: String, params: Option<Value>) -> Result<Value, String> {
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
          window
            .emit("winMain_on_config_change", Value::Object(patch_obj.clone()))
            .map_err(|e| e.to_string())?;
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
    (Some("winMain"), "get_cache_size") | (None, "get_cache_size") => Ok(Value::from(0)),
    (Some("winMain"), "clear_cache") | (None, "clear_cache") => Ok(Value::Null),
    (Some("winMain"), "set_window_size") | (None, "set_window_size") => Ok(Value::Null),
    (Some("winMain"), "get_music_url") | (None, "get_music_url") => {
      let key = params.and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
      if key.is_empty() {
        return Ok(Value::Null);
      }
      let store = load_data_store()?;
      let map = store.get("music_url").cloned().map(ensure_object).unwrap_or_default();
      Ok(map.get(&key).cloned().unwrap_or(Value::Null))
    }
    (Some("winMain"), "save_music_url") | (None, "save_music_url") => {
      let (id, url) = match params {
        Some(Value::Object(obj)) => {
          let id = obj.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          let url = obj.get("url").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          (id, url)
        }
        _ => (String::new(), String::new()),
      };
      if id.is_empty() || url.is_empty() {
        return Ok(Value::Null);
      }
      let mut store = load_data_store()?;
      let mut map = store.get("music_url").cloned().map(ensure_object).unwrap_or_default();
      map.insert(id, Value::String(url));
      store.insert("music_url".to_string(), Value::Object(map));
      save_data_store(&store)?;
      Ok(Value::Null)
    }
    (Some("winMain"), "clear_music_url") | (None, "clear_music_url") => {
      let mut store = load_data_store()?;
      store.insert("music_url".to_string(), Value::Object(Map::new()));
      save_data_store(&store)?;
      Ok(Value::Null)
    }
    (Some("winMain"), "get_music_url_count") | (None, "get_music_url_count") => {
      let store = load_data_store()?;
      let map = store.get("music_url").cloned().map(ensure_object).unwrap_or_default();
      Ok(Value::from(map.len() as i64))
    }
    (Some("winMain"), "get_lyric_raw") | (None, "get_lyric_raw") => {
      let id = params.and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
      if id.is_empty() {
        return Ok(default_lyric_info());
      }
      let store = load_data_store()?;
      let map = store.get("lyric_raw").cloned().map(ensure_object).unwrap_or_default();
      Ok(map.get(&id).cloned().unwrap_or_else(default_lyric_info))
    }
    (Some("winMain"), "save_lyric_raw") | (None, "save_lyric_raw") => {
      let (id, lyrics) = match params {
        Some(Value::Object(obj)) => {
          let id = obj.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          let lyrics = obj.get("lyrics").cloned().unwrap_or_else(default_lyric_info);
          (id, lyrics)
        }
        _ => (String::new(), default_lyric_info()),
      };
      if id.is_empty() {
        return Ok(Value::Null);
      }
      let mut store = load_data_store()?;
      let mut map = store.get("lyric_raw").cloned().map(ensure_object).unwrap_or_default();
      map.insert(id, lyrics);
      store.insert("lyric_raw".to_string(), Value::Object(map));
      save_data_store(&store)?;
      Ok(Value::Null)
    }
    (Some("winMain"), "clear_lyric_raw") | (None, "clear_lyric_raw") => {
      let mut store = load_data_store()?;
      store.insert("lyric_raw".to_string(), Value::Object(Map::new()));
      save_data_store(&store)?;
      Ok(Value::Null)
    }
    (Some("winMain"), "get_lyric_raw_count") | (None, "get_lyric_raw_count") => {
      let store = load_data_store()?;
      let map = store.get("lyric_raw").cloned().map(ensure_object).unwrap_or_default();
      Ok(Value::from(map.len() as i64))
    }
    (Some("winMain"), "get_lyric_edited") | (None, "get_lyric_edited") => {
      let id = params.and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
      if id.is_empty() {
        return Ok(default_lyric_info());
      }
      let store = load_data_store()?;
      let map = store.get("lyric_edited").cloned().map(ensure_object).unwrap_or_default();
      Ok(map.get(&id).cloned().unwrap_or_else(default_lyric_info))
    }
    (Some("winMain"), "save_lyric_edited") | (None, "save_lyric_edited") => {
      let (id, lyrics) = match params {
        Some(Value::Object(obj)) => {
          let id = obj.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          let lyrics = obj.get("lyrics").cloned().unwrap_or_else(default_lyric_info);
          (id, lyrics)
        }
        _ => (String::new(), default_lyric_info()),
      };
      if id.is_empty() {
        return Ok(Value::Null);
      }
      let mut store = load_data_store()?;
      let mut map = store.get("lyric_edited").cloned().map(ensure_object).unwrap_or_default();
      map.insert(id, lyrics);
      store.insert("lyric_edited".to_string(), Value::Object(map));
      save_data_store(&store)?;
      Ok(Value::Null)
    }
    (Some("winMain"), "remove_lyric_edited") | (None, "remove_lyric_edited") => {
      let id = params.and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
      if id.is_empty() {
        return Ok(Value::Null);
      }
      let mut store = load_data_store()?;
      let mut map = store.get("lyric_edited").cloned().map(ensure_object).unwrap_or_default();
      map.remove(&id);
      store.insert("lyric_edited".to_string(), Value::Object(map));
      save_data_store(&store)?;
      Ok(Value::Null)
    }
    (Some("winMain"), "clear_lyric_edited") | (None, "clear_lyric_edited") => {
      let mut store = load_data_store()?;
      store.insert("lyric_edited".to_string(), Value::Object(Map::new()));
      save_data_store(&store)?;
      Ok(Value::Null)
    }
    (Some("winMain"), "get_lyric_edited_count") | (None, "get_lyric_edited_count") => {
      let store = load_data_store()?;
      let map = store.get("lyric_edited").cloned().map(ensure_object).unwrap_or_default();
      Ok(Value::from(map.len() as i64))
    }
    (Some("winMain"), "get_lyric") | (None, "get_lyric") => {
      let id = params.and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
      if id.is_empty() {
        return Ok(serde_json::json!({
          "lyric": "",
          "tlyric": null,
          "rlyric": null,
          "lxlyric": null,
          "rawlrcInfo": default_lyric_info()
        }));
      }
      let store = load_data_store()?;
      let raw_map = store.get("lyric_raw").cloned().map(ensure_object).unwrap_or_default();
      let edited_map = store.get("lyric_edited").cloned().map(ensure_object).unwrap_or_default();
      let raw = raw_map.get(&id).cloned().unwrap_or_else(default_lyric_info);
      let base = edited_map.get(&id).cloned().unwrap_or_else(|| raw.clone());
      let mut out = ensure_object(base);
      out.insert("rawlrcInfo".to_string(), raw);
      Ok(Value::Object(out))
    }
    (Some("winMain"), "get_sound_effect_eq_preset") | (None, "get_sound_effect_eq_preset") => {
      let store = load_data_store()?;
      let list = store.get("sound_effect_eq_preset").cloned().map(ensure_array).unwrap_or_default();
      Ok(Value::Array(list))
    }
    (Some("winMain"), "get_sound_effect_convolution_preset") | (None, "get_sound_effect_convolution_preset") => {
      let store = load_data_store()?;
      let list = store
        .get("sound_effect_convolution_preset")
        .cloned()
        .map(ensure_array)
        .unwrap_or_default();
      Ok(Value::Array(list))
    }
    (Some("winMain"), "download_list_get") | (None, "download_list_get") => Ok(Value::Array(load_download_list()?)),
    (Some("winMain"), "download_list_add") | (None, "download_list_add") => {
      let (list, add_type) = match params {
        Some(Value::Object(obj)) => {
          let list = obj.get("list").and_then(|v| v.as_array()).cloned().unwrap_or_default();
          let add_type = obj.get("addMusicLocationType").and_then(|v| v.as_str()).unwrap_or("bottom").to_string();
          (list, add_type)
        }
        _ => (vec![], "bottom".to_string()),
      };
      if list.is_empty() {
        return Ok(Value::Null);
      }
      let mut stored = load_download_list()?;
      if add_type == "top" {
        let mut out = list;
        out.extend(stored.drain(..));
        stored = out;
      } else {
        stored.extend(list);
      }
      save_download_list(&stored)?;
      Ok(Value::Null)
    }
    (Some("winMain"), "download_list_update") | (None, "download_list_update") => {
      let updates = params.and_then(|v| v.as_array().cloned()).unwrap_or_default();
      if updates.is_empty() {
        return Ok(Value::Null);
      }
      let mut stored = load_download_list()?;
      let mut map: std::collections::HashMap<String, Value> = std::collections::HashMap::new();
      for item in updates {
        if let Some(id) = item.get("id").and_then(|v| v.as_str()) {
          map.insert(id.to_string(), item);
        }
      }
      if map.is_empty() {
        return Ok(Value::Null);
      }
      for item in stored.iter_mut() {
        if let Some(id) = item.get("id").and_then(|v| v.as_str()) {
          if let Some(new_item) = map.remove(id) {
            *item = new_item;
          }
        }
      }
      for (_, item) in map {
        stored.push(item);
      }
      save_download_list(&stored)?;
      Ok(Value::Null)
    }
    (Some("winMain"), "download_list_remove") | (None, "download_list_remove") => {
      let ids = params.and_then(|v| v.as_array().cloned()).unwrap_or_default();
      if ids.is_empty() {
        return Ok(Value::Null);
      }
      let id_set = ids
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect::<std::collections::HashSet<String>>();
      if id_set.is_empty() {
        return Ok(Value::Null);
      }
      let mut stored = load_download_list()?;
      stored.retain(|item| item.get("id").and_then(|v| v.as_str()).map(|id| !id_set.contains(id)).unwrap_or(true));
      save_download_list(&stored)?;
      Ok(Value::Null)
    }
    (Some("winMain"), "download_list_clear") | (None, "download_list_clear") => {
      save_download_list(&[])?;
      Ok(Value::Null)
    }

    (Some("winMain"), "show_select_dialog") | (None, "show_select_dialog") => {
      use tauri::api::dialog::blocking::FileDialogBuilder;
      let opts = params.unwrap_or(Value::Null);
      let properties = opts.get("properties").and_then(|v| v.as_array()).cloned().unwrap_or_default();
      let mut is_dir = false;
      let mut multi = false;
      for p in &properties {
        if let Some(s) = p.as_str() {
          if s == "openDirectory" {
            is_dir = true;
          } else if s == "multiSelections" {
            multi = true;
          }
        }
      }
      let mut builder = FileDialogBuilder::new();
      if let Some(title) = opts.get("title").and_then(|v| v.as_str()) {
        builder = builder.set_title(title);
      }
      if let Some(default_path) = opts.get("defaultPath").and_then(|v| v.as_str()) {
        if !default_path.is_empty() {
          builder = builder.set_directory(default_path);
        }
      }
      if let Some(filters) = opts.get("filters").and_then(|v| v.as_array()) {
        for filter in filters {
          let name = filter.get("name").and_then(|v| v.as_str()).unwrap_or("Files");
          let exts = filter.get("extensions").and_then(|v| v.as_array()).cloned().unwrap_or_default();
          let ext_owned: Vec<String> = exts.into_iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect();
          let ext_refs: Vec<&str> = ext_owned.iter().map(|s| s.as_str()).collect();
          if !ext_refs.is_empty() {
            builder = builder.add_filter(name, &ext_refs);
          }
        }
      }
      if is_dir {
        if multi {
          let paths = builder.pick_folders().unwrap_or_default();
          Ok(serde_json::json!({ "canceled": paths.is_empty(), "filePaths": paths.iter().map(|p| p.to_string_lossy().to_string()).collect::<Vec<String>>() }))
        } else {
          let path = builder.pick_folder();
          Ok(serde_json::json!({ "canceled": path.is_none(), "filePaths": path.map(|p| vec![p.to_string_lossy().to_string()]).unwrap_or_default() }))
        }
      } else if multi {
        let paths = builder.pick_files().unwrap_or_default();
        Ok(serde_json::json!({ "canceled": paths.is_empty(), "filePaths": paths.iter().map(|p| p.to_string_lossy().to_string()).collect::<Vec<String>>() }))
      } else {
        let path = builder.pick_file();
        Ok(serde_json::json!({ "canceled": path.is_none(), "filePaths": path.map(|p| vec![p.to_string_lossy().to_string()]).unwrap_or_default() }))
      }
    }
    (Some("winMain"), "show_save_dialog") | (None, "show_save_dialog") => {
      use tauri::api::dialog::blocking::FileDialogBuilder;
      let opts = params.unwrap_or(Value::Null);
      let mut builder = FileDialogBuilder::new();
      if let Some(title) = opts.get("title").and_then(|v| v.as_str()) {
        builder = builder.set_title(title);
      }
      if let Some(default_path) = opts.get("defaultPath").and_then(|v| v.as_str()) {
        if !default_path.is_empty() {
          builder = builder.set_file_name(default_path);
        }
      }
      let path = builder.save_file();
      Ok(serde_json::json!({ "canceled": path.is_none(), "filePath": path.map(|p| p.to_string_lossy().to_string()) }))
    }
    (Some("winMain"), "show_dialog") | (None, "show_dialog") => Ok(Value::Null),
    (Some("winMain"), "save_theme") | (None, "save_theme") => {
      let theme = params.unwrap_or(Value::Null);
      let id = theme.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
      if id.is_empty() {
        return Ok(Value::Null);
      }
      let mut store = load_theme_store()?;
      if let Some(pos) = store.userThemes.iter().position(|t| t.get("id").and_then(|v| v.as_str()) == Some(id.as_str())) {
        store.userThemes[pos] = theme.clone();
      } else {
        store.userThemes.push(theme.clone());
      }
      save_theme_store(&store)?;
      Ok(theme)
    }
    (Some("winMain"), "remove_theme") | (None, "remove_theme") => {
      let id = params.and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
      if id.is_empty() {
        return Ok(Value::Null);
      }
      let mut store = load_theme_store()?;
      store
        .userThemes
        .retain(|t| t.get("id").and_then(|v| v.as_str()) != Some(id.as_str()));
      save_theme_store(&store)?;
      Ok(Value::String(id))
    }
    (Some("winMain"), "get_themes") | (None, "get_themes") => {
      let store = load_theme_store()?;
      Ok(serde_json::json!({
        "themes": [],
        "userThemes": store.userThemes,
        "dataPath": config_root_dir()?.to_string_lossy().to_string()
      }))
    }
    (Some("winMain"), "fullscreen") | (None, "fullscreen") => {
      let enable = params.and_then(|v| v.as_bool()).unwrap_or(false);
      window.set_fullscreen(enable).map_err(|e| e.to_string())?;
      Ok(Value::from(enable))
    }

    (Some("player"), "list_get") => {
      let store = load_list_store()?;
      let list = store
        .userList
        .into_iter()
        .map(|l| {
          serde_json::json!({
            "id": l.id,
            "name": l.name,
            "source": l.source,
            "sourceListId": l.sourceListId,
            "locationUpdateTime": l.locationUpdateTime
          })
        })
        .collect::<Vec<Value>>();
      Ok(Value::Array(list))
    }
    (Some("player"), "list_add") => {
      let mut store = load_list_store()?;
      let (position, list_infos) = match params {
        Some(Value::Object(obj)) => {
          let position = obj.get("position").and_then(|v| v.as_i64()).unwrap_or(0).max(0) as usize;
          let list_infos = obj.get("listInfos").and_then(|v| v.as_array()).cloned().unwrap_or_default();
          (position, list_infos)
        }
        _ => (0, vec![]),
      };
      let mut new_infos: Vec<Value> = Vec::new();
      let mut insert_items: Vec<UserListFull> = Vec::new();
      for info in list_infos {
        let id = info.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        if id.is_empty() {
          continue;
        }
        let name = info.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let source = info.get("source").and_then(|v| v.as_str()).map(|s| s.to_string());
        let source_list_id = info.get("sourceListId").and_then(|v| v.as_str()).map(|s| s.to_string());
        let location_update_time = info.get("locationUpdateTime").and_then(|v| v.as_i64());
        insert_items.push(UserListFull {
          id: id.clone(),
          name: name.clone(),
          source,
          sourceListId: source_list_id,
          locationUpdateTime: location_update_time,
          list: vec![],
        });
        new_infos.push(serde_json::json!({
          "id": id,
          "name": name,
          "locationUpdateTime": location_update_time
        }));
      }
      let pos = position.min(store.userList.len());
      store.userList.splice(pos..pos, insert_items);
      save_list_store(&store)?;
      let payload = serde_json::json!({ "position": pos, "listInfos": new_infos });
      window.emit("player_list_add", payload).map_err(|e| e.to_string())?;
      Ok(Value::Null)
    }
    (Some("player"), "list_remove") => {
      let ids = params.and_then(|v| v.as_array().cloned()).unwrap_or_default();
      let id_set = ids
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect::<std::collections::HashSet<String>>();
      if id_set.is_empty() {
        return Ok(Value::Null);
      }
      let mut store = load_list_store()?;
      store.userList.retain(|l| !id_set.contains(&l.id));
      save_list_store(&store)?;
      window.emit("player_list_remove", Value::Array(ids)).map_err(|e| e.to_string())?;
      Ok(Value::Null)
    }
    (Some("player"), "list_update") => {
      let list_infos = params.and_then(|v| v.as_array().cloned()).unwrap_or_default();
      if list_infos.is_empty() {
        return Ok(Value::Null);
      }
      let mut store = load_list_store()?;
      for info in &list_infos {
        let id = info.get("id").and_then(|v| v.as_str()).unwrap_or_default();
        if id.is_empty() {
          continue;
        }
        if let Some(target) = store.userList.iter_mut().find(|l| l.id == id) {
          if let Some(name) = info.get("name").and_then(|v| v.as_str()) {
            target.name = name.to_string();
          }
          if let Some(t) = info.get("locationUpdateTime").and_then(|v| v.as_i64()) {
            target.locationUpdateTime = Some(t);
          }
        }
      }
      save_list_store(&store)?;
      window.emit("player_list_update", Value::Array(list_infos)).map_err(|e| e.to_string())?;
      Ok(Value::Null)
    }
    (Some("player"), "list_update_position") => {
      let (position, ids) = match params {
        Some(Value::Object(obj)) => {
          let position = obj.get("position").and_then(|v| v.as_i64()).unwrap_or(0).max(0) as usize;
          let ids = obj.get("ids").and_then(|v| v.as_array()).cloned().unwrap_or_default();
          (position, ids)
        }
        _ => (0, vec![]),
      };
      let id_list = ids
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect::<Vec<String>>();
      if id_list.is_empty() {
        return Ok(Value::Null);
      }
      let mut store = load_list_store()?;
      let mut moved: Vec<UserListFull> = Vec::new();
      store.userList.retain(|l| {
        if id_list.contains(&l.id) {
          moved.push(l.clone());
          false
        } else {
          true
        }
      });
      let pos = position.min(store.userList.len());
      store.userList.splice(pos..pos, moved);
      save_list_store(&store)?;
      let payload = serde_json::json!({ "position": pos, "ids": ids });
      window.emit("player_list_update_position", payload).map_err(|e| e.to_string())?;
      Ok(Value::Null)
    }
    (Some("player"), "list_music_get") => {
      let list_id = params.and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
      if list_id.is_empty() {
        return Ok(Value::Array(vec![]));
      }
      let store = load_list_store()?;
      Ok(Value::Array(get_list_ref(&store, &list_id).cloned().unwrap_or_default()))
    }
    (Some("player"), "list_music_add") => {
      let (id, music_infos, location) = match params {
        Some(Value::Object(obj)) => {
          let id = obj.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          let music_infos = obj.get("musicInfos").and_then(|v| v.as_array()).cloned().unwrap_or_default();
          let location = obj.get("addMusicLocationType").and_then(|v| v.as_str()).unwrap_or("bottom").to_string();
          (id, music_infos, location)
        }
        _ => (String::new(), vec![], "bottom".to_string()),
      };
      if id.is_empty() || music_infos.is_empty() {
        return Ok(Value::Null);
      }
      let mut store = load_list_store()?;
      let list = match get_list_mut(&mut store, &id) {
        Some(l) => l,
        None => return Ok(Value::Null),
      };
      if location == "top" {
        let mut out = music_infos.clone();
        out.extend(list.drain(..));
        *list = out;
      } else {
        list.extend(music_infos.clone());
      }
      save_list_store(&store)?;
      let payload = serde_json::json!({ "id": id, "musicInfos": music_infos, "addMusicLocationType": location });
      window.emit("player_list_music_add", payload).map_err(|e| e.to_string())?;
      Ok(Value::Null)
    }
    (Some("player"), "list_music_move") => {
      let (from_id, to_id, music_infos, location) = match params {
        Some(Value::Object(obj)) => {
          let from_id = obj.get("fromId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          let to_id = obj.get("toId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          let music_infos = obj.get("musicInfos").and_then(|v| v.as_array()).cloned().unwrap_or_default();
          let location = obj.get("addMusicLocationType").and_then(|v| v.as_str()).unwrap_or("bottom").to_string();
          (from_id, to_id, music_infos, location)
        }
        _ => (String::new(), String::new(), vec![], "bottom".to_string()),
      };
      if from_id.is_empty() || to_id.is_empty() || music_infos.is_empty() {
        return Ok(Value::Null);
      }
      let move_ids = music_infos
        .iter()
        .filter_map(music_id)
        .collect::<std::collections::HashSet<String>>();
      let mut store = load_list_store()?;
      if let Some(from_list) = get_list_mut(&mut store, &from_id) {
        from_list.retain(|m| music_id(m).map(|id| !move_ids.contains(&id)).unwrap_or(true));
      }
      if let Some(to_list) = get_list_mut(&mut store, &to_id) {
        if location == "top" {
          let mut out = music_infos.clone();
          out.extend(to_list.drain(..));
          *to_list = out;
        } else {
          to_list.extend(music_infos.clone());
        }
      }
      save_list_store(&store)?;
      let payload = serde_json::json!({ "fromId": from_id, "toId": to_id, "musicInfos": music_infos, "addMusicLocationType": location });
      window.emit("player_list_music_move", payload).map_err(|e| e.to_string())?;
      Ok(Value::Null)
    }
    (Some("player"), "list_music_remove") => {
      let (list_id, ids) = match params {
        Some(Value::Object(obj)) => {
          let list_id = obj.get("listId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          let ids = obj.get("ids").and_then(|v| v.as_array()).cloned().unwrap_or_default();
          (list_id, ids)
        }
        _ => (String::new(), vec![]),
      };
      if list_id.is_empty() || ids.is_empty() {
        return Ok(Value::Null);
      }
      let id_set = ids
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect::<std::collections::HashSet<String>>();
      let mut store = load_list_store()?;
      if let Some(list) = get_list_mut(&mut store, &list_id) {
        list.retain(|m| music_id(m).map(|id| !id_set.contains(&id)).unwrap_or(true));
      }
      save_list_store(&store)?;
      let payload = serde_json::json!({ "listId": list_id, "ids": ids });
      window.emit("player_list_music_remove", payload).map_err(|e| e.to_string())?;
      Ok(Value::Null)
    }
    (Some("player"), "list_music_overwrite") => {
      let (list_id, music_infos) = match params {
        Some(Value::Object(obj)) => {
          let list_id = obj.get("listId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          let music_infos = obj.get("musicInfos").and_then(|v| v.as_array()).cloned().unwrap_or_default();
          (list_id, music_infos)
        }
        _ => (String::new(), vec![]),
      };
      if list_id.is_empty() {
        return Ok(Value::Null);
      }
      let mut store = load_list_store()?;
      if let Some(list) = get_list_mut(&mut store, &list_id) {
        *list = music_infos.clone();
      }
      save_list_store(&store)?;
      let payload = serde_json::json!({ "listId": list_id, "musicInfos": music_infos });
      window.emit("player_list_music_overwrite", payload).map_err(|e| e.to_string())?;
      Ok(Value::Null)
    }
    (Some("player"), "list_music_clear") => {
      let list_ids = params.and_then(|v| v.as_array().cloned()).unwrap_or_default();
      if list_ids.is_empty() {
        return Ok(Value::Null);
      }
      let mut store = load_list_store()?;
      for id in &list_ids {
        let id = id.as_str().unwrap_or_default();
        if let Some(list) = get_list_mut(&mut store, id) {
          list.clear();
        }
      }
      save_list_store(&store)?;
      window.emit("player_list_music_clear", Value::Array(list_ids)).map_err(|e| e.to_string())?;
      Ok(Value::Null)
    }
    (Some("player"), "list_data_overwire") => {
      let data = params.unwrap_or(Value::Null);
      let mut store = load_list_store()?;
      if let Some(d) = data.get("defaultList").and_then(|v| v.as_array()) {
        store.defaultList = d.to_vec();
      }
      if let Some(d) = data.get("loveList").and_then(|v| v.as_array()) {
        store.loveList = d.to_vec();
      }
      if let Some(d) = data.get("tempList").and_then(|v| v.as_array()) {
        store.tempList = d.to_vec();
      }
      if let Some(u) = data.get("userList").and_then(|v| v.as_array()) {
        let mut user_list: Vec<UserListFull> = Vec::new();
        for item in u {
          let id = item.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          if id.is_empty() {
            continue;
          }
          let name = item.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          let source = item.get("source").and_then(|v| v.as_str()).map(|s| s.to_string());
          let source_list_id = item.get("sourceListId").and_then(|v| v.as_str()).map(|s| s.to_string());
          let location_update_time = item.get("locationUpdateTime").and_then(|v| v.as_i64());
          let list = item.get("list").and_then(|v| v.as_array()).cloned().unwrap_or_default();
          user_list.push(UserListFull {
            id,
            name,
            source,
            sourceListId: source_list_id,
            locationUpdateTime: location_update_time,
            list,
          });
        }
        store.userList = user_list;
      }
      save_list_store(&store)?;
      window.emit("player_list_data_overwire", data).map_err(|e| e.to_string())?;
      Ok(Value::Null)
    }
    (Some("player"), "list_music_check_exist") => {
      let (list_id, music_info_id) = match params {
        Some(Value::Object(obj)) => {
          let list_id = obj.get("listId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          let music_info_id = obj.get("musicInfoId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
          (list_id, music_info_id)
        }
        _ => (String::new(), String::new()),
      };
      if list_id.is_empty() || music_info_id.is_empty() {
        return Ok(Value::from(false));
      }
      let store = load_list_store()?;
      let ok = get_list_ref(&store, &list_id)
        .map(|list| list.iter().any(|m| music_id(m).map(|id| id == music_info_id).unwrap_or(false)))
        .unwrap_or(false);
      Ok(Value::from(ok))
    }
    (Some("player"), "list_music_get_list_ids") => {
      let music_info_id = params.and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
      if music_info_id.is_empty() {
        return Ok(Value::Array(vec![]));
      }
      let store = load_list_store()?;
      let mut ids: Vec<Value> = Vec::new();
      for (id, list) in [
        ("default".to_string(), &store.defaultList),
        ("love".to_string(), &store.loveList),
        ("temp".to_string(), &store.tempList),
      ] {
        if list.iter().any(|m| music_id(m).map(|mid| mid == music_info_id).unwrap_or(false)) {
          ids.push(Value::String(id));
        }
      }
      for l in &store.userList {
        if l.list.iter().any(|m| music_id(m).map(|mid| mid == music_info_id).unwrap_or(false)) {
          ids.push(Value::String(l.id.clone()));
        }
      }
      Ok(Value::Array(ids))
    }

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
    (Some("dislike"), "add_dislike_music_infos") => {
      let infos = params.and_then(|v| v.as_array().cloned()).unwrap_or_default();
      if infos.is_empty() {
        return Ok(Value::Null);
      }
      let mut store = load_data_store()?;
      let mut rules = store.get("dislike_rules").and_then(|v| v.as_str()).unwrap_or("").to_string();
      for item in &infos {
        let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let singer = item.get("singer").and_then(|v| v.as_str()).unwrap_or("");
        rules.push('\n');
        rules.push_str(name);
        rules.push('@');
        rules.push_str(singer);
      }
      store.insert("dislike_rules".to_string(), Value::String(rules));
      save_data_store(&store)?;
      window.emit("dislike_add_dislike_music_infos", Value::Array(infos)).map_err(|e| e.to_string())?;
      Ok(Value::Null)
    }
    (Some("dislike"), "overwrite_dislike_music_infos") => {
      let rules = params.and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
      let mut store = load_data_store()?;
      store.insert("dislike_rules".to_string(), Value::String(rules.clone()));
      save_data_store(&store)?;
      window
        .emit("dislike_overwrite_dislike_music_infos", Value::String(rules))
        .map_err(|e| e.to_string())?;
      Ok(Value::Null)
    }
    (Some("dislike"), "clear_dislike_music_infos") => {
      let mut store = load_data_store()?;
      store.insert("dislike_rules".to_string(), Value::String(String::new()));
      save_data_store(&store)?;
      window.emit("dislike_clear_dislike_music_infos", Value::Null).map_err(|e| e.to_string())?;
      Ok(Value::Null)
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
    (Some("winMain"), "save_sound_effect_eq_preset") | (None, "save_sound_effect_eq_preset") => {
      let list = params.clone().map(ensure_array).unwrap_or_default();
      let mut store = load_data_store()?;
      store.insert("sound_effect_eq_preset".to_string(), Value::Array(list));
      save_data_store(&store)?;
      Ok(())
    }
    (Some("winMain"), "save_sound_effect_convolution_preset") | (None, "save_sound_effect_convolution_preset") => {
      let list = params.clone().map(ensure_array).unwrap_or_default();
      let mut store = load_data_store()?;
      store.insert("sound_effect_convolution_preset".to_string(), Value::Array(list));
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
