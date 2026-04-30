import { openDirInExplorer as revealInExplorer } from './electron'
import { hasTauriContext, safeInvoke } from './runtime'
import {
  cancelUserApiRuntimeRequest,
  clearUserApiRuntime,
  getUserApiRuntimeStatus,
  loadUserApiRuntime,
  requestUserApiRuntime,
} from './userApiRuntime'
import defaultSetting from '@common/defaultSetting'
import { DEFAULT_SETTING, DATA_KEYS, STORE_NAMES } from '@common/constants'
import { HOTKEY_COMMON, HOTKEY_DESKTOP_LYRIC, HOTKEY_PLAYER } from '@common/hotKey'
import builtinThemes from '@common/theme/index.json'

type RemoveListener = () => void
type Listener<T = any> = (event: { params: T }) => void
type UserApiInfo = LX.UserApi.UserApiInfo & { script?: string }

const listeners = new Map<string, Set<Listener>>()
const MUSIC_URL_STORE = 'tauri_music_url_cache'
const DOWNLOAD_TASK_STORE = 'tauri_download_task_store'
const emit = <T>(name: string, params: T) => {
  listeners.get(name)?.forEach(listener => listener({ params }))
}
const on = <T>(name: string, listener: Listener<T>): RemoveListener => {
  const set = listeners.get(name) ?? new Set<Listener>()
  set.add(listener as Listener)
  listeners.set(name, set)
  return () => {
    set.delete(listener as Listener)
  }
}
const browserStoreKey = (name: string) => `lx-tauri:${name}`

const storeGet = async<T>(name: string, fallback: T): Promise<T> => {
  if (!hasTauriContext()) {
    const raw = localStorage.getItem(browserStoreKey(name))
    return raw ? JSON.parse(raw) as T : fallback
  }
  const value = await safeInvoke<T | null>('store_get', { name }, null)
  return value ?? fallback
}
const storeSet = async(name: string, value: unknown) => {
  if (!hasTauriContext()) {
    localStorage.setItem(browserStoreKey(name), JSON.stringify(value))
    return
  }
  await safeInvoke('store_set', { name, value }, undefined)
}
const dataStoreGet = async<T>(key: string, fallback: T): Promise<T> => {
  const data = await storeGet<Record<string, T>>(STORE_NAMES.DATA, {} as Record<string, T>)
  return data[key] ?? fallback
}
const dataStoreSet = async(key: string, value: unknown) => {
  const data = await storeGet<Record<string, unknown>>(STORE_NAMES.DATA, {})
  data[key] = value
  await storeSet(STORE_NAMES.DATA, data)
}

const USER_API_INFO_LIMITS = {
  name: 24,
  description: 36,
  author: 56,
  homepage: 1024,
  version: 36,
} as const

const parseUserApiScriptInfo = (script: string) => {
  const block = /^\/\*[\S\s]+?\*\//.exec(script)?.[0]
  if (!block) throw new Error('无效的自定义源文件')
  const lines = block.split(/\r?\n/)
  const rxp = /^\s?\*\s?@(\w+)\s(.+)$/
  const info: Record<keyof typeof USER_API_INFO_LIMITS, string> = {
    name: '',
    description: '',
    author: '',
    homepage: '',
    version: '',
  }
  for (const line of lines) {
    const result = rxp.exec(line)
    if (!result) continue
    const key = result[1] as keyof typeof USER_API_INFO_LIMITS
    if (!(key in USER_API_INFO_LIMITS)) continue
    info[key] = result[2].trim()
  }
  for (const [key, len] of Object.entries(USER_API_INFO_LIMITS) as Array<[keyof typeof USER_API_INFO_LIMITS, number]>) {
    if (info[key].length > len) info[key] = info[key].slice(0, len) + '...'
  }
  info.name ||= `user_api_${new Date().toLocaleString()}`
  return info
}

