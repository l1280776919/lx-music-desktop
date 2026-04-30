import { DISLIKE_EVENT_NAME, PLAYER_EVENT_NAME, WIN_MAIN_RENDERER_EVENT_NAME } from '@common/ipcNames'
import iconv from 'iconv-lite'
import { Buffer } from 'buffer'
import { inflate as pakoInflate } from 'pako'
import {
  dispatchPlayerAction as dispatchTauriPlayerAction,
  onPlayerAction as onTauriPlayerAction,
  sendPlayerStatus as sendTauriPlayerStatus,
  setPlayerAction as setTauriPlayerAction,
} from './ipc'
import { hasTauriContext, safeInvoke } from './runtime'

type Listener = (...args: any[]) => any

const listeners = new Map<string, Set<Listener>>()
const listenerWrappers = new WeakMap<(...args: any[]) => any, Listener>()
const on = (name: string, listener: Listener) => {
  const set = listeners.get(name) ?? new Set<Listener>()
  set.add(listener)
  listeners.set(name, set)
}
const off = (name: string, listener: Listener) => {
  listeners.get(name)?.delete(listener)
}
const emit = (name: string, params?: any) => {
  const event = { ports: [] } as Electron.IpcRendererEvent
  listeners.get(name)?.forEach(listener => listener(event, params))
}
onTauriPlayerAction(({ params }) => {
  emit(WIN_MAIN_RENDERER_EVENT_NAME.player_action_on_button_click, params)
})

const LIST_STORE = 'tauri_list_data'
const DISLIKE_STORE = 'tauri_dislike_data'
const browserStoreKey = (name: string) => `lx-tauri:${name}`

const kwLyricKey = Buffer.from('yeelion')

const decodeKwLyric = (lrcBase64: string, isGetLyricx: boolean) => {
  const buf = Buffer.from(lrcBase64, 'base64')
  if (buf.toString('utf8', 0, 10) != 'tp=content') return ''

  const headerEnd = buf.indexOf('\r\n\r\n')
  if (headerEnd < 0) return ''

  const lrcData = Buffer.from(pakoInflate(buf.subarray(headerEnd + 4)))
  if (!isGetLyricx) return iconv.decode(lrcData, 'gb18030')

  const encrypted = Buffer.from(lrcData.toString(), 'base64')
  const output = new Uint8Array(encrypted.length)
  for (let i = 0; i < encrypted.length; i++) {
    output[i] = encrypted[i] ^ kwLyricKey[i % kwLyricKey.length]
  }
  return iconv.decode(Buffer.from(output), 'gb18030')
}

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

type ListStore = {
  defaultList: LX.Music.MusicInfo[]
  loveList: LX.Music.MusicInfo[]
  tempList: LX.Music.MusicInfo[]
  userLists: LX.List.UserListInfo[]
  userListMusics: Record<string, LX.Music.MusicInfo[]>
}

const getListStore = async(): Promise<ListStore> => {
  return storeGet<ListStore>(LIST_STORE, {
    defaultList: [],
    loveList: [],
    tempList: [],
    userLists: [],
    userListMusics: {},
  })
}

const setListStore = async(store: ListStore) => {
  await storeSet(LIST_STORE, store)
}

type DislikeStore = {
  rules: string
}

const getDislikeStore = async(): Promise<DislikeStore> => {
  return storeGet<DislikeStore>(DISLIKE_STORE, { rules: '' })
}

const buildDislikeInfo = (rules: string): LX.Dislike.DislikeInfo => {
  const musicNames = new Set<string>()
  const singerNames = new Set<string>()
  const names = new Set<string>()
  for (const item of rules.split('\n')) {
    if (!item) continue
    const [name = '', singer = ''] = item.split('@')
    if (name && singer) names.add(`${name}@${singer}`)
    else if (name) musicNames.add(name)
    else if (singer) singerNames.add(singer)
  }
  return { rules, musicNames, singerNames, names }
}

export function rendererSend(name: string): void
export function rendererSend<T>(name: string, params: T): void
export function rendererSend<T>(name: string, params?: T): void {
  void rendererInvoke(name, params)
}

export function rendererSendSync(name: string): void
export function rendererSendSync<T>(name: string, params: T): void
export function rendererSendSync<T>(name: string, params?: T): void {
  void rendererInvoke(name, params)
}

