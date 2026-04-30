use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

pub const PLAYER_ACTION_EVENT: &str = "player_action";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStatusState {
    pub status: Option<String>,
    pub name: Option<String>,
    pub singer: Option<String>,
    pub album_name: Option<String>,
    pub pic_url: Option<String>,
    pub progress: Option<f64>,
    pub duration: Option<f64>,
    pub playback_rate: Option<f64>,
    pub lyric_line_text: Option<String>,
    pub lyric_line_all_text: Option<String>,
    pub lyric: Option<String>,
    pub tlyric: Option<String>,
    pub rlyric: Option<String>,
    pub lxlyric: Option<String>,
    pub collect: Option<bool>,
    pub volume: Option<f64>,
    pub mute: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerButtonsState {
    pub empty: bool,
    pub collect: bool,
    pub play: bool,
    pub next: bool,
    pub prev: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBackendSnapshot {
    pub status: PlayerStatusState,
    pub buttons: PlayerButtonsState,
    pub last_action: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerActionPayload {
    pub action: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Default)]
pub struct PlayerBackendState {
    inner: Mutex<PlayerBackendSnapshot>,
}

impl PlayerBackendState {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(PlayerBackendSnapshot::default()),
        }
    }
}

#[tauri::command]
pub fn player_update_status(
    state: State<'_, PlayerBackendState>,
    status: PlayerStatusState,
) -> Result<PlayerBackendSnapshot, String> {
    let mut guard = state.inner.lock().map_err(|err| err.to_string())?;
    merge_player_status(&mut guard.status, status);
    Ok(guard.clone())
}

#[tauri::command]
pub fn player_set_buttons(
    state: State<'_, PlayerBackendState>,
    buttons: PlayerButtonsState,
) -> Result<PlayerBackendSnapshot, String> {
    let mut guard = state.inner.lock().map_err(|err| err.to_string())?;
    guard.buttons = buttons;
    Ok(guard.clone())
}

#[tauri::command]
pub fn player_get_snapshot(
    state: State<'_, PlayerBackendState>,
) -> Result<PlayerBackendSnapshot, String> {
    let guard = state.inner.lock().map_err(|err| err.to_string())?;
    Ok(guard.clone())
}

#[tauri::command]
pub fn player_dispatch_action(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
    payload: PlayerActionPayload,
) -> Result<PlayerBackendSnapshot, String> {
    dispatch_action(app, state, payload)
}

#[tauri::command]
pub fn player_play(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
) -> Result<PlayerBackendSnapshot, String> {
    dispatch_action(app, state, PlayerActionPayload {
        action: "play".to_string(),
        data: None,
    })
}

#[tauri::command]
pub fn player_pause(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
) -> Result<PlayerBackendSnapshot, String> {
    dispatch_action(app, state, PlayerActionPayload {
        action: "pause".to_string(),
        data: None,
    })
}

#[tauri::command]
pub fn player_stop(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
) -> Result<PlayerBackendSnapshot, String> {
    dispatch_action(app, state, PlayerActionPayload {
        action: "stop".to_string(),
        data: None,
    })
}

#[tauri::command]
pub fn player_toggle_play(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
) -> Result<PlayerBackendSnapshot, String> {
    dispatch_action(app, state, PlayerActionPayload {
        action: "togglePlay".to_string(),
        data: None,
    })
}

#[tauri::command]
pub fn player_prev(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
) -> Result<PlayerBackendSnapshot, String> {
    dispatch_action(app, state, PlayerActionPayload {
        action: "prev".to_string(),
        data: None,
    })
}

#[tauri::command]
pub fn player_next(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
) -> Result<PlayerBackendSnapshot, String> {
    dispatch_action(app, state, PlayerActionPayload {
        action: "next".to_string(),
        data: None,
    })
}

#[tauri::command]
pub fn player_seek(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
    progress: f64,
) -> Result<PlayerBackendSnapshot, String> {
    dispatch_action(app, state, PlayerActionPayload {
        action: "seek".to_string(),
        data: Some(serde_json::json!(progress)),
    })
}

#[tauri::command]
pub fn player_set_volume(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
    volume: f64,
) -> Result<PlayerBackendSnapshot, String> {
    dispatch_action(app, state, PlayerActionPayload {
        action: "volume".to_string(),
        data: Some(serde_json::json!(volume)),
    })
}

#[tauri::command]
pub fn player_set_mute(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
    mute: bool,
) -> Result<PlayerBackendSnapshot, String> {
    dispatch_action(app, state, PlayerActionPayload {
        action: "mute".to_string(),
        data: Some(serde_json::json!(mute)),
    })
}

#[tauri::command]
pub fn player_collect(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
) -> Result<PlayerBackendSnapshot, String> {
    dispatch_action(app, state, PlayerActionPayload {
        action: "collect".to_string(),
        data: None,
    })
}

#[tauri::command]
pub fn player_uncollect(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
) -> Result<PlayerBackendSnapshot, String> {
    dispatch_action(app, state, PlayerActionPayload {
        action: "unCollect".to_string(),
        data: None,
    })
}

#[tauri::command]
pub fn player_dislike(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
) -> Result<PlayerBackendSnapshot, String> {
    dispatch_action(app, state, PlayerActionPayload {
        action: "dislike".to_string(),
        data: None,
    })
}

fn merge_player_status(target: &mut PlayerStatusState, patch: PlayerStatusState) {
    if let Some(value) = patch.status {
        target.status = Some(value);
    }
    if let Some(value) = patch.name {
        target.name = Some(value);
    }
    if let Some(value) = patch.singer {
        target.singer = Some(value);
    }
    if let Some(value) = patch.album_name {
        target.album_name = Some(value);
    }
    if let Some(value) = patch.pic_url {
        target.pic_url = Some(value);
    }
    if let Some(value) = patch.progress {
        target.progress = Some(value);
    }
    if let Some(value) = patch.duration {
        target.duration = Some(value);
    }
    if let Some(value) = patch.playback_rate {
        target.playback_rate = Some(value);
    }
    if let Some(value) = patch.lyric_line_text {
        target.lyric_line_text = Some(value);
    }
    if let Some(value) = patch.lyric_line_all_text {
        target.lyric_line_all_text = Some(value);
    }
    if let Some(value) = patch.lyric {
        target.lyric = Some(value);
    }
    if let Some(value) = patch.tlyric {
        target.tlyric = Some(value);
    }
    if let Some(value) = patch.rlyric {
        target.rlyric = Some(value);
    }
    if let Some(value) = patch.lxlyric {
        target.lxlyric = Some(value);
    }
    if let Some(value) = patch.collect {
        target.collect = Some(value);
    }
    if let Some(value) = patch.volume {
        target.volume = Some(value);
    }
    if let Some(value) = patch.mute {
        target.mute = Some(value);
    }
}

fn dispatch_action(
    app: AppHandle,
    state: State<'_, PlayerBackendState>,
    payload: PlayerActionPayload,
) -> Result<PlayerBackendSnapshot, String> {
    app.emit(PLAYER_ACTION_EVENT, payload.clone())
        .map_err(|err| err.to_string())?;
    let mut guard = state.inner.lock().map_err(|err| err.to_string())?;
    guard.last_action = Some(payload.action);
    Ok(guard.clone())
}