const getUserApiStore = async() => storeGet<UserApiInfo[]>(STORE_NAMES.USER_API, [])
const setUserApiStore = async(list: UserApiInfo[]) => {
  await storeSet(STORE_NAMES.USER_API, list)
}
const toUserApiInfo = ({ script: _script, ...info }: UserApiInfo): LX.UserApi.UserApiInfo => info
const emitUserApiStatus = (status: boolean, message?: string, apiInfo?: LX.UserApi.UserApiInfo) => {
  emit('userApiStatus', { status, message, apiInfo })
}

export const allHotKeys = [
  ...Object.values(HOTKEY_COMMON),
  ...Object.values(HOTKEY_PLAYER),
  ...Object.values(HOTKEY_DESKTOP_LYRIC),
]

export const getSetting = async() => {
  const saved = await storeGet<Partial<LX.AppSetting>>(STORE_NAMES.APP_SETTINGS, {})
  return { ...defaultSetting, ...saved }
}
export const updateSetting = async(setting: Partial<LX.AppSetting>) => {
  const current = await getSetting()
  const merged = { ...current, ...setting }
  await storeSet(STORE_NAMES.APP_SETTINGS, merged)
  emit('settingChanged', setting)
}
export const onSettingChanged = (listener: Listener<Partial<LX.AppSetting>>): RemoveListener => on('settingChanged', listener)
export const onThemeChange = (listener: Listener<LX.ThemeSetting>): RemoveListener => {
  const emitTheme = () => {
    const shouldUseDarkColors = window.matchMedia('(prefers-color-scheme: dark)').matches
    listener({
      params: {
        shouldUseDarkColors,
        theme: {
          id: shouldUseDarkColors ? 'dark' : 'light',
        } as LX.Theme,
      },
    })
  }
  const media = window.matchMedia('(prefers-color-scheme: dark)')
  const handler = () => emitTheme()
  media.addEventListener?.('change', handler)
  emitTheme()
  return () => media.removeEventListener?.('change', handler)
}

export const sendInited = () => {}

export const getHotKeyConfig = async() => {
  return storeGet<LX.HotKeyConfigAll>(STORE_NAMES.HOTKEY, {
    local: { enable: false, keys: {} },
    global: { enable: false, keys: {} },
  })
}
export const hotKeySetEnable = async({ type, enable }: { type: 'local' | 'global', enable: boolean }) => {
  const config = await getHotKeyConfig()
  config[type].enable = enable
  await storeSet(STORE_NAMES.HOTKEY, config)
}
export const hotKeySetConfig = async(config: LX.HotKeyConfigAll) => {
  await storeSet(STORE_NAMES.HOTKEY, config)
  emit('updateHotkey', config)
}
export const hotKeyGetStatus = async() => true

export const getEnvParams = async(): Promise<LX.EnvParams> => {
  return {
    cmdParams: {},
    deeplink: '',
  } as LX.EnvParams
}
export const clearEnvParamsDeeplink = () => {}
export const onDeeplink = (listener: Listener<string>): RemoveListener => on('deeplink', listener)

export const checkUpdate = () => {}
export const downloadUpdate = () => {}
export const quitUpdate = () => {}
export const onUpdateAvailable = (_listener: Listener<any>): RemoveListener => () => {}
export const onUpdateError = (_listener: Listener<string>): RemoveListener => () => {}
export const onUpdateProgress = (_listener: Listener<any>): RemoveListener => () => {}
export const onUpdateDownloaded = (_listener: Listener<any>): RemoveListener => () => {}
export const onUpdateNotAvailable = (_listener: Listener<any>): RemoveListener => () => {}

