export type MainTypes = any
export type DownloadTypes = any

export const createMainWorker = () => window.lx.worker.main
export const createDownloadWorker = () => window.lx.worker.download

export const proxyCallback = <Args extends any[]>(callback: (...args: Args) => void) => {
  return callback
}
