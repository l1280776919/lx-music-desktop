// import defaultSetting from '@common/defaultSetting'
import createWorkers from '@renderer/worker'

window.lx = {
  // appSetting: defaultSetting,
  isEditingHotKey: false,
  isPlayedStop: false,
  appHotKeyConfig: {
    local: {
      enable: false,
      keys: {},
    },
    global: {
      enable: false,
      keys: {},
    },
  },
  songListInfo: {
    fromName: '',
    searchKey: '',
    searchPosition: 0,
    songlistKey: '',
    songlistPosition: 0,
  },
  restorePlayInfo: null,
  worker: createWorkers(),
  isProd: import.meta.env.PROD,
  rootOffset: window.dt ? 0 : 8,
  apiInitPromise: [Promise.resolve(false), true, () => {}],
}

window.lxData = {}

window.ELECTRON_DISABLE_SECURITY_WARNINGS = 'true'