export const importUserApi = async(fileText: string) => {
  const info = parseUserApiScriptInfo(fileText)
  const apiInfo: UserApiInfo = {
    id: `user_api_${Math.random().toString().slice(2, 5)}_${Date.now()}`,
    ...info,
    script: fileText,
    allowShowUpdateAlert: true,
  }
  const list = await getUserApiStore()
  const next = [...list, apiInfo]
  await setUserApiStore(next)
  return {
    apiInfo: toUserApiInfo(apiInfo),
    apiList: next.map(toUserApiInfo),
  }
}
export const setUserApi = async(id: string): Promise<void> => {
  if (!/^user_api/.test(id)) {
    clearUserApiRuntime()
    return
  }
  const target = (await getUserApiStore()).find(item => item.id == id)
  if (!target) {
    emitUserApiStatus(false, '未找到自定义源', undefined)
    return
  }
  const setting = await getSetting()
  const proxy = setting['network.proxy.enable'] && setting['network.proxy.host']
    ? {
        host: setting['network.proxy.host'],
        port: setting['network.proxy.port'],
      }
    : { host: '', port: '' }
  await loadUserApiRuntime(target, proxy, (status) => {
    emitUserApiStatus(status.status, status.message, status.apiInfo)
  }, (info) => {
    emit('showUserApiUpdateAlert', info)
  })
}
export const removeUserApi = async(ids: string[]) => {
  const list = await getUserApiStore()
  const currentStatus = getUserApiRuntimeStatus()
  if (currentStatus.apiInfo?.id && ids.includes(currentStatus.apiInfo.id)) clearUserApiRuntime()
  const next = list.filter((item: any) => !ids.includes(item.id))
  await setUserApiStore(next)
  return next.map(toUserApiInfo)
}
export const onShowUserApiUpdateAlert = (listener: Listener<any>): RemoveListener => on('showUserApiUpdateAlert', listener)
export const setAllowShowUserApiUpdateAlert = async(id: string, enable: boolean): Promise<void> => {
  const list = await getUserApiStore()
  const target = list.find(item => item.id == id)
  if (!target) return
  target.allowShowUpdateAlert = enable
  await setUserApiStore(list)
}
export const onUserApiStatus = (listener: Listener<LX.UserApi.UserApiStatus>): RemoveListener => on('userApiStatus', listener)
export const getUserApiList = async() => (await getUserApiStore()).map(toUserApiInfo)
export const getUserApiStatus = async(): Promise<LX.UserApi.UserApiStatus> => getUserApiRuntimeStatus()
export const sendUserApiRequest = async(data: LX.UserApi.UserApiRequestParams) => requestUserApiRuntime(data)
export const userApiRequestCancel = (requestKey: string) => {
  cancelUserApiRuntimeRequest(requestKey)
}

export const sendPlayerStatus = (_status: Partial<LX.Player.Status>) => {}
export const onPlayerAction = (_listener: Listener<any>): RemoveListener => () => {}
export const setPlayerAction = (_buttons: LX.TaskBarButtonFlags) => {}
export const sendOpenAPIAction = async(_action: any) => ({ status: false })

