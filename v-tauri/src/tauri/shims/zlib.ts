import { deflateRaw as pakoDeflateRaw, inflate as pakoInflate } from 'pako'

export const deflateRaw = (data: string | Uint8Array, callback: (err: Error | null, result?: Uint8Array) => void) => {
  try {
    callback(null, pakoDeflateRaw(data))
  } catch (error) {
    callback(error as Error)
  }
}

export const inflate = (data: Uint8Array, callback: (err: Error | null, result?: Uint8Array) => void) => {
  try {
    callback(null, pakoInflate(data))
  } catch (error) {
    callback(error as Error)
  }
}
