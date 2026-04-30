import pkg from '../../package.json'
import { createWorkers } from './workers'

const formatLang = (lang = 'en') => {
  if (lang == 'zh-cn') return 'zh-Hans'
  if (lang == 'zh-tw') return 'zh-Hant'
  return lang.split('-')[0]
}

const getPlatform = () => {
  const ua = navigator.userAgent.toLowerCase()
  if (ua.includes('mac')) return 'darwin'
  if (ua.includes('win')) return 'win32'
  return 'linux'
}

const getArch = () => {
  const ua = navigator.userAgent.toLowerCase()
  if (/arm64|aarch64/.test(ua)) return 'arm64'
  if (/arm/.test(ua)) return 'arm'
  if (/x64|win64|wow64|x86_64|amd64/.test(ua)) return 'x64'
  return 'ia32'
}

const ensureProcessInfo = () => {
  const globalProcess = (globalThis as typeof globalThis & {
    process?: {
      arch?: string
      platform?: string
      versions?: Record<string, string>
    }
  }).process ?? {}

  globalThis.process = Object.assign(globalProcess, {
    arch: globalProcess.arch ?? getArch(),
    platform: globalProcess.platform ?? getPlatform(),
    versions: {
      ...globalProcess.versions,
      app: globalProcess.versions?.app ?? pkg.version,
    },
  })
}

ensureProcessInfo()

const searchParams = new URLSearchParams(window.location.search)
const themeStyle = document.createElement('style')
const hasTauriWindow = typeof window.__TAURI_INTERNALS__?.invoke == 'function'

window.setLang = (lang = navigator.language.toLocaleLowerCase()) => {
  document.documentElement.setAttribute('lang', formatLang(lang))
}

window.dt = searchParams.get('dt') == 'true' || hasTauriWindow
window.shouldUseDarkColors = searchParams.get('dark') == 'true' ||
  window.matchMedia?.('(prefers-color-scheme: dark)').matches == true
window.setTheme = (colors) => {
  themeStyle.innerText = `:root {${Object.entries(colors).map(([key, value]) => `${key}:${value};`).join('')}}`
  if (!themeStyle.isConnected) document.body.appendChild(themeStyle)
}

window.setLang()
document.documentElement.classList.add(window.dt ? 'disableTransparent' : 'transparent')

const rawTheme = searchParams.get('theme')
if (rawTheme) {
  try {
    const theme = JSON.parse(decodeURIComponent(rawTheme))
    window.setTheme(theme.colors ?? {})
  } catch {}
}

window.lx = {
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
  rootOffset: 0,
  apiInitPromise: [Promise.resolve(false), true, () => {}],
}

window.lxData = {}