export const saveLastStartInfo = (version: string) => { void dataStoreSet(DATA_KEYS.lastStartInfo, version) }
export const getLastStartInfo = async() => dataStoreGet<string | null>(DATA_KEYS.lastStartInfo, null)
export const savePlayInfo = (playInfo: LX.Player.SavedPlayInfo) => { void dataStoreSet(DATA_KEYS.playInfo, playInfo) }
export const getPlayInfo = async() => dataStoreGet<LX.Player.SavedPlayInfo | null>(DATA_KEYS.playInfo, null)
export const saveSearchHistoryList = (list: LX.List.SearchHistoryList) => { void dataStoreSet(DATA_KEYS.searchHistoryList, list) }
export const getSearchHistoryList = async() => dataStoreGet<string[] | null>(DATA_KEYS.searchHistoryList, null)
export const saveListPositionInfo = (listPosition: LX.List.ListPositionInfo) => { void dataStoreSet(DATA_KEYS.listScrollPosition, listPosition) }
export const getListPositionInfo = async() => dataStoreGet<LX.List.ListPositionInfo | null>(DATA_KEYS.listScrollPosition, null)
export const saveListPrevSelectId = (listPosition: string | null) => { void dataStoreSet(DATA_KEYS.listPrevSelectId, listPosition) }
export const getListPrevSelectId = async() => dataStoreGet<string | null>(DATA_KEYS.listPrevSelectId, null)
export const saveListUpdateInfo = (listPosition: LX.List.ListUpdateInfo) => { void dataStoreSet(DATA_KEYS.listUpdateInfo, listPosition) }
export const getListUpdateInfo = async() => dataStoreGet<LX.List.ListUpdateInfo | null>(DATA_KEYS.listUpdateInfo, null)
export const saveIgnoreVersion = (version: string) => { void dataStoreSet(DATA_KEYS.ignoreVersion, version) }
export const getIgnoreVersion = async() => dataStoreGet<string | null>(DATA_KEYS.ignoreVersion, null)
export const saveSearchSetting = (setting: typeof DEFAULT_SETTING.search) => { void dataStoreSet(DATA_KEYS.searchSetting, setting) }
export const getSearchSetting = async() => dataStoreGet(DATA_KEYS.searchSetting, DEFAULT_SETTING.search)
export const saveSongListSetting = (setting: typeof DEFAULT_SETTING.songList) => { void dataStoreSet(DATA_KEYS.songListSetting, setting) }
export const getSongListSetting = async() => dataStoreGet(DATA_KEYS.songListSetting, DEFAULT_SETTING.songList)
export const saveLeaderboardSetting = (setting: typeof DEFAULT_SETTING.leaderboard) => { void dataStoreSet(DATA_KEYS.leaderboardSetting, setting) }
export const getLeaderboardSetting = async() => dataStoreGet(DATA_KEYS.leaderboardSetting, DEFAULT_SETTING.leaderboard)
export const saveViewPrevState = (state: typeof DEFAULT_SETTING.viewPrevState) => { void dataStoreSet(DATA_KEYS.viewPrevState, state) }
export const getViewPrevState = async() => dataStoreGet(DATA_KEYS.viewPrevState, DEFAULT_SETTING.viewPrevState)

export const showSelectDialog = async(options: any) => {
  const result = await safeInvoke<any>('dialog_open', { options }, null)
  if (result == null) return { canceled: true, filePaths: [] }
  if (typeof result == 'string') return { canceled: false, filePaths: [result] }
  return { canceled: false, filePaths: result }
}
export const openSaveDir = async(options: any) => {
  const filePath = await safeInvoke<string | null>('dialog_save', { options }, null)
  return filePath ? { canceled: false, filePath } : { canceled: true, filePath: '' }
}
export const openDirInExplorer = async(path: string) => {
  await revealInExplorer(path)
}
export const getCacheSize = async() => 0
export const clearCache = async() => {}

export const minWindow = async() => { await safeInvoke('window_minimize', {}, undefined) }
export const maxWindow = async() => { await safeInvoke('window_toggle_maximize', {}, undefined) }
export const closeWindow = async() => { await safeInvoke('window_close', {}, undefined) }
export const focusWindow = async() => { await safeInvoke('window_focus', {}, undefined) }
export const setFullScreen = async(fullscreen: boolean) => { await safeInvoke('window_set_fullscreen', { fullscreen }, undefined) }
export const setWindowSize = async(width: number, height?: number) => {
  void width
  void height
}
export const quitApp = async() => { await safeInvoke('app_quit', {}, undefined) }
export const openDevTools = async() => {}
export const minMaxWindowToggle = async() => { await maxWindow() }
export const showHideWindowToggle = async() => {}
export const setIgnoreMouseEvents = (_ignore: boolean) => {}
export const setPowerSaveBlocker = async(_enable: boolean) => {}

