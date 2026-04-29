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
  sendSync() {},
  invoke(channel, params) {
    const t = getTauri()
    return t.invoke('lx_ipc_invoke', { channel, params }).then((result) => {
      if (typeof channel === 'string' && channel.endsWith('get_dislike_music_infos')) {
        const rules = String(result?.rules ?? '')
        const DISLIKE_NAME = '@'
        const DISLIKE_NAME_ALIAS = '#'

        const names = new Set()
        const musicNames = new Set()
        const singerNames = new Set()

        for (const line of rules.split('\n')) {
          if (!line) continue
          let [name, singer] = line.split(DISLIKE_NAME)
          if (name) {
            name = name.replaceAll(DISLIKE_NAME, DISLIKE_NAME_ALIAS).toLocaleLowerCase().trim()
            if (singer) {
              singer = singer.replaceAll(DISLIKE_NAME, DISLIKE_NAME_ALIAS).toLocaleLowerCase().trim()
              names.add(`${name}${DISLIKE_NAME}${singer}`)
            } else {
              musicNames.add(name)
            }
          } else if (singer) {
            singer = singer.replaceAll(DISLIKE_NAME, DISLIKE_NAME_ALIAS).toLocaleLowerCase().trim()
            singerNames.add(singer)
          }
        }

        return {
          rules,
          names,
          musicNames,
          singerNames,
        }
      }
      return result
    })
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
