import { appLogger } from '../../../../tauri/logger'

type DiagnosticMusicInfo = LX.Music.MusicInfoOnline | LX.Music.MusicInfoLocal | LX.Download.ListItem | null | undefined

const normalizeText = (value: unknown) => {
  if (value == null) return ''
  return String(value).replaceAll(/\s+/g, ' ').trim()
}

const serializeMusicInfo = (musicInfo: DiagnosticMusicInfo) => {
  if (!musicInfo) return null
  if ('progress' in musicInfo) {
    const info = musicInfo.metadata.musicInfo
    return {
      id: musicInfo.id,
      source: 'local',
      name: info.name,
      singer: info.singer,
      interval: info.interval ?? '',
      type: 'download',
    }
  }
  return {
    id: musicInfo.id,
    source: musicInfo.source,
    name: musicInfo.name,
    singer: musicInfo.singer,
    interval: musicInfo.interval ?? '',
    type: musicInfo.source == 'local' ? 'local' : 'online',
  }
}

const serializeError = (error: unknown) => {
  if (error instanceof Error) {
    return {
      name: error.name,
      message: normalizeText(error.message),
    }
  }
  if (typeof error == 'string') return { message: normalizeText(error) }
  if (error && typeof error == 'object') {
    const target = error as Record<string, unknown>
    return {
      name: normalizeText(target.name),
      message: normalizeText(target.message),
      code: normalizeText(target.code),
    }
  }
  return { message: normalizeText(error) }
}

const compact = (data: Record<string, unknown>) => {
  return Object.fromEntries(Object.entries(data).filter(([, value]) => {
    if (value == null) return false
    if (Array.isArray(value)) return value.length > 0
    if (typeof value == 'string') return value.length > 0
    return true
  }))
}

export const logMusicDiagnostic = (event: string, payload: {
  musicInfo?: DiagnosticMusicInfo
  targetMusicInfo?: DiagnosticMusicInfo
  quality?: string
  isRefresh?: boolean
  allowToggleSource?: boolean
  retryedSource?: string[]
  otherSourceCount?: number
  stage?: string
  extra?: Record<string, unknown>
  error?: unknown
}) => {
    const pad = (n: number) => n.toString().padStart(2, '0')
    const ms = (n: number) => n.toString().padStart(3, '0')
    const d = new Date()
    const time = `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}.${ms(d.getMilliseconds())}`
    appLogger.warn(event, compact({
      time,
    event,
    stage: payload.stage,
    quality: payload.quality,
    isRefresh: payload.isRefresh,
    allowToggleSource: payload.allowToggleSource,
    retryedSource: payload.retryedSource,
    otherSourceCount: payload.otherSourceCount,
    musicInfo: serializeMusicInfo(payload.musicInfo),
    targetMusicInfo: serializeMusicInfo(payload.targetMusicInfo),
    extra: payload.extra ? compact(payload.extra) : undefined,
    error: payload.error ? serializeError(payload.error) : undefined,
  }), 'frontend.music')
}

export const hasLyricContent = (lyricInfo: Partial<LX.Music.LyricInfo> | Partial<LX.Player.LyricInfo> | null | undefined) => {
  if (!lyricInfo) return false
  return !!normalizeText(lyricInfo.lyric) ||
    !!normalizeText(lyricInfo.tlyric) ||
    !!normalizeText(lyricInfo.rlyric) ||
    !!normalizeText(lyricInfo.lxlyric)
}