export const getThemes = async() => {
  const userThemes = await storeGet<LX.Theme[]>(STORE_NAMES.THEME, [])
  return {
    themes: builtinThemes as LX.Theme[],
    userThemes,
    dataPath: '',
  }
}
export const saveTheme = async(theme: any) => {
  const { userThemes } = await getThemes()
  const next = [...userThemes.filter((item: any) => item.id != theme.id), theme]
  await storeSet(STORE_NAMES.THEME, next)
  return next
}
export const removeTheme = async(id: string) => {
  const { userThemes } = await getThemes()
  const next = userThemes.filter((item: any) => item.id != id)
  await storeSet(STORE_NAMES.THEME, next)
  return next
}
export const getSystemFonts = async() => ['Microsoft YaHei', 'Arial', 'Segoe UI', 'PingFang SC']
export const getUserSoundEffectEQPresetList = async() => {
  const store = await storeGet<{ eq?: any[] }>(STORE_NAMES.SOUND_EFFECT, {})
  return store.eq ?? []
}
export const saveUserSoundEffectEQPresetList = async(list: any[]) => {
  const store = await storeGet<Record<string, any>>(STORE_NAMES.SOUND_EFFECT, {})
  store.eq = list
  await storeSet(STORE_NAMES.SOUND_EFFECT, store)
}
export const getUserSoundEffectConvolutionPresetList = async() => {
  const store = await storeGet<{ convolution?: any[] }>(STORE_NAMES.SOUND_EFFECT, {})
  return store.convolution ?? []
}
export const saveUserSoundEffectConvolutionPresetList = async(list: any[]) => {
  const store = await storeGet<Record<string, any>>(STORE_NAMES.SOUND_EFFECT, {})
  store.convolution = list
  await storeSet(STORE_NAMES.SOUND_EFFECT, store)
}

export const sendSyncAction = async(_action: any) => ({ status: false })
export const onSyncAction = (_listener: Listener<any>): RemoveListener => () => {}
export const getSyncServerDevices = async() => []
export const removeSyncServerDevice = async() => {}

export const onFocus = (listener: Listener<void>): RemoveListener => {
  const handler = () => listener({ params: undefined as void })
  window.addEventListener('focus', handler)
  return () => window.removeEventListener('focus', handler)
}
export const onKeyDown = (_listener: Listener<{ key: string }>): RemoveListener => () => {}
export const onUpdateHotkey = (listener: Listener<LX.HotKeyConfigAll>): RemoveListener => on('updateHotkey', listener)

export const getOtherSource = async(_id: string): Promise<LX.Music.MusicInfoOnline[]> => []
export const saveOtherSource = async(_id: string, _sourceInfo: LX.Music.MusicInfoOnline[]) => {}
export const clearOtherSource = async() => {}
export const getOtherSourceCount = async() => 0
const getMusicId = (musicInfoOrId: any) => typeof musicInfoOrId == 'string' ? musicInfoOrId : musicInfoOrId?.id ?? ''
const getMusicUrlKey = (musicInfoOrId: any, type?: string) => typeof musicInfoOrId == 'string'
  ? musicInfoOrId
  : `${musicInfoOrId?.id ?? ''}_${type ?? ''}`

