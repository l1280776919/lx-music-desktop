const splitPath = (p = '') => String(p).replace(/\\/g, '/').split('/')

export const join = (...parts) => {
  const cleaned = parts.filter(Boolean).map(p => String(p).replace(/\\/g, '/'))
  return cleaned.join('/').replace(/\/+/g, '/')
}

export const extname = (p = '') => {
  const name = basename(p)
  const i = name.lastIndexOf('.')
  return i >= 0 ? name.slice(i) : ''
}

export const basename = (p = '', ext) => {
  const parts = splitPath(p)
  let name = parts[parts.length - 1] || ''
  if (ext && name.endsWith(ext)) name = name.slice(0, -ext.length)
  return name
}

export const dirname = (p = '') => {
  const parts = splitPath(p)
  parts.pop()
  return parts.join('/') || '/'
}

export default {
  join,
  extname,
  basename,
  dirname,
}

