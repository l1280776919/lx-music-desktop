import path from 'path-browserify'
import { gzip, ungzip } from 'pako'
import CryptoJS from 'crypto-js'
import { hasTauriContext, safeInvoke } from './runtime'

export const joinPath = (...paths: string[]): string => path.join(...paths)
export const extname = (p: string): string => path.extname(p)
export const basename = (p: string, ext?: string): string => path.basename(p, ext)
export const dirname = (p: string): string => path.dirname(p)

export const checkPath = async(path: string): Promise<boolean> => {
  if (!path) return false
  return safeInvoke<boolean>('fs_exists', { path }, false)
}

export const checkAndCreateDir = async(path: string) => {
  if (!path) return false
  return safeInvoke<boolean>('fs_create_dir_if_missing', { path }, false)
}

export const createDir = async(path: string) => {
  if (!path) return
  await safeInvoke('fs_create_dir', { path }, undefined)
}

export const removeFile = async(path: string) => {
  if (!path) return
  await safeInvoke('fs_remove_file', { path }, undefined)
}

export const readFile = async(path: string) => {
  if (!path) return Buffer.alloc(0)
  const data = await safeInvoke<number[]>('fs_read_binary', { path }, [])
  return Buffer.from(data)
}

export const getFileStats = async(path: string): Promise<{ size: number } | null> => {
  return safeInvoke<{ size: number } | null>('fs_metadata', { path }, null)
}

export const saveLxConfigFile = async(path: string, data: any) => {
  const filePath = path.endsWith('.lxmc') ? path : `${path}.lxmc`
  const text = JSON.stringify(data)
  const compressed = await gzipData(text)
  await safeInvoke('fs_write_binary', { path: filePath, data: Array.from(compressed) }, undefined)
}

export const readLxConfigFile = async(path: string): Promise<any> => {
  if (!path) return null
  const isJSON = path.endsWith('.json')
  if (isJSON) {
    const text = await safeInvoke<string>('fs_read_text', { path }, '')
    return JSON.parse(text)
  }
  const data = await safeInvoke<number[]>('fs_read_binary', { path }, [])
  const text = await gunzipData(Buffer.from(data))
  let parsed = JSON.parse(text)
  if (typeof parsed != 'object') parsed = JSON.parse(parsed)
  return parsed
}

export const gzipData = async(str: string): Promise<Buffer> => {
  return Buffer.from(gzip(str))
}

export const gunzipData = async(buf: Buffer | Uint8Array): Promise<string> => {
  return new TextDecoder().decode(ungzip(Uint8Array.from(buf)))
}

export const saveStrToFile = async(path: string, str: string | Buffer | Uint8Array): Promise<void> => {
  if (!path || !hasTauriContext()) return
  if (typeof str == 'string') {
    await safeInvoke('fs_write_text', { path, text: str }, undefined)
    return
  }
  const data = str instanceof Uint8Array ? str : Uint8Array.from(str)
  await safeInvoke('fs_write_binary', { path, data: Array.from(data) }, undefined)
}

export const b64DecodeUnicode = (str: string): string => {
  return Buffer.from(str, 'base64').toString()
}

export const toMD5 = (str: string) => {
  return CryptoJS.MD5(str).toString()
}

export const copyFile = async(sourcePath: string, distPath: string) => {
  if (!sourcePath || !distPath) return
  await safeInvoke('fs_copy_file', { sourcePath, distPath }, undefined)
}

export const moveFile = async(sourcePath: string, distPath: string) => {
  if (!sourcePath || !distPath) return
  await safeInvoke('fs_move_file', { sourcePath, distPath }, undefined)
}

export const getAddress = (): string[] => {
  return []
}
