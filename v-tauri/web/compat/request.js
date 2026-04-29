const normalizeHeaders = (headers = {}) => {
  const h = {}
  for (const [k, v] of Object.entries(headers)) {
    if (v == null) continue
    h[k] = String(v)
  }
  return h
}

const doFetch = async(url, { method = 'GET', headers = {}, body, timeout = 15000 } = {}, cb) => {
  const controller = new AbortController()
  const id = setTimeout(() => controller.abort(), timeout)
  const init = {
    method: method.toUpperCase(),
    headers: normalizeHeaders(headers),
    signal: controller.signal,
  }
  if (body != null && init.method !== 'GET' && init.method !== 'HEAD') init.body = body

  const requestObj = {
    abort() {
      controller.abort()
    },
  }

  fetch(url, init).then(async(resp) => {
    clearTimeout(id)
    const text = await resp.text()
    const out = {
      statusCode: resp.status,
      headers: Object.fromEntries(resp.headers.entries()),
      body: text,
      raw: {
        toString: () => text,
      },
    }
    cb?.(null, out, text)
  }).catch((err) => {
    clearTimeout(id)
    cb?.(err)
  })

  return requestObj
}

export const httpFetch = (url, options = { method: 'get' }) => {
  const obj = {
    isCancelled: false,
    requestObj: null,
    cancelFn: null,
  }
  obj.cancelHttp = () => {
    if (!obj.requestObj) {
      obj.isCancelled = true
      return
    }
    obj.requestObj.abort?.()
    obj.requestObj = null
    obj.cancelFn?.(new Error('cancelled'))
    obj.cancelFn = null
  }

  obj.promise = new Promise((resolve, reject) => {
    obj.cancelFn = reject
    const requestObj = doFetch(url, {
      method: options.method || 'get',
      headers: options.headers,
      timeout: options.timeout,
      body: options.body,
    }, (err, resp) => {
      obj.requestObj = null
      obj.cancelFn = null
      if (err) return reject(err)
      resolve(resp)
    })
    obj.requestObj = requestObj
    if (obj.isCancelled) obj.cancelHttp()
  })

  return obj
}

export const cancelHttp = requestObj => {
  if (!requestObj) return
  if (!requestObj.abort) return
  requestObj.abort()
}

export const http = (url, options, cb) => {
  if (typeof options === 'function') {
    cb = options
    options = {}
  }
  if (options.method == null) options.method = 'get'
  return doFetch(url, options, cb)
}

export const httpGet = (url, options, callback) => {
  if (typeof options === 'function') {
    callback = options
    options = {}
  }
  return doFetch(url, { ...options, method: 'get' }, callback)
}

export const httpPost = (url, data, options, callback) => {
  if (typeof options === 'function') {
    callback = options
    options = {}
  }
  const headers = { ...(options.headers || {}) }
  if (!headers['Content-Type'] && !(data instanceof FormData)) headers['Content-Type'] = 'application/json'
  const body = data instanceof FormData ? data : JSON.stringify(data ?? {})
  return doFetch(url, { ...options, method: 'post', headers, body }, callback)
}

export const http_jsonp = (url, options, callback) => {
  if (typeof options === 'function') {
    callback = options
    options = {}
  }
  const jsonpCallback = 'jsonpCallback'
  if (url.indexOf('?') < 0) url += '?'
  url += `&${options.jsonpCallback}=${jsonpCallback}`

  return doFetch(url, { ...options, method: 'get' }, (err, resp, body) => {
    if (err) return callback(err)
    try {
      const raw = String(body ?? '')
      const json = JSON.parse(raw.replace(new RegExp(`^${jsonpCallback}\\\\((.*)\\\\)$`), '$1'))
      callback(null, resp, json)
    } catch (e) {
      callback(e)
    }
  })
}

export const checkUrl = (url, options = {}) => {
  return new Promise((resolve, reject) => {
    doFetch(url, { ...options, method: 'head' }, (err, resp) => {
      if (err) return reject(err)
      if (resp.statusCode === 200) resolve()
      else reject(new Error(resp.statusCode))
    })
  })
}

