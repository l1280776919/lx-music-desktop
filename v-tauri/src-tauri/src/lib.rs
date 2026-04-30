mod player;

use rfd::FileDialog;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Proxy;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::pkcs8::DecodePublicKey;
use rsa::traits::PublicKeyParts;
use rsa::{BigUint, RsaPublicKey};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};
use tauri::{AppHandle, Manager, WebviewWindow};

#[derive(Debug, Serialize, Deserialize)]
struct FileMetadata {
    size: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HttpRequestPayload {
    method: String,
    url: String,
    headers: Option<std::collections::HashMap<String, String>>,
    body: Option<String>,
    timeout: Option<u64>,
    proxy: Option<HttpProxyPayload>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HttpProxyPayload {
    host: String,
    port: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HttpResponsePayload {
    status_code: u16,
    status_message: String,
    headers: std::collections::HashMap<String, String>,
    bytes: usize,
    body: String,
    raw: Vec<u8>,
    final_url: String,
    http_version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RsaEncryptPayload {
    buffer: Vec<u8>,
    key: String,
}

fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|err| err.to_string())?;
    fs::create_dir_all(&dir).map_err(|err| err.to_string())?;
    Ok(dir)
}

fn build_header_map(
    headers: Option<std::collections::HashMap<String, String>>,
) -> Result<HeaderMap, String> {
    let mut header_map = HeaderMap::new();
    for (key, value) in headers.unwrap_or_default() {
        let header_name =
            HeaderName::from_bytes(key.as_bytes()).map_err(|err| err.to_string())?;
        let header_value = HeaderValue::from_str(&value).map_err(|err| err.to_string())?;
        header_map.insert(header_name, header_value);
    }
    Ok(header_map)
}

fn build_proxy(proxy: Option<HttpProxyPayload>) -> Result<Option<Proxy>, String> {
    let Some(proxy) = proxy else {
        return Ok(None);
    };
    if proxy.host.is_empty() {
        return Ok(None);
    }
    let proxy_url = format!("http://{}:{}", proxy.host, proxy.port);
    Proxy::all(&proxy_url)
        .map(Some)
        .map_err(|err| err.to_string())
}

fn parse_rsa_public_key(key: &str) -> Result<RsaPublicKey, String> {
    RsaPublicKey::from_public_key_pem(key)
        .or_else(|_| RsaPublicKey::from_pkcs1_pem(key))
        .map_err(|err| err.to_string())
}

fn store_file_path(app: &AppHandle, name: &str) -> Result<PathBuf, String> {
    let dir = app_data_dir(app)?;
    Ok(dir.join(format!("{name}.json")))
}

#[tauri::command]
fn store_get(app: AppHandle, name: String) -> Result<Option<Value>, String> {
    let path = store_file_path(&app, &name)?;
    if !path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(path).map_err(|err| err.to_string())?;
    serde_json::from_str(&text)
        .map(Some)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn store_set(app: AppHandle, name: String, value: Value) -> Result<(), String> {
    let path = store_file_path(&app, &name)?;
    let text = serde_json::to_string_pretty(&value).map_err(|err| err.to_string())?;
    fs::write(path, text).map_err(|err| err.to_string())
}

#[tauri::command]
fn fs_exists(path: String) -> bool {
    Path::new(&path).exists()
}

#[tauri::command]
fn fs_create_dir(path: String) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|err| err.to_string())
}

#[tauri::command]
fn fs_create_dir_if_missing(path: String) -> Result<bool, String> {
    if Path::new(&path).exists() {
        return Ok(true);
    }
    fs::create_dir_all(path)
        .map(|_| true)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn fs_remove_file(path: String) -> Result<(), String> {
    if !Path::new(&path).exists() {
        return Ok(());
    }
    fs::remove_file(path).map_err(|err| err.to_string())
}

#[tauri::command]
fn fs_read_binary(path: String) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|err| err.to_string())
}

#[tauri::command]
fn fs_read_text(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|err| err.to_string())
}

#[tauri::command]
fn fs_write_binary(path: String, data: Vec<u8>) -> Result<(), String> {
    fs::write(path, data).map_err(|err| err.to_string())
}

