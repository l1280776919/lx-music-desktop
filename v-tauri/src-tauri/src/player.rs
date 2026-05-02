use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerTogglePlayInfo {
    pub music_info: Value,
    pub list_id: Option<String>,
    pub is_temp_play: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerToggleDislikeInfo {
    pub names: Vec<String>,
    pub music_names: Vec<String>,
    pub singer_names: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerToggleResolvePayload {
    pub is_next: bool,
    pub is_auto_toggle: bool,
    pub toggle_play_method: String,
    pub current_list_id: Option<String>,
    pub current_list: Vec<Value>,
    pub played_list: Vec<PlayerTogglePlayInfo>,
    pub temp_play_list: Vec<PlayerTogglePlayInfo>,
    pub current_music_info: Option<Value>,
    pub player_music_info: Option<Value>,
    pub random_next_info: Option<PlayerTogglePlayInfo>,
    pub dislike_info: PlayerToggleDislikeInfo,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerToggleResolveResult {
    pub selected: Option<PlayerTogglePlayInfo>,
    pub cleaned_played_list: Vec<PlayerTogglePlayInfo>,
    pub consume_temp_play: bool,
    pub should_stop: bool,
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

#[tauri::command]
pub fn player_resolve_toggle(
    payload: PlayerToggleResolvePayload,
) -> Result<PlayerToggleResolveResult, String> {
    Ok(resolve_toggle(payload))
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

#[derive(Default)]
struct DislikeLookup {
    names: HashSet<String>,
    music_names: HashSet<String>,
    singer_names: HashSet<String>,
}

#[derive(Default)]
struct FilterMusicListResult {
    filtered_list: Vec<Value>,
    can_play_list: Vec<Value>,
    player_index: isize,
}

fn resolve_toggle(payload: PlayerToggleResolvePayload) -> PlayerToggleResolveResult {
    let mut result = PlayerToggleResolveResult::default();

    if payload.is_next {
        if let Some(play_info) = payload.temp_play_list.first() {
            result.selected = Some(play_info.clone());
            result.consume_temp_play = true;
            return result;
        }
    }

    let Some(current_music_info) = payload.current_music_info.as_ref() else {
        result.should_stop = true;
        return result;
    };
    let Some(current_list_id) = payload.current_list_id.clone() else {
        result.should_stop = true;
        return result;
    };

    let current_id = music_id(current_music_info).or_else(|| {
        payload
            .player_music_info
            .as_ref()
            .and_then(music_id)
    });

    result.cleaned_played_list = cleanup_played_list(
        payload.played_list,
        &current_list_id,
        &payload.current_list,
    );

    if payload.toggle_play_method == "random" {
        if let Some(current_id) = current_id.as_deref() {
            if let Some(position) = find_played_index(&result.cleaned_played_list, current_id) {
                let history_item = if payload.is_next {
                    result.cleaned_played_list.get(position + 1)
                } else if position > 0 {
                    result.cleaned_played_list.get(position - 1)
                } else {
                    None
                };
                if let Some(play_info) = history_item {
                    result.selected = Some(play_info.clone());
                    return result;
                }
            }
        }

        if payload.is_next {
            if let Some(play_info) = payload.random_next_info {
                result.selected = Some(play_info);
                return result;
            }
        }
    }

    let dislike_lookup = build_dislike_lookup(&payload.dislike_info);
    let player_music_info = payload
        .player_music_info
        .as_ref()
        .or(payload.current_music_info.as_ref());
    let mut filter_result = filter_music_list(
        &payload.current_list,
        &result.cleaned_played_list,
        &current_list_id,
        player_music_info,
        payload.is_next,
        &dislike_lookup,
    );

    if filter_result.filtered_list.is_empty() && !result.cleaned_played_list.is_empty() {
        result.cleaned_played_list.clear();
        filter_result.filtered_list = filter_result.can_play_list.clone();
    }
    if filter_result.filtered_list.is_empty() {
        result.should_stop = true;
        return result;
    }

    if filter_result.player_index < 0 {
        filter_result.player_index = 0;
    }
    let next_index = resolve_target_index(
        &payload.toggle_play_method,
        payload.is_next,
        payload.is_auto_toggle,
        filter_result.player_index,
        filter_result.filtered_list.len(),
    );
    if next_index < 0 {
        return result;
    }

    if let Some(music_info) = filter_result.filtered_list.get(next_index as usize).cloned() {
        result.selected = Some(PlayerTogglePlayInfo {
            music_info,
            list_id: Some(current_list_id),
            is_temp_play: false,
        });
    }

    result
}

fn build_dislike_lookup(info: &PlayerToggleDislikeInfo) -> DislikeLookup {
    let names = info
        .names
        .iter()
        .map(|item| normalize_rule(item))
        .filter(|item| !item.is_empty())
        .collect();
    let music_names = info
        .music_names
        .iter()
        .map(|item| normalize_text(item))
        .filter(|item| !item.is_empty())
        .collect();
    let singer_names = info
        .singer_names
        .iter()
        .map(|item| normalize_text(item))
        .filter(|item| !item.is_empty())
        .collect();

    DislikeLookup {
        names,
        music_names,
        singer_names,
    }
}

fn cleanup_played_list(
    played_list: Vec<PlayerTogglePlayInfo>,
    current_list_id: &str,
    current_list: &[Value],
) -> Vec<PlayerTogglePlayInfo> {
    let current_ids: HashSet<String> = current_list.iter().filter_map(music_id).collect();

    played_list
        .into_iter()
        .filter(|play_info| {
            if play_info.list_id.as_deref() != Some(current_list_id) {
                return true;
            }
            played_item_id(play_info)
                .map(|id| current_ids.contains(&id))
                .unwrap_or(false)
        })
        .collect()
}

fn filter_music_list(
    list: &[Value],
    played_list: &[PlayerTogglePlayInfo],
    list_id: &str,
    player_music_info: Option<&Value>,
    is_next: bool,
    dislike_lookup: &DislikeLookup,
) -> FilterMusicListResult {
    let mut player_index = -1;
    let mut can_play_list = Vec::new();
    let mut filtered_list = Vec::new();
    let mut filtered_played_ids: Vec<String> = played_list
        .iter()
        .filter(|item| item.list_id.as_deref() == Some(list_id) && !item.is_temp_play)
        .filter_map(played_item_id)
        .collect();

    let player_music_id = player_music_info.and_then(music_id);
    let mut is_dislike = false;

    for item in list {
        if is_download_item(item) {
            if !is_download_complete(item) {
                continue;
            }
        } else if has_dislike(item, dislike_lookup) {
            if music_id(item).as_deref() != player_music_id.as_deref() {
                continue;
            }
            is_dislike = true;
        }

        can_play_list.push(item.clone());

        let Some(id) = music_id(item) else {
            continue;
        };
        if let Some(index) = filtered_played_ids.iter().position(|played_id| played_id == &id) {
            filtered_played_ids.remove(index);
            continue;
        }
        filtered_list.push(item.clone());
    }

    if let Some(player_music_info) = player_music_info {
        let player_music_id = music_id(player_music_info);
        if is_dislike {
            if filtered_list.len() <= 1 {
                if !filtered_list.is_empty() {
                    filtered_list.remove(0);
                }
                if can_play_list.len() > 1 {
                    if let Some(current_music_index) = player_music_id
                        .as_deref()
                        .and_then(|id| can_play_list.iter().position(|item| music_id(item).as_deref() == Some(id)))
                    {
                        if is_next {
                            player_index = current_music_index as isize - 1;
                            if player_index < 0 && can_play_list.len() > 1 {
                                player_index = can_play_list.len() as isize - 2;
                            }
                        } else {
                            player_index = current_music_index as isize;
                            if can_play_list.len() <= 1 {
                                player_index = -1;
                            }
                        }
                        can_play_list.remove(current_music_index);
                    }
                } else if !can_play_list.is_empty() {
                    can_play_list.remove(0);
                }
            } else if let Some(current_music_index) = player_music_id
                .as_deref()
                .and_then(|id| filtered_list.iter().position(|item| music_id(item).as_deref() == Some(id)))
            {
                if is_next {
                    player_index = current_music_index as isize - 1;
                    if player_index < 0 && filtered_list.len() > 1 {
                        player_index = filtered_list.len() as isize - 2;
                    }
                } else {
                    player_index = current_music_index as isize;
                    if filtered_list.len() <= 1 {
                        player_index = -1;
                    }
                }
                filtered_list.remove(current_music_index);
            }
        } else {
            let target_list = if filtered_list.is_empty() {
                &can_play_list
            } else {
                &filtered_list
            };
            player_index = player_music_id
                .as_deref()
                .and_then(|id| target_list.iter().position(|item| music_id(item).as_deref() == Some(id)))
                .map(|index| index as isize)
                .unwrap_or(-1);
        }
    }

    FilterMusicListResult {
        filtered_list,
        can_play_list,
        player_index,
    }
}

fn resolve_target_index(
    toggle_play_method: &str,
    is_next: bool,
    is_auto_toggle: bool,
    player_index: isize,
    list_len: usize,
) -> isize {
    if list_len == 0 {
        return -1;
    }

    let list_len = list_len as isize;
    let mut toggle_play_method = toggle_play_method.to_string();
    if !is_auto_toggle {
        match toggle_play_method.as_str() {
            "list" | "singleLoop" | "none" => toggle_play_method = "listLoop".to_string(),
            _ => {}
        }
    }

    if is_next {
        match toggle_play_method.as_str() {
            "listLoop" => {
                if player_index == list_len - 1 {
                    0
                } else {
                    player_index + 1
                }
            }
            "random" => random_index(list_len as usize),
            "list" => {
                if player_index == list_len - 1 {
                    -1
                } else {
                    player_index + 1
                }
            }
            "singleLoop" => player_index,
            _ => -1,
        }
    } else {
        match toggle_play_method.as_str() {
            "random" => random_index(list_len as usize),
            "listLoop" | "list" => {
                if player_index <= 0 {
                    list_len - 1
                } else {
                    player_index - 1
                }
            }
            "singleLoop" => player_index,
            _ => -1,
        }
    }
}

fn random_index(list_len: usize) -> isize {
    if list_len == 0 {
        return -1;
    }
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    (seed % list_len as u128) as isize
}

fn find_played_index(played_list: &[PlayerTogglePlayInfo], current_id: &str) -> Option<usize> {
    played_list
        .iter()
        .position(|play_info| played_item_id(play_info).as_deref() == Some(current_id))
}

fn played_item_id(play_info: &PlayerTogglePlayInfo) -> Option<String> {
    music_id(&play_info.music_info)
}

fn music_id(music_info: &Value) -> Option<String> {
    music_info
        .get("id")
        .and_then(Value::as_str)
        .map(str::to_owned)
}

fn is_download_item(music_info: &Value) -> bool {
    music_info.get("progress").is_some()
}

fn is_download_complete(music_info: &Value) -> bool {
    music_info
        .get("isComplate")
        .and_then(Value::as_bool)
        .unwrap_or(true)
}

fn has_dislike(music_info: &Value, dislike_lookup: &DislikeLookup) -> bool {
    let name = normalize_text(
        music_info
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or_default(),
    );
    let singer = normalize_text(
        music_info
            .get("singer")
            .and_then(Value::as_str)
            .unwrap_or_default(),
    );

    dislike_lookup.music_names.contains(&name)
        || dislike_lookup.singer_names.contains(&singer)
        || dislike_lookup.names.contains(&format!("{name}@{singer}"))
}

fn normalize_rule(rule: &str) -> String {
    let mut parts = rule.splitn(2, '@');
    let name = normalize_text(parts.next().unwrap_or_default());
    let singer = normalize_text(parts.next().unwrap_or_default());
    if singer.is_empty() {
        name
    } else {
        format!("{name}@{singer}")
    }
}

fn normalize_text(text: &str) -> String {
    text.replace('@', "#").trim().to_lowercase()
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
