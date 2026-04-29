const getTauri = () => {
  const t = globalThis.__TAURI__
  if (!t) throw new Error('__TAURI__ not found')
  return t
}

export const ipcRenderer = {
  send(channel, params) {
    const t = getTauri()
    return t.invoke('lx_ipc_send', { channel, params }).catch(() => {})
  },
  invoke(channel, params) {
    const t = getTauri()
    return t.invoke('lx_ipc_invoke', { channel, params })
  },
  on() {},
  once() {},
  removeListener() {},
  removeAllListeners() {},
}

export const shell = {
  showItemInFolder() {},
  openExternal(url) {
    const t = getTauri()
    return t.shell?.open?.(url)
  },
}

export const clipboard = {
  writeText(text) {
    const t = getTauri()
    return t.clipboard?.writeText?.(text)
  },
  readText() {
    const t = getTauri()
    return t.clipboard?.readText?.()
  },
}

