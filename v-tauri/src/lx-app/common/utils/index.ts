import log from 'electron-log/node'

const getPlatformFromBrowser = (): NodeJS.Platform => {
  const ua = navigator.userAgent.toLowerCase()
  if (ua.includes('win')) return 'win32'
  if (ua.includes('mac')) return 'darwin'
  return 'linux'
}
const currentPlatform = typeof process != 'undefined' && process.platform ? process.platform : getPlatformFromBrowser()

export const isLinux = currentPlatform == 'linux'
export const isWin = currentPlatform == 'win32'
export const isMac = currentPlatform == 'darwin'
export const isProd = import.meta.env.PROD

export const getPlatform = (platform: NodeJS.Platform = currentPlatform) => {
  switch (platform) {
    case 'win32': return 'windows'
    case 'darwin': return 'mac'
    default: return 'linux'
  }
}


// https://stackoverflow.com/a/53387532
export function compareVer(currentVer: string, targetVer: string): -1 | 0 | 1 {
  // treat non-numerical characters as lower version
  // replacing them with a negative number based on charcode of each character
  const fix = (s: string) => `.${s.toLowerCase().charCodeAt(0) - 2147483647}.`

  const currentVerArr: Array<string | number> = ('' + currentVer).replace(/[^0-9.]/g, fix).split('.')
  const targetVerArr: Array<string | number> = ('' + targetVer).replace(/[^0-9.]/g, fix).split('.')
  let c = Math.max(currentVerArr.length, targetVerArr.length)
  for (let i = 0; i < c; i++) {
    // convert to integer the most efficient way
    currentVerArr[i] = ~~currentVerArr[i]
    targetVerArr[i] = ~~targetVerArr[i]
    if (currentVerArr[i] > targetVerArr[i]) return 1
    else if (currentVerArr[i] < targetVerArr[i]) return -1
  }
  return 0
}


export {
  log,
}

export * from './common'