export async function rendererInvoke(name: string): Promise<void>
export async function rendererInvoke<V>(name: string): Promise<V>
export async function rendererInvoke<T>(name: string, params: T): Promise<void>
export async function rendererInvoke<T, V>(name: string, params: T): Promise<V>
export async function rendererInvoke<T, V>(name: string, params?: T): Promise<V> {
  switch (name) {
    case PLAYER_EVENT_NAME.list_get: {
      const store = await getListStore()
      return store.userLists as V
    }
    case PLAYER_EVENT_NAME.list_add: {
      const store = await getListStore()
      const data = params as any
      const { position, listInfos } = data
      const items = [...listInfos]
      store.userLists.splice(position, 0, ...items)
      for (const item of items) store.userListMusics[item.id] ??= []
      await setListStore(store)
      emit(PLAYER_EVENT_NAME.list_add, data)
      return undefined as V
    }
    case PLAYER_EVENT_NAME.list_remove: {
      const store = await getListStore()
      const ids = params as string[]
      store.userLists = store.userLists.filter(item => !ids.includes(item.id))
      for (const id of ids) delete store.userListMusics[id]
      await setListStore(store)
      emit(PLAYER_EVENT_NAME.list_remove, ids)
      return undefined as V
    }
    case PLAYER_EVENT_NAME.list_update: {
      const store = await getListStore()
      const listInfos = params as LX.List.UserListInfo[]
      for (const info of listInfos) {
        const target = store.userLists.find(item => item.id == info.id)
        if (target) Object.assign(target, info)
      }
      await setListStore(store)
      emit(PLAYER_EVENT_NAME.list_update, listInfos)
      return undefined as V
    }
    case PLAYER_EVENT_NAME.list_update_position: {
      const store = await getListStore()
      const { position, ids } = params as any
      const moving = store.userLists.filter(item => ids.includes(item.id))
      store.userLists = store.userLists.filter(item => !ids.includes(item.id))
      store.userLists.splice(position, 0, ...moving)
      await setListStore(store)
      emit(PLAYER_EVENT_NAME.list_update_position, params)
      return undefined as V
    }
    case PLAYER_EVENT_NAME.list_music_get: {
      const store = await getListStore()
      const id = params as string
      if (id == 'default') return store.defaultList as V
      if (id == 'love') return store.loveList as V
      if (id == 'temp') return store.tempList as V
      return (store.userListMusics[id] ?? []) as V
    }
    case PLAYER_EVENT_NAME.list_music_add: {
      const store = await getListStore()
      const { id, musicInfos } = params as any
      const list = store.userListMusics[id] ?? []
      list.push(...musicInfos)
      store.userListMusics[id] = list
      await setListStore(store)
      emit(PLAYER_EVENT_NAME.list_music_add, params)
      return undefined as V
    }
    case PLAYER_EVENT_NAME.list_music_move: {
      const store = await getListStore()
      const { fromId, toId, musicInfos } = params as any
      const moveIds = new Set(musicInfos.map((item: any) => item.id))
      store.userListMusics[fromId] = (store.userListMusics[fromId] ?? []).filter(item => !moveIds.has(item.id))
      store.userListMusics[toId] = [...(store.userListMusics[toId] ?? []), ...musicInfos]
      await setListStore(store)
      emit(PLAYER_EVENT_NAME.list_music_move, params)
      return undefined as V
    }
    case PLAYER_EVENT_NAME.list_music_remove: {
      const store = await getListStore()
      const { listId, ids } = params as any
      const idSet = new Set(ids)
      store.userListMusics[listId] = (store.userListMusics[listId] ?? []).filter(item => !idSet.has(item.id))
      await setListStore(store)
      emit(PLAYER_EVENT_NAME.list_music_remove, params)
      return undefined as V
    }
    case PLAYER_EVENT_NAME.list_music_update: {
      return undefined as V
    }
    case PLAYER_EVENT_NAME.list_music_update_position: {
      return undefined as V
    }
    case PLAYER_EVENT_NAME.list_music_overwrite: {
      const store = await getListStore()
      const { listId, musicInfos } = params as any
      if (listId == 'default') store.defaultList = musicInfos
      else if (listId == 'love') store.loveList = musicInfos
      else if (listId == 'temp') store.tempList = musicInfos
      else store.userListMusics[listId] = musicInfos
      await setListStore(store)
      emit(PLAYER_EVENT_NAME.list_music_overwrite, params)
      return undefined as V
    }
    case PLAYER_EVENT_NAME.list_music_clear: {
      const store = await getListStore()
      for (const id of params as string[]) {
        if (id == 'default') store.defaultList = []
        else if (id == 'love') store.loveList = []
        else if (id == 'temp') store.tempList = []
        else store.userListMusics[id] = []
      }
      await setListStore(store)
      emit(PLAYER_EVENT_NAME.list_music_clear, params)
      return undefined as V
    }
    case PLAYER_EVENT_NAME.list_data_overwire: {
      const storeData = params as any
      const store: ListStore = {
        defaultList: storeData.defaultList,
        loveList: storeData.loveList,
        tempList: storeData.tempList ?? [],
        userLists: storeData.userList.map((item: any) => {
          const { list, ...info } = item
          return info
        }),
        userListMusics: Object.fromEntries(storeData.userList.map((item: any) => [item.id, item.list])),
      }
      await setListStore(store)
      emit(PLAYER_EVENT_NAME.list_data_overwire, params)
      return undefined as V
    }
    case PLAYER_EVENT_NAME.list_music_check_exist: {
      const store = await getListStore()
      const { listId, musicInfoId } = params as any
      return (store.userListMusics[listId] ?? []).some(item => item.id == musicInfoId) as V
    }
    case PLAYER_EVENT_NAME.list_music_get_list_ids: {
      const store = await getListStore()
      const musicInfoId = params as string
      return Object.entries(store.userListMusics)
        .filter(([, list]) => list.some(item => item.id == musicInfoId))
        .map(([id]) => id) as V
    }
    case DISLIKE_EVENT_NAME.get_dislike_music_infos: {
      const store = await getDislikeStore()
      return buildDislikeInfo(store.rules) as V
    }
    case DISLIKE_EVENT_NAME.add_dislike_music_infos: {
      const store = await getDislikeStore()
      const infos = params as any[]
      const extra = infos.map(info => `${info.name ?? ''}@${info.singer ?? ''}`).join('\n')
      store.rules = [store.rules, extra].filter(Boolean).join('\n')
      await storeSet(DISLIKE_STORE, store)
      emit(DISLIKE_EVENT_NAME.add_dislike_music_infos, infos)
      return undefined as V
    }
    case DISLIKE_EVENT_NAME.overwrite_dislike_music_infos: {
      const store = { rules: params as string }
      await storeSet(DISLIKE_STORE, store)
      emit(DISLIKE_EVENT_NAME.overwrite_dislike_music_infos, params)
      return undefined as V
    }
    case DISLIKE_EVENT_NAME.clear_dislike_music_infos: {
      await storeSet(DISLIKE_STORE, { rules: '' })
      emit(DISLIKE_EVENT_NAME.clear_dislike_music_infos)
      return undefined as V
    }
    case WIN_MAIN_RENDERER_EVENT_NAME.handle_kw_decode_lyric: {
      const { lrcBase64, isGetLyricx } = params as { lrcBase64: string, isGetLyricx: boolean }
      return Buffer.from(decodeKwLyric(lrcBase64, isGetLyricx)).toString('base64') as V
    }
    case WIN_MAIN_RENDERER_EVENT_NAME.handle_tx_decode_lyric: {
      return { lyric: '', tlyric: '', rlyric: '' } as V
    }
    case WIN_MAIN_RENDERER_EVENT_NAME.player_status: {
      await sendTauriPlayerStatus(params as Partial<LX.Player.Status>)
      return undefined as V
    }
    case WIN_MAIN_RENDERER_EVENT_NAME.player_action_set_buttons: {
      await setTauriPlayerAction(params as LX.TaskBarButtonFlags)
      return undefined as V
    }
    case WIN_MAIN_RENDERER_EVENT_NAME.player_action_dispatch: {
      const { action, data } = params as { action: LX.Player.StatusButtonActions, data?: unknown }
      await dispatchTauriPlayerAction(action, data)
      return undefined as V
    }
    default:
      return undefined as V
  }
}

