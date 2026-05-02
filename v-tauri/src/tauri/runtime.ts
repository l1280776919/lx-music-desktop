import { invoke as tauriInvoke } from '@tauri-apps/api/core'

export const hasTauriContext = () => {
  return typeof window != 'undefined' &&
    typeof (window as any).__TAURI_INTERNALS__?.invoke == 'function'
}

export async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>, fallback?: T): Promise<T> {
  if (hasTauriContext()) return tauriInvoke<T>(cmd, args as any)
  if (arguments.length >= 3) return fallback as T
  throw new Error(`Tauri invoke unavailable: ${cmd}`)
}
