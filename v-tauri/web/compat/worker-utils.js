export const createMainWorker = () => {
  return {
    langS2t: async(b64) => b64,
    langT2s: async(b64) => b64,
  }
}

export const createDownloadWorker = () => {
  return {}
}

export const proxyCallback = (callback) => callback