#[tauri::command]
fn fs_write_text(path: String, text: String) -> Result<(), String> {
    fs::write(path, text).map_err(|err| err.to_string())
}

#[tauri::command]
fn fs_copy_file(source_path: String, dist_path: String) -> Result<(), String> {
    fs::copy(source_path, dist_path)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn fs_move_file(source_path: String, dist_path: String) -> Result<(), String> {
    fs::rename(source_path, dist_path).map_err(|err| err.to_string())
}

#[tauri::command]
fn fs_metadata(path: String) -> Result<Option<FileMetadata>, String> {
    if !Path::new(&path).exists() {
        return Ok(None);
    }
    let metadata = fs::metadata(path).map_err(|err| err.to_string())?;
    Ok(Some(FileMetadata {
        size: metadata.len(),
    }))
}

fn build_dialog(options: &Value) -> FileDialog {
    let mut dialog = FileDialog::new();
    if let Some(title) = options.get("title").and_then(Value::as_str) {
        dialog = dialog.set_title(title);
    }
    if let Some(default_path) = options.get("defaultPath").and_then(Value::as_str) {
        dialog = dialog.set_directory(default_path);
    }
    if let Some(filters) = options.get("filters").and_then(Value::as_array) {
        for filter in filters {
            if let (Some(name), Some(extensions)) = (
                filter.get("name").and_then(Value::as_str),
                filter.get("extensions").and_then(Value::as_array),
            ) {
                let ext_values: Vec<&str> = extensions.iter().filter_map(Value::as_str).collect();
                dialog = dialog.add_filter(name, &ext_values);
            }
        }
    }
    dialog
}

#[tauri::command]
fn dialog_open(options: Value) -> Result<Option<Value>, String> {
    let dialog = build_dialog(&options);
    let properties = options
        .get("properties")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let has = |name: &str| {
        properties
            .iter()
            .any(|value| value.as_str() == Some(name))
    };

    if has("openDirectory") {
        if has("multiSelections") {
            let selected = dialog
                .pick_folders()
                .map(|paths| {
                    paths
                        .into_iter()
                        .map(|path| Value::String(path.to_string_lossy().to_string()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            return Ok((!selected.is_empty()).then_some(Value::Array(selected)));
        }
        return Ok(dialog
            .pick_folder()
            .map(|path| Value::String(path.to_string_lossy().to_string())));
    }

    if has("multiSelections") {
        let selected = dialog
            .pick_files()
            .map(|paths| {
                paths
                    .into_iter()
                    .map(|path| Value::String(path.to_string_lossy().to_string()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        return Ok((!selected.is_empty()).then_some(Value::Array(selected)));
    }

    Ok(dialog
        .pick_file()
        .map(|path| Value::String(path.to_string_lossy().to_string())))
}

#[tauri::command]
fn dialog_save(options: Value) -> Result<Option<String>, String> {
    let dialog = build_dialog(&options);
    Ok(dialog
        .save_file()
        .map(|path| path.to_string_lossy().to_string()))
}

#[tauri::command]
fn open_in_explorer(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let target = Path::new(&path);
        if target.is_file() {
            std::process::Command::new("explorer")
                .args(["/select,", &path])
                .spawn()
                .map_err(|err| err.to_string())?;
        } else {
            std::process::Command::new("explorer")
                .arg(&path)
                .spawn()
                .map_err(|err| err.to_string())?;
        }
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("-R")
            .arg(&path)
            .spawn()
            .map_err(|err| err.to_string())?;
        return Ok(());
    }

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        open::that(path).map_err(|err| err.to_string())?;
        Ok(())
    }
}

#[tauri::command]
fn window_minimize(window: WebviewWindow) -> Result<(), String> {
    window.minimize().map_err(|err| err.to_string())
}

#[tauri::command]
fn window_toggle_maximize(window: WebviewWindow) -> Result<(), String> {
    let is_maximized = window.is_maximized().map_err(|err| err.to_string())?;
    if is_maximized {
        window.unmaximize().map_err(|err| err.to_string())
    } else {
        window.maximize().map_err(|err| err.to_string())
    }
}

#[tauri::command]
fn window_close(window: WebviewWindow) -> Result<(), String> {
    window.close().map_err(|err| err.to_string())
}

#[tauri::command]
fn window_focus(window: WebviewWindow) -> Result<(), String> {
    window.set_focus().map_err(|err| err.to_string())
}

#[tauri::command]
fn window_set_fullscreen(window: WebviewWindow, fullscreen: bool) -> Result<(), String> {
    window
        .set_fullscreen(fullscreen)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn app_quit(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn http_request(payload: HttpRequestPayload) -> Result<HttpResponsePayload, String> {
    let mut builder = Client::builder()
        .timeout(Duration::from_millis(payload.timeout.unwrap_or(15_000)))
        .http1_only()
        .redirect(reqwest::redirect::Policy::limited(10))
        .tcp_nodelay(true);
    if let Some(proxy) = build_proxy(payload.proxy)? {
        builder = builder.proxy(proxy);
    }
    let client = builder.build().map_err(|err| err.to_string())?;
    let headers = build_header_map(payload.headers)?;
    let method = payload
        .method
        .parse::<reqwest::Method>()
        .map_err(|err| err.to_string())?;
    let mut request = client.request(method, payload.url).headers(headers);
    if let Some(body) = payload.body {
        request = request.body(body);
    }
    let response = request.send().map_err(|err| err.to_string())?;
    let status_code = response.status().as_u16();
    let status_message = response.status().canonical_reason().unwrap_or("").to_string();
    let final_url = response.url().to_string();
    let http_version = format!("{:?}", response.version());
    let headers = response
        .headers()
        .iter()
        .filter_map(|(key, value)| {
            value
                .to_str()
                .ok()
                .map(|value| (key.as_str().to_string(), value.to_string()))
        })
        .collect();
    let raw = response.bytes().map_err(|err| err.to_string())?.to_vec();
    let bytes = raw.len();
    let body = String::from_utf8_lossy(&raw).to_string();
    Ok(HttpResponsePayload {
        status_code,
        status_message,
        headers,
        bytes,
        body,
        raw,
        final_url,
        http_version,
    })
}

#[tauri::command]
fn rsa_public_encrypt(payload: RsaEncryptPayload) -> Result<Vec<u8>, String> {
    let public_key = parse_rsa_public_key(&payload.key)?;
    let modulus = public_key.n();
    let exponent = public_key.e();
    let size = modulus.bits().div_ceil(8);
    if payload.buffer.len() > size {
        return Err("buffer is longer than rsa key size".to_string());
    }
    let mut padded = vec![0u8; size - payload.buffer.len()];
    padded.extend_from_slice(&payload.buffer);
    let message = BigUint::from_bytes_be(&padded);
    let encrypted = message.modpow(exponent, modulus);
    let mut result = encrypted.to_bytes_be();
    if result.len() < size {
        let mut output = vec![0u8; size - result.len()];
        output.append(&mut result);
        Ok(output)
    } else {
        Ok(result)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(player::PlayerBackendState::new())
        .invoke_handler(tauri::generate_handler![
            store_get,
            store_set,
            fs_exists,
            fs_create_dir,
            fs_create_dir_if_missing,
            fs_remove_file,
            fs_read_binary,
            fs_read_text,
            fs_write_binary,
            fs_write_text,
            fs_copy_file,
            fs_move_file,
            fs_metadata,
            dialog_open,
            dialog_save,
            open_in_explorer,
            window_minimize,
            window_toggle_maximize,
            window_close,
            window_focus,
            window_set_fullscreen,
            app_quit,
            http_request,
            rsa_public_encrypt,
            player::player_update_status,
            player::player_set_buttons,
            player::player_get_snapshot,
            player::player_dispatch_action,
            player::player_play,
            player::player_pause,
            player::player_stop,
            player::player_toggle_play,
            player::player_prev,
            player::player_next,
            player::player_seek,
            player::player_set_volume,
            player::player_set_mute,
            player::player_collect,
            player::player_uncollect,
            player::player_dislike,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
