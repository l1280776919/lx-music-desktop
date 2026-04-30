import { openUrl as tauriOpenUrl } from '@tauri-apps/plugin-opener'
import { hasTauriContext, safeInvoke } from './runtime'

let clipboardTextCache = ''
const hasClipboardApi = () => typeof navigator != 'undefined' && !!navigator.clipboard

export const openDirInExplorer = async(dir: string) => {
  await safeInvoke('open_in_explorer', { path: dir }, undefined)
}

export const openUrl = async(url: string) => {
  if (!/^https?:\/\//.test(url)) return
  if (hasTauriContext()) {
    try {
      await tauriOpenUrl(url)
      return
    } catch {}
  }
  if (typeof window != 'undefined' && typeof window.open == 'function') {
    window.open(url, '_blank', 'noopener,noreferrer')
  }
}

export const clipboardWriteText = (str: string) => {
  clipboardTextCache = str
  if (!hasClipboardApi()) return
  void navigator.clipboard.writeText(str).catch(() => {})
}

export const clipboardReadText = (): string => {
  if (!hasClipboardApi()) return clipboardTextCache
  void navigator.clipboard.readText().then(text => {
    clipboardTextCache = text
  }).catch(() => {})
  return clipboardTextCache
}

export const encodePath = (path: string) => {
  return path.replaceAll('%', '%25').replaceAll('#', '%23')
}