export const getPlayerLyric = async(musicInfo: LX.Music.MusicInfo): Promise<LX.Player.LyricInfo> => {
  const id = getMusicId(musicInfo)
  const rawStore = await storeGet<Record<string, any>>(STORE_NAMES.LRC_RAW, {})
  const editedStore = await storeGet<Record<string, any>>(STORE_NAMES.LRC_EDITED, {})
  const raw = rawStore[id]
  const edited = editedStore[id]
  if (edited) return { ...edited, rawlrcInfo: raw ?? edited }
  if (raw) return { ...raw, rawlrcInfo: raw }
  return { lyric: '', tlyric: '', rlyric: '', lxlyric: '', rawlrcInfo: { lyric: '', tlyric: '', rlyric: '', lxlyric: '' } }
}
export const getLyricRaw = async(musicInfo: LX.Music.MusicInfo) => {
  const rawStore = await storeGet<Record<string, any>>(STORE_NAMES.LRC_RAW, {})
  return rawStore[getMusicId(musicInfo)] ?? null
}
export const getLyricEdited = async(musicInfo: LX.Music.MusicInfo) => {
  const editedStore = await storeGet<Record<string, any>>(STORE_NAMES.LRC_EDITED, {})
  return editedStore[getMusicId(musicInfo)] ?? null
}
export const saveLyric = async(musicInfo: any, lyricInfo: any) => {
  const id = getMusicId(musicInfo)
  const rawStore = await storeGet<Record<string, any>>(STORE_NAMES.LRC_RAW, {})
  rawStore[id] = 'rawlrcInfo' in lyricInfo ? lyricInfo.rawlrcInfo : lyricInfo
  await storeSet(STORE_NAMES.LRC_RAW, rawStore)
  if ('rawlrcInfo' in lyricInfo && lyricInfo.lyric != lyricInfo.rawlrcInfo?.lyric) {
    const editedStore = await storeGet<Record<string, any>>(STORE_NAMES.LRC_EDITED, {})
    const { rawlrcInfo, ...edited } = lyricInfo
    editedStore[id] = edited
    await storeSet(STORE_NAMES.LRC_EDITED, editedStore)
  }
}
export const saveMusicUrl = async(musicInfo: any, type?: any, url?: any) => {
  const store = await storeGet<Record<string, string>>(MUSIC_URL_STORE, {})
  if (typeof musicInfo == 'object' && typeof type == 'string' && typeof url == 'string') {
    store[getMusicUrlKey(musicInfo, type)] = url
  } else if (musicInfo?.id && musicInfo?.url) {
    store[musicInfo.id] = musicInfo.url
  }
  await storeSet(MUSIC_URL_STORE, store)
}
export const getMusicUrl = async(musicInfo: any, type?: any): Promise<string> => {
  const store = await storeGet<Record<string, string>>(MUSIC_URL_STORE, {})
  return store[getMusicUrlKey(musicInfo, type)] ?? ''
}
export const saveLyricEdited = async(musicInfo: any, lyric: any) => {
  const id = getMusicId(musicInfo)
  const store = await storeGet<Record<string, any>>(STORE_NAMES.LRC_EDITED, {})
  store[id] = lyric
  await storeSet(STORE_NAMES.LRC_EDITED, store)
}
export const removeLyricEdited = async(musicInfo: any) => {
  const store = await storeGet<Record<string, any>>(STORE_NAMES.LRC_EDITED, {})
  delete store[getMusicId(musicInfo)]
  await storeSet(STORE_NAMES.LRC_EDITED, store)
}
export const clearLyric = async() => { await storeSet(STORE_NAMES.LRC_RAW, {}) }
export const clearLyricRaw = clearLyric
export const clearLyricEdited = async() => { await storeSet(STORE_NAMES.LRC_EDITED, {}) }
export const getLyricRawCount = async() => Object.keys(await storeGet<Record<string, any>>(STORE_NAMES.LRC_RAW, {})).length
export const getLyricEditedCount = async() => Object.keys(await storeGet<Record<string, any>>(STORE_NAMES.LRC_EDITED, {})).length
export const clearMusicUrl = async() => { await storeSet(MUSIC_URL_STORE, {}) }
export const getMusicUrlCount = async() => Object.keys(await storeGet<Record<string, string>>(MUSIC_URL_STORE, {})).length
export const downloadTasksGet = async(): Promise<LX.Download.ListItem[]> => {
  return storeGet<LX.Download.ListItem[]>(DOWNLOAD_TASK_STORE, [])
}
export const downloadTasksCreate = async(list: LX.Download.ListItem[], addMusicLocationType: LX.AddMusicLocationType) => {
  const current = await downloadTasksGet()
  const next = addMusicLocationType == 'top' ? [...list, ...current] : [...current, ...list]
  await storeSet(DOWNLOAD_TASK_STORE, next)
}
export const downloadTasksUpdate = async(list: LX.Download.ListItem[]) => {
  const current = await downloadTasksGet()
  const map = new Map(current.map(item => [item.id, item]))
  for (const item of list) map.set(item.id, item)
  await storeSet(DOWNLOAD_TASK_STORE, Array.from(map.values()))
}
export const downloadTasksRemove = async(ids: string[]) => {
  const current = await downloadTasksGet()
  await storeSet(DOWNLOAD_TASK_STORE, current.filter(item => !ids.includes(item.id)))
}
export const downloadListClear = async() => {
  await storeSet(DOWNLOAD_TASK_STORE, [])
}
export const onNewDesktopLyricProcess = (_listener: Listener<any>): RemoveListener => () => {}
