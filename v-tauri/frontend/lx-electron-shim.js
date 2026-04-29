(() => {
  const t = window.__TAURI__
  if (!t) return

  const ensureProcess = () => {
    if (!window.process) window.process = {}
    if (!window.process.env) window.process.env = {}
    if (!('NODE_ENV' in window.process.env)) window.process.env.NODE_ENV = 'production'
  }

  ensureProcess()

  const ipcRenderer = {
    send(channel, params) {
      return t.invoke('lx_ipc_send', { channel, params }).catch(() => {})
    },
    invoke(channel, params) {
      return t.invoke('lx_ipc_invoke', { channel, params })
    },
    on() {},
    once() {},
    removeListener() {},
    removeAllListeners() {},
  }

  const shell = {
    openExternal(url) {
      return t.shell?.open?.(url)
    },
  }

  const clipboard = {
    writeText(text) {
      return t.clipboard?.writeText?.(text)
    },
    readText() {
      return t.clipboard?.readText?.()
    },
  }

  window.electron = { ipcRenderer, shell, clipboard }
})()