export function rendererOn(name: string, listener: LX.IpcRendererEventListener): void
export function rendererOn<T>(name: string, listener: LX.IpcRendererEventListenerParams<T>): void
export function rendererOn<T>(name: string, listener: LX.IpcRendererEventListenerParams<T>): void {
  const wrapped = (event: Electron.IpcRendererEvent, params: T) => {
    listener({ event, params })
  }
  listenerWrappers.set(listener as (...args: any[]) => any, wrapped)
  on(name, wrapped)
}

export function rendererOnce(name: string, listener: LX.IpcRendererEventListener): void
export function rendererOnce<T>(name: string, listener: LX.IpcRendererEventListenerParams<T>): void
export function rendererOnce<T>(name: string, listener: LX.IpcRendererEventListenerParams<T>): void {
  const wrapped = (event: Electron.IpcRendererEvent, params: T) => {
    off(name, wrapped)
    listener({ event, params })
  }
  listenerWrappers.set(listener as (...args: any[]) => any, wrapped)
  on(name, wrapped)
}

export const rendererOff = (name: string, listener: (...args: any[]) => any) => {
  off(name, listenerWrappers.get(listener) ?? listener)
  listenerWrappers.delete(listener)
}

export const rendererOffAll = (name: string) => {
  listeners.delete(name)
}
