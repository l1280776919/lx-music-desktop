const isDev = import.meta.env.DEV

export const debug = isDev && true
export const debugRequest = isDev && false
export const debugDownload = isDev && false
