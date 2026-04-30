use rusqlite::{params, Connection, OptionalExtension};
use serde_json::{Map, Value};
use std::{collections::HashSet, fs, path::PathBuf};
use tauri::{AppHandle, Manager};

const STORE_DB_NAME: &str = "lx.sqlite3";
const LEGACY_STORE_TABLE: &str = "app_store";
const STORE_REGISTRY_TABLE: &str = "store_registry";
const APP_SETTINGS_TABLE: &str = "app_settings_entries";
const APP_DATA_TABLE: &str = "app_data_entries";
const SYNC_TABLE: &str = "sync_entries";
const HOTKEY_TABLE: &str = "hotkey_entries";
const USER_API_TABLE: &str = "user_api_entries";
const THEME_TABLE: &str = "theme_entries";
const SOUND_EFFECT_TABLE: &str = "sound_effect_entries";
const LYRIC_RAW_TABLE: &str = "lyric_raw_entries";
const LYRIC_EDITED_TABLE: &str = "lyric_edited_entries";
const MUSIC_URL_TABLE: &str = "music_url_entries";
const DOWNLOAD_TASK_TABLE: &str = "download_task_entries";
const LIST_META_TABLE: &str = "list_meta_entries";
const LIST_MUSIC_TABLE: &str = "list_music_entries";
const DISLIKE_RULE_TABLE: &str = "dislike_rule_entries";
const MISC_STORE_TABLE: &str = "misc_store_values";

const STORE_APP_SETTINGS: &str = "config_v2";
const STORE_DATA: &str = "data";
const STORE_SYNC: &str = "sync";
const STORE_HOTKEY: &str = "hot_key";
const STORE_USER_API: &str = "user_api";
const STORE_LRC_RAW: &str = "lyrics";
const STORE_LRC_EDITED: &str = "lyrics_edited";
const STORE_THEME: &str = "theme";
const STORE_SOUND_EFFECT: &str = "sound_effect";
const STORE_MUSIC_URL: &str = "tauri_music_url_cache";
const STORE_DOWNLOAD_TASK: &str = "tauri_download_task_store";
const STORE_LIST_DATA: &str = "tauri_list_data";
const STORE_DISLIKE: &str = "tauri_dislike_data";

enum StoreKind {
    AppSettings,
    AppData,
    Sync,
    Hotkey,
    UserApi,
    Theme,
    SoundEffect,
    LyricRaw,
    LyricEdited,
    MusicUrl,
    DownloadTask,
    ListData,
    Dislike,
    Misc,
}

impl StoreKind {
    fn from_name(name: &str) -> Self {
        match name {
            STORE_APP_SETTINGS => Self::AppSettings,
            STORE_DATA => Self::AppData,
            STORE_SYNC => Self::Sync,
            STORE_HOTKEY => Self::Hotkey,
            STORE_USER_API => Self::UserApi,
            STORE_THEME => Self::Theme,
            STORE_SOUND_EFFECT => Self::SoundEffect,
            STORE_LRC_RAW => Self::LyricRaw,
            STORE_LRC_EDITED => Self::LyricEdited,
            STORE_MUSIC_URL => Self::MusicUrl,
            STORE_DOWNLOAD_TASK => Self::DownloadTask,
            STORE_LIST_DATA => Self::ListData,
            STORE_DISLIKE => Self::Dislike,
            _ => Self::Misc,
        }
    }
}

pub fn get_store(app: &AppHandle, name: &str) -> Result<Option<Value>, String> {
    let conn = open_store_db(app)?;
    let kind = StoreKind::from_name(name);
    if is_store_registered(&conn, name)? {
        return load_store_value(&conn, name, &kind).map(Some);
    }
    if let Some(legacy_value) = load_legacy_store(app, &conn, name)? {
        save_store_value(&conn, name, &kind, &legacy_value)?;
        return Ok(Some(legacy_value));
    }
    Ok(None)
}

pub fn set_store(app: &AppHandle, name: &str, value: &Value) -> Result<(), String> {
    let conn = open_store_db(app)?;
    let kind = StoreKind::from_name(name);
    save_store_value(&conn, name, &kind, value)
}

fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|err| err.to_string())?;
    fs::create_dir_all(&dir).map_err(|err| err.to_string())?;
    Ok(dir)
}

fn store_file_path(app: &AppHandle, name: &str) -> Result<PathBuf, String> {
    let dir = app_data_dir(app)?;
    Ok(dir.join(format!("{name}.json")))
}

fn store_db_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app_data_dir(app)?;
    Ok(dir.join(STORE_DB_NAME))
}

fn open_store_db(app: &AppHandle) -> Result<Connection, String> {
    let db_path = store_db_path(app)?;
    let conn = Connection::open(db_path).map_err(|err| err.to_string())?;
    conn.execute_batch(&format!(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;

        CREATE TABLE IF NOT EXISTS {STORE_REGISTRY_TABLE} (
            name TEXT PRIMARY KEY NOT NULL,
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        CREATE TABLE IF NOT EXISTS {APP_SETTINGS_TABLE} (
            entry_key TEXT PRIMARY KEY NOT NULL,
            value_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS {APP_DATA_TABLE} (
            entry_key TEXT PRIMARY KEY NOT NULL,
            value_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS {SYNC_TABLE} (
            entry_key TEXT PRIMARY KEY NOT NULL,
            value_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS {HOTKEY_TABLE} (
            entry_key TEXT PRIMARY KEY NOT NULL,
            value_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS {USER_API_TABLE} (
            entry_id TEXT PRIMARY KEY NOT NULL,
            sort_index INTEGER NOT NULL,
            value_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS {THEME_TABLE} (
            entry_id TEXT PRIMARY KEY NOT NULL,
            sort_index INTEGER NOT NULL,
            value_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS {SOUND_EFFECT_TABLE} (
            kind TEXT NOT NULL,
            sort_index INTEGER NOT NULL,
            value_json TEXT NOT NULL,
            PRIMARY KEY(kind, sort_index)
        );

        CREATE TABLE IF NOT EXISTS {LYRIC_RAW_TABLE} (
            music_id TEXT PRIMARY KEY NOT NULL,
            value_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS {LYRIC_EDITED_TABLE} (
            music_id TEXT PRIMARY KEY NOT NULL,
            value_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS {MUSIC_URL_TABLE} (
            cache_key TEXT PRIMARY KEY NOT NULL,
            url TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS {DOWNLOAD_TASK_TABLE} (
            task_id TEXT PRIMARY KEY NOT NULL,
            sort_index INTEGER NOT NULL,
            value_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS {LIST_META_TABLE} (
            list_id TEXT PRIMARY KEY NOT NULL,
            list_type TEXT NOT NULL,
            sort_index INTEGER NOT NULL,
            value_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS {LIST_MUSIC_TABLE} (
            list_id TEXT NOT NULL,
            sort_index INTEGER NOT NULL,
            music_id TEXT NOT NULL,
            value_json TEXT NOT NULL,
            PRIMARY KEY(list_id, sort_index)
        );

        CREATE TABLE IF NOT EXISTS {DISLIKE_RULE_TABLE} (
            sort_index INTEGER PRIMARY KEY NOT NULL,
            rule_text TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS {MISC_STORE_TABLE} (
            name TEXT PRIMARY KEY NOT NULL,
            value_json TEXT NOT NULL
        );
        "
    ))
    .map_err(|err| err.to_string())?;
    ensure_list_music_table_schema(&conn)?;
    Ok(conn)
}

fn ensure_list_music_table_schema(conn: &Connection) -> Result<(), String> {
    let mut stmt = conn
        .prepare(&format!("PRAGMA table_info({LIST_MUSIC_TABLE})"))
        .map_err(|err| err.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(1)?,
                row.get::<_, i64>(5)?,
            ))
        })
        .map_err(|err| err.to_string())?;
    let mut primary_key_columns = Vec::new();
    for row in rows {
        let (name, pk) = row.map_err(|err| err.to_string())?;
        if pk > 0 {
            primary_key_columns.push((pk, name));
        }
    }
    primary_key_columns.sort_by_key(|(pk, _)| *pk);
    let current: Vec<String> = primary_key_columns
        .into_iter()
        .map(|(_, name)| name)
        .collect();
    let expected = vec!["list_id".to_string(), "sort_index".to_string()];
    if current == expected {
        return Ok(());
    }

    conn.execute_batch(&format!(
        "
        ALTER TABLE {LIST_MUSIC_TABLE} RENAME TO {LIST_MUSIC_TABLE}_legacy;
        CREATE TABLE {LIST_MUSIC_TABLE} (
            list_id TEXT NOT NULL,
            sort_index INTEGER NOT NULL,
            music_id TEXT NOT NULL,
            value_json TEXT NOT NULL,
            PRIMARY KEY(list_id, sort_index)
        );
        INSERT INTO {LIST_MUSIC_TABLE}(list_id, sort_index, music_id, value_json)
        SELECT list_id, sort_index, music_id, value_json
        FROM {LIST_MUSIC_TABLE}_legacy
        ORDER BY list_id, sort_index;
        DROP TABLE {LIST_MUSIC_TABLE}_legacy;
        "
    ))
    .map_err(|err| err.to_string())
}

fn is_store_registered(conn: &Connection, name: &str) -> Result<bool, String> {
    conn.query_row(
        &format!(
            "SELECT 1 FROM {STORE_REGISTRY_TABLE} WHERE name = ?1 LIMIT 1"
        ),
        params![name],
        |_row| Ok(()),
    )
    .optional()
    .map(|value| value.is_some())
    .map_err(|err| err.to_string())
}

fn mark_store_registered(conn: &Connection, name: &str) -> Result<(), String> {
    conn.execute(
        &format!(
            "
            INSERT INTO {STORE_REGISTRY_TABLE}(name, updated_at)
            VALUES(?1, strftime('%s', 'now'))
            ON CONFLICT(name) DO UPDATE SET updated_at = excluded.updated_at
            "
        ),
        params![name],
    )
    .map(|_| ())
    .map_err(|err| err.to_string())
}

fn load_legacy_store(
    app: &AppHandle,
    conn: &Connection,
    name: &str,
) -> Result<Option<Value>, String> {
    if has_table(conn, LEGACY_STORE_TABLE)? {
        let value = conn
            .query_row(
                &format!("SELECT value FROM {LEGACY_STORE_TABLE} WHERE name = ?1 LIMIT 1"),
                params![name],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|err| err.to_string())?;
        if let Some(text) = value {
            return serde_json::from_str(&text)
                .map(Some)
                .map_err(|err| err.to_string());
        }
    }

    let path = store_file_path(app, name)?;
    if !path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(path).map_err(|err| err.to_string())?;
    serde_json::from_str(&text)
        .map(Some)
        .map_err(|err| err.to_string())
}

fn has_table(conn: &Connection, table_name: &str) -> Result<bool, String> {
    conn.query_row(
        "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1 LIMIT 1",
        params![table_name],
        |_row| Ok(()),
    )
    .optional()
    .map(|value| value.is_some())
    .map_err(|err| err.to_string())
}

fn load_store_value(conn: &Connection, name: &str, kind: &StoreKind) -> Result<Value, String> {
    match kind {
        StoreKind::AppSettings => load_json_object_table(conn, APP_SETTINGS_TABLE, "entry_key"),
        StoreKind::AppData => load_json_object_table(conn, APP_DATA_TABLE, "entry_key"),
        StoreKind::Sync => load_json_object_table(conn, SYNC_TABLE, "entry_key"),
        StoreKind::Hotkey => load_json_object_table(conn, HOTKEY_TABLE, "entry_key"),
        StoreKind::UserApi => load_ordered_array_table(conn, USER_API_TABLE, "entry_id"),
        StoreKind::Theme => load_ordered_array_table(conn, THEME_TABLE, "entry_id"),
        StoreKind::SoundEffect => load_sound_effect_table(conn),
        StoreKind::LyricRaw => load_json_object_table(conn, LYRIC_RAW_TABLE, "music_id"),
        StoreKind::LyricEdited => load_json_object_table(conn, LYRIC_EDITED_TABLE, "music_id"),
        StoreKind::MusicUrl => load_string_map_table(conn, MUSIC_URL_TABLE),
        StoreKind::DownloadTask => load_ordered_array_table(conn, DOWNLOAD_TASK_TABLE, "task_id"),
        StoreKind::ListData => load_list_store(conn),
        StoreKind::Dislike => load_dislike_store(conn),
        StoreKind::Misc => load_misc_store(conn, name),
    }
}

fn save_store_value(
    conn: &Connection,
    name: &str,
    kind: &StoreKind,
    value: &Value,
) -> Result<(), String> {
    clear_store_value(conn, name, kind)?;
    match kind {
        StoreKind::AppSettings => {
            save_json_object_table(conn, APP_SETTINGS_TABLE, "entry_key", value)?
        }
        StoreKind::AppData => save_json_object_table(conn, APP_DATA_TABLE, "entry_key", value)?,
        StoreKind::Sync => save_json_object_table(conn, SYNC_TABLE, "entry_key", value)?,
        StoreKind::Hotkey => save_json_object_table(conn, HOTKEY_TABLE, "entry_key", value)?,
        StoreKind::UserApi => save_ordered_array_table(conn, USER_API_TABLE, "entry_id", value)?,
        StoreKind::Theme => save_ordered_array_table(conn, THEME_TABLE, "entry_id", value)?,
        StoreKind::SoundEffect => save_sound_effect_table(conn, value)?,
        StoreKind::LyricRaw => save_json_object_table(conn, LYRIC_RAW_TABLE, "music_id", value)?,
        StoreKind::LyricEdited => {
            save_json_object_table(conn, LYRIC_EDITED_TABLE, "music_id", value)?
        }
        StoreKind::MusicUrl => save_string_map_table(conn, MUSIC_URL_TABLE, value)?,
        StoreKind::DownloadTask => {
            save_ordered_array_table(conn, DOWNLOAD_TASK_TABLE, "task_id", value)?
        }
        StoreKind::ListData => save_list_store(conn, value)?,
        StoreKind::Dislike => save_dislike_store(conn, value)?,
        StoreKind::Misc => save_misc_store(conn, name, value)?,
    }
    mark_store_registered(conn, name)
}

fn clear_store_value(conn: &Connection, name: &str, kind: &StoreKind) -> Result<(), String> {
    match kind {
        StoreKind::AppSettings => clear_table(conn, APP_SETTINGS_TABLE),
        StoreKind::AppData => clear_table(conn, APP_DATA_TABLE),
        StoreKind::Sync => clear_table(conn, SYNC_TABLE),
        StoreKind::Hotkey => clear_table(conn, HOTKEY_TABLE),
        StoreKind::UserApi => clear_table(conn, USER_API_TABLE),
        StoreKind::Theme => clear_table(conn, THEME_TABLE),
        StoreKind::SoundEffect => clear_table(conn, SOUND_EFFECT_TABLE),
        StoreKind::LyricRaw => clear_table(conn, LYRIC_RAW_TABLE),
        StoreKind::LyricEdited => clear_table(conn, LYRIC_EDITED_TABLE),
        StoreKind::MusicUrl => clear_table(conn, MUSIC_URL_TABLE),
        StoreKind::DownloadTask => clear_table(conn, DOWNLOAD_TASK_TABLE),
        StoreKind::ListData => {
            clear_table(conn, LIST_META_TABLE)?;
            clear_table(conn, LIST_MUSIC_TABLE)
        }
        StoreKind::Dislike => clear_table(conn, DISLIKE_RULE_TABLE),
        StoreKind::Misc => conn
            .execute(
                &format!("DELETE FROM {MISC_STORE_TABLE} WHERE name = ?1"),
                params![name],
            )
            .map(|_| ())
            .map_err(|err| err.to_string()),
    }
}

fn clear_table(conn: &Connection, table_name: &str) -> Result<(), String> {
    conn.execute(&format!("DELETE FROM {table_name}"), [])
        .map(|_| ())
        .map_err(|err| err.to_string())
}

fn load_json_object_table(
    conn: &Connection,
    table_name: &str,
    key_column: &str,
) -> Result<Value, String> {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {key_column}, value_json FROM {table_name} ORDER BY {key_column}"
        ))
        .map_err(|err| err.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|err| err.to_string())?;
    let mut map = Map::new();
    for row in rows {
        let (key, text) = row.map_err(|err| err.to_string())?;
        let value = serde_json::from_str::<Value>(&text).map_err(|err| err.to_string())?;
        map.insert(key, value);
    }
    Ok(Value::Object(map))
}

fn save_json_object_table(
    conn: &Connection,
    table_name: &str,
    key_column: &str,
    value: &Value,
) -> Result<(), String> {
    let Some(object) = value.as_object() else {
        return Err(format!("{table_name} expects object value"));
    };
    for (key, item) in object {
        conn.execute(
            &format!(
                "INSERT INTO {table_name}({key_column}, value_json) VALUES(?1, ?2)"
            ),
            params![key, serde_json::to_string(item).map_err(|err| err.to_string())?],
        )
        .map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn load_ordered_array_table(
    conn: &Connection,
    table_name: &str,
    id_column: &str,
) -> Result<Value, String> {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT value_json FROM {table_name} ORDER BY sort_index, {id_column}"
        ))
        .map_err(|err| err.to_string())?;
    let rows = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|err| err.to_string())?;
    let mut list = Vec::new();
    for row in rows {
        let text = row.map_err(|err| err.to_string())?;
        list.push(serde_json::from_str::<Value>(&text).map_err(|err| err.to_string())?);
    }
    Ok(Value::Array(list))
}

fn save_ordered_array_table(
    conn: &Connection,
    table_name: &str,
    id_column: &str,
    value: &Value,
) -> Result<(), String> {
    let Some(list) = value.as_array() else {
        return Err(format!("{table_name} expects array value"));
    };
    let id_field = match id_column {
        "entry_id" => "id",
        "task_id" => "id",
        _ => "id",
    };
    for (index, item) in list.iter().enumerate() {
        let entry_id = json_identity(item, &[id_field], &format!("row_{index}"));
        conn.execute(
            &format!(
                "INSERT INTO {table_name}({id_column}, sort_index, value_json) VALUES(?1, ?2, ?3)"
            ),
            params![
                entry_id,
                index as i64,
                serde_json::to_string(item).map_err(|err| err.to_string())?
            ],
        )
        .map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn load_sound_effect_table(conn: &Connection) -> Result<Value, String> {
    let mut result = Map::new();
    for kind in ["eq", "convolution"] {
        let mut stmt = conn
            .prepare(&format!(
                "SELECT value_json FROM {SOUND_EFFECT_TABLE} WHERE kind = ?1 ORDER BY sort_index"
            ))
            .map_err(|err| err.to_string())?;
        let rows = stmt
            .query_map(params![kind], |row| row.get::<_, String>(0))
            .map_err(|err| err.to_string())?;
        let mut list = Vec::new();
        for row in rows {
            let text = row.map_err(|err| err.to_string())?;
            list.push(serde_json::from_str::<Value>(&text).map_err(|err| err.to_string())?);
        }
        if !list.is_empty() {
            result.insert(kind.to_string(), Value::Array(list));
        }
    }
    Ok(Value::Object(result))
}

fn save_sound_effect_table(conn: &Connection, value: &Value) -> Result<(), String> {
    let Some(object) = value.as_object() else {
        return Err("sound_effect expects object value".to_string());
    };
    for kind in ["eq", "convolution"] {
        let Some(list) = object.get(kind).and_then(Value::as_array) else {
            continue;
        };
        for (index, item) in list.iter().enumerate() {
            conn.execute(
                &format!(
                    "INSERT INTO {SOUND_EFFECT_TABLE}(kind, sort_index, value_json) VALUES(?1, ?2, ?3)"
                ),
                params![
                    kind,
                    index as i64,
                    serde_json::to_string(item).map_err(|err| err.to_string())?
                ],
            )
            .map_err(|err| err.to_string())?;
        }
    }
    Ok(())
}

fn load_string_map_table(conn: &Connection, table_name: &str) -> Result<Value, String> {
    let (key_column, value_column) = if table_name == MUSIC_URL_TABLE {
        ("cache_key", "url")
    } else {
        ("entry_key", "value_text")
    };
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {key_column}, {value_column} FROM {table_name} ORDER BY {key_column}"
        ))
        .map_err(|err| err.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|err| err.to_string())?;
    let mut map = Map::new();
    for row in rows {
        let (key, value) = row.map_err(|err| err.to_string())?;
        map.insert(key, Value::String(value));
    }
    Ok(Value::Object(map))
}

fn save_string_map_table(
    conn: &Connection,
    table_name: &str,
    value: &Value,
) -> Result<(), String> {
    let Some(object) = value.as_object() else {
        return Err(format!("{table_name} expects object value"));
    };
    for (key, item) in object {
        let Some(text) = item.as_str() else {
            return Err(format!("{table_name}.{key} expects string value"));
        };
        conn.execute(
            &format!("INSERT INTO {table_name}(cache_key, url) VALUES(?1, ?2)"),
            params![key, text],
        )
        .map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn load_list_store(conn: &Connection) -> Result<Value, String> {
    let mut user_lists = Vec::new();
    let mut stmt = conn
        .prepare(&format!(
            "SELECT value_json FROM {LIST_META_TABLE} WHERE list_type = 'user' ORDER BY sort_index, list_id"
        ))
        .map_err(|err| err.to_string())?;
    let rows = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|err| err.to_string())?;
    for row in rows {
        let text = row.map_err(|err| err.to_string())?;
        user_lists.push(serde_json::from_str::<Value>(&text).map_err(|err| err.to_string())?);
    }

    let mut default_list = Vec::new();
    let mut love_list = Vec::new();
    let mut temp_list = Vec::new();
    let mut user_list_musics = Map::new();
    let mut stmt = conn
        .prepare(&format!(
            "SELECT list_id, value_json FROM {LIST_MUSIC_TABLE} ORDER BY list_id, sort_index"
        ))
        .map_err(|err| err.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|err| err.to_string())?;
    for row in rows {
        let (list_id, text) = row.map_err(|err| err.to_string())?;
        let value = serde_json::from_str::<Value>(&text).map_err(|err| err.to_string())?;
        match list_id.as_str() {
            "default" => default_list.push(value),
            "love" => love_list.push(value),
            "temp" => temp_list.push(value),
            _ => {
                let entry = user_list_musics
                    .entry(list_id)
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Some(list) = entry.as_array_mut() {
                    list.push(value);
                }
            }
        }
    }

    let mut result = Map::new();
    result.insert("defaultList".to_string(), Value::Array(default_list));
    result.insert("loveList".to_string(), Value::Array(love_list));
    result.insert("tempList".to_string(), Value::Array(temp_list));
    result.insert("userLists".to_string(), Value::Array(user_lists));
    result.insert("userListMusics".to_string(), Value::Object(user_list_musics));
    Ok(Value::Object(result))
}

fn save_list_store(conn: &Connection, value: &Value) -> Result<(), String> {
    let Some(object) = value.as_object() else {
        return Err("list store expects object value".to_string());
    };

    if let Some(user_lists) = object.get("userLists").and_then(Value::as_array) {
        let mut seen = HashSet::new();
        let mut sort_index = 0_i64;
        for item in user_lists {
            let list_id = json_identity(item, &["id"], &format!("user_{sort_index}"));
            if !seen.insert(list_id.clone()) {
                continue;
            }
            conn.execute(
                &format!(
                    "INSERT OR REPLACE INTO {LIST_META_TABLE}(list_id, list_type, sort_index, value_json) VALUES(?1, 'user', ?2, ?3)"
                ),
                params![
                    list_id,
                    sort_index,
                    serde_json::to_string(item).map_err(|err| err.to_string())?
                ],
            )
            .map_err(|err| err.to_string())?;
            sort_index += 1;
        }
    }

    for (list_id, list_key) in [
        ("default", "defaultList"),
        ("love", "loveList"),
        ("temp", "tempList"),
    ] {
        if let Some(list) = object.get(list_key).and_then(Value::as_array) {
            insert_list_musics(conn, list_id, list)?;
        }
    }

    if let Some(user_lists) = object.get("userListMusics").and_then(Value::as_object) {
        for (list_id, list_value) in user_lists {
            let Some(list) = list_value.as_array() else {
                continue;
            };
            insert_list_musics(conn, list_id, list)?;
        }
    }

    Ok(())
}

fn insert_list_musics(
    conn: &Connection,
    list_id: &str,
    list: &[Value],
) -> Result<(), String> {
    for (index, item) in list.iter().enumerate() {
        let music_id = json_identity(item, &["id", "songmid", "songId"], &format!("music_{index}"));
        conn.execute(
            &format!(
                "INSERT OR REPLACE INTO {LIST_MUSIC_TABLE}(list_id, sort_index, music_id, value_json) VALUES(?1, ?2, ?3, ?4)"
            ),
            params![
                list_id,
                index as i64,
                music_id,
                serde_json::to_string(item).map_err(|err| err.to_string())?
            ],
        )
        .map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn load_dislike_store(conn: &Connection) -> Result<Value, String> {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT rule_text FROM {DISLIKE_RULE_TABLE} ORDER BY sort_index"
        ))
        .map_err(|err| err.to_string())?;
    let rows = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|err| err.to_string())?;
    let mut rules = Vec::new();
    for row in rows {
        rules.push(row.map_err(|err| err.to_string())?);
    }
    let mut map = Map::new();
    map.insert("rules".to_string(), Value::String(rules.join("\n")));
    Ok(Value::Object(map))
}

fn save_dislike_store(conn: &Connection, value: &Value) -> Result<(), String> {
    let Some(object) = value.as_object() else {
        return Err("dislike store expects object value".to_string());
    };
    let rules = object
        .get("rules")
        .and_then(Value::as_str)
        .unwrap_or_default();
    for (index, rule) in rules.split('\n').filter(|item| !item.is_empty()).enumerate() {
        conn.execute(
            &format!(
                "INSERT INTO {DISLIKE_RULE_TABLE}(sort_index, rule_text) VALUES(?1, ?2)"
            ),
            params![index as i64, rule],
        )
        .map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn load_misc_store(conn: &Connection, name: &str) -> Result<Value, String> {
    let text = conn
        .query_row(
            &format!(
                "SELECT value_json FROM {MISC_STORE_TABLE} WHERE name = ?1 LIMIT 1"
            ),
            params![name],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|err| err.to_string())?;
    match text {
        Some(text) => serde_json::from_str(&text).map_err(|err| err.to_string()),
        None => Ok(Value::Null),
    }
}

fn save_misc_store(conn: &Connection, name: &str, value: &Value) -> Result<(), String> {
    conn.execute(
        &format!(
            "INSERT INTO {MISC_STORE_TABLE}(name, value_json) VALUES(?1, ?2)"
        ),
        params![name, serde_json::to_string(value).map_err(|err| err.to_string())?],
    )
    .map(|_| ())
    .map_err(|err| err.to_string())
}

fn json_identity(value: &Value, keys: &[&str], fallback: &str) -> String {
    if let Some(object) = value.as_object() {
        for key in keys {
            if let Some(value) = object.get(*key) {
                if let Some(text) = scalar_to_string(value) {
                    if !text.is_empty() {
                        return text;
                    }
                }
            }
        }
    }
    fallback.to_string()
}

fn scalar_to_string(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        Value::Bool(value) => Some(value.to_string()),
        _ => None,
    }
}
