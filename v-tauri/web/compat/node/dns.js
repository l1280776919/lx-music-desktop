export const lookup = (hostname, options, callback) => {
  const cb = typeof options === 'function' ? options : callback
  if (typeof cb === 'function') cb(new Error('dns.lookup is not supported in web build'))
}

export default {
  lookup,
}

