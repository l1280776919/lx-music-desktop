import needle from 'needle'
import CryptoJS from 'crypto-js'
import { deflate, inflate } from 'pako'
type UserApiInfo = LX.UserApi.UserApiInfo & { script?: string }
type ProxyInfo = { host: string, port: string }

const EVENT_NAMES = {
  request: 'request',
  inited: 'inited',
  updateAlert: 'updateAlert',
} as const

const allSources = ['kw', 'kg', 'tx', 'wy', 'mg', 'xm', 'local'] as const
const supportQualitys: Record<string, LX.Quality[]> = {
  kw: ['128k', '320k', 'flac', 'flac24bit'],
  kg: ['128k', '320k', 'flac', 'flac24bit'],
  tx: ['128k', '320k', 'flac', 'flac24bit'],
  wy: ['128k', '320k', 'flac', 'flac24bit'],
  mg: ['128k', '320k', 'flac', 'flac24bit'],
  xm: ['128k', '320k', 'flac', 'flac24bit'],
  local: [],
}
const supportActions: Record<string, LX.UserApi.UserApiSourceInfoActions[]> = {
  kw: ['musicUrl'],
  kg: ['musicUrl'],
  tx: ['musicUrl'],
  wy: ['musicUrl'],
  mg: ['musicUrl'],
  xm: ['musicUrl'],
  local: ['musicUrl', 'lyric', 'pic'],
}

type RuntimeEventHandler = (payload: any) => Promise<any>
type RuntimeState = {
  apiInfo: LX.UserApi.UserApiInfo
  requestHandler: RuntimeEventHandler | null
  cancelers: Map<string, () => void>
  status: LX.UserApi.UserApiStatus
  proxy: ProxyInfo
  scriptContext: any
  destroyScriptWindow: (() => void) | null
  recentTraces: Array<{ scope: string, payload: Record<string, unknown> }>
}

let runtime: RuntimeState | null = null

const toBuffer = (buffer: Uint8Array | ArrayBuffer | string) => {
  return typeof buffer == 'string' ? Buffer.from(buffer) : Buffer.from(buffer as any)
}

const debugLabel = '[user-api-runtime:debug]'
const safePreview = (value: unknown) => {
  if (typeof value == 'string') return value.slice(0, 80)
  if (typeof value == 'number' || typeof value == 'boolean' || value == null) return value
  if (Buffer.isBuffer(value)) return `Buffer(${value.length})`
  if (value instanceof Uint8Array) return `Uint8Array(${value.length})`
  if (Array.isArray(value)) return `Array(${value.length})`
  if (typeof value == 'object') {
    try {
      return JSON.stringify(value).slice(0, 160)
    } catch {
      return Object.prototype.toString.call(value)
    }
  }
  return String(value)
}
const runtimeDebug = (scope: string, payload: Record<string, unknown>) => {
  if (runtime) {
    runtime.recentTraces.push({ scope, payload })
    if (runtime.recentTraces.length > 20) runtime.recentTraces.shift()
  }
  try {
    console.log(debugLabel, scope, payload)
  } catch {}
}
const previewBuffer = (value: Uint8Array | ArrayBuffer | string | unknown) => {
  try {
    if (typeof value == 'string') return Buffer.from(value).toString('hex').slice(0, 48)
    if (value instanceof Uint8Array || value instanceof ArrayBuffer) return Buffer.from(value as any).toString('hex').slice(0, 48)
  } catch {}
  return undefined
}
const normalizeExternalUrl = (value: string) => {
  let url = value.trim()
  while (url.length > 1) {
    const first = url[0]
    const last = url[url.length - 1]
    if (
      (first == '`' && last == '`') ||
      (first == '"' && last == '"') ||
      (first == '\'' && last == '\'')
    ) {
      url = url.slice(1, -1).trim()
      continue
    }
    break
  }
  return url
}

const bufferToWordArray = (buffer: Uint8Array | ArrayBuffer | string) => {
  const data = typeof buffer == 'string' ? Buffer.from(buffer) : Buffer.from(buffer as any)
  const words: number[] = []
  for (let i = 0; i < data.length; i += 1) {
    words[i >>> 2] |= data[i] << (24 - (i % 4) * 8)
  }
  return CryptoJS.lib.WordArray.create(words, data.length)
}

const wordArrayToBuffer = (wordArray: CryptoJS.lib.WordArray) => {
  const { words, sigBytes } = wordArray
  const result = Buffer.alloc(sigBytes)
  for (let i = 0; i < sigBytes; i += 1) {
    result[i] = (words[i >>> 2] >>> (24 - (i % 4) * 8)) & 0xff
  }
  return result
}

const getAesMode = (mode: string) => {
  switch (mode) {
    case 'aes-128-ecb':
      return CryptoJS.mode.ECB
    case 'aes-128-cbc':
      return CryptoJS.mode.CBC
    default:
      throw new Error(`Unsupported AES mode: ${mode}`)
  }
}

const aesEncrypt = (buffer: Uint8Array | ArrayBuffer | string, mode: string, key: Uint8Array | ArrayBuffer | string, iv?: Uint8Array | ArrayBuffer | string) => {
  const source = toBuffer(buffer)
  const secret = toBuffer(key)
  const vector = mode == 'aes-128-ecb' ? Buffer.alloc(0) : toBuffer(iv ?? '')
  runtimeDebug('aesEncrypt:input', {
    mode,
    sourceLength: source.length,
    sourcePreview: source.toString('hex').slice(0, 48),
    keyLength: secret.length,
    keyPreview: secret.toString('hex').slice(0, 48),
    ivLength: vector.length,
    ivPreview: vector.toString('hex').slice(0, 48),
  })
  const encrypted = CryptoJS.AES.encrypt(bufferToWordArray(source), bufferToWordArray(secret), {
    iv: mode == 'aes-128-ecb' ? undefined : bufferToWordArray(vector),
    mode: getAesMode(mode),
    padding: CryptoJS.pad.Pkcs7,
  })
  const result = wordArrayToBuffer(encrypted.ciphertext)
  runtimeDebug('aesEncrypt:output', {
    mode,
    resultLength: result.length,
    resultPreview: result.toString('hex').slice(0, 48),
  })
  return result
}

const stripPem = (pem: string) => pem.replace(/-----(BEGIN|END) [^-]+-----/g, '').replace(/\s+/g, '')
const readDerLength = (bytes: Uint8Array, offset: number) => {
  const first = bytes[offset]
  if ((first & 0x80) == 0) return { length: first, offset: offset + 1 }
  const size = first & 0x7f
  let length = 0
  for (let i = 0; i < size; i += 1) length = (length << 8) | bytes[offset + 1 + i]
  return { length, offset: offset + 1 + size }
}
const readAsn1 = (bytes: Uint8Array, offset = 0): { tag: number, start: number, end: number, nextOffset: number } => {
  const tag = bytes[offset]
  const lengthInfo = readDerLength(bytes, offset + 1)
  const start = lengthInfo.offset
  const end = start + lengthInfo.length
  return { tag, start, end, nextOffset: end }
}
const readInteger = (bytes: Uint8Array, node: { start: number, end: number }) => {
  let value = bytes.slice(node.start, node.end)
  while (value.length > 1 && value[0] == 0) value = value.slice(1)
  return BigInt(`0x${Buffer.from(value).toString('hex') || '0'}`)
}
const parseRsaPublicKey = (pem: string) => {
  const bytes = Uint8Array.from(Buffer.from(stripPem(pem), 'base64'))
  const root = readAsn1(bytes, 0)
  if (root.tag != 0x30) throw new Error('Invalid RSA public key')
  const first = readAsn1(bytes, root.start)
  if (first.tag == 0x02) {
    const exponentNode = readAsn1(bytes, first.nextOffset)
    return {
      modulus: readInteger(bytes, first),
      exponent: readInteger(bytes, exponentNode),
      size: first.end - first.start - (bytes[first.start] == 0 ? 1 : 0),
    }
  }
  if (first.tag == 0x30) {
    const second = readAsn1(bytes, first.nextOffset)
    if (second.tag == 0x03) {
      const bitString = bytes.slice(second.start + 1, second.end)
      const keyRoot = readAsn1(bitString, 0)
      const modulusNode = readAsn1(bitString, keyRoot.start)
      const exponentNode = readAsn1(bitString, modulusNode.nextOffset)
      return {
        modulus: readInteger(bitString, modulusNode),
        exponent: readInteger(bitString, exponentNode),
        size: modulusNode.end - modulusNode.start - (bitString[modulusNode.start] == 0 ? 1 : 0),
      }
    }
  }
  throw new Error('Unsupported RSA public key format')
}
const modPow = (base: bigint, exponent: bigint, modulus: bigint) => {
  let result = 1n
  let currentBase = base % modulus
  let currentExponent = exponent
  while (currentExponent > 0n) {
    if (currentExponent & 1n) result = (result * currentBase) % modulus
    currentExponent >>= 1n
    currentBase = (currentBase * currentBase) % modulus
  }
  return result
}
const bigintToBuffer = (value: bigint, size: number) => {
  const hex = value.toString(16).padStart(size * 2, '0')
  return Buffer.from(hex, 'hex')
}

const rsaEncrypt = (buffer: Uint8Array | ArrayBuffer | string, key: string) => {
  const source = toBuffer(buffer)
  const { modulus, exponent, size } = parseRsaPublicKey(key)
  const padded = source.length < size ? Buffer.concat([Buffer.alloc(size - source.length), source]) : source
  const encrypted = modPow(BigInt(`0x${padded.toString('hex') || '0'}`), exponent, modulus)
  const result = bigintToBuffer(encrypted, size)
  runtimeDebug('rsaEncrypt', {
    sourceLength: source.length,
    sourcePreview: source.toString('hex').slice(0, 48),
    paddedLength: padded.length,
    resultLength: result.length,
    resultPreview: result.toString('hex').slice(0, 48),
  })
  return result
}

const md5 = (str: string) => {
  const result = CryptoJS.MD5(str).toString()
  runtimeDebug('md5', {
    inputLength: str.length,
    result,
  })
  return result
}

const verifyLyricInfo = (info: any) => {
  if (typeof info != 'object' || typeof info?.lyric != 'string') throw new Error('failed')
  return {
    lyric: info.lyric,
    tlyric: typeof info.tlyric == 'string' ? info.tlyric : null,
    rlyric: typeof info.rlyric == 'string' ? info.rlyric : null,
    lxlyric: typeof info.lxlyric == 'string' ? info.lxlyric : null,
  }
}

const normalizeSources = (sources: any): LX.UserApi.UserApiSources => {
  const normalized = {} as LX.UserApi.UserApiSources
  for (const source of allSources) {
    const userSource = sources?.[source]
    if (!userSource || userSource.type != 'music') continue
    normalized[source] = {
      name: typeof userSource.name == 'string' ? userSource.name : '',
      type: 'music',
      actions: supportActions[source].filter(action => Array.isArray(userSource.actions) && userSource.actions.includes(action)),
      qualitys: supportQualitys[source].filter(quality => Array.isArray(userSource.qualitys) && userSource.qualitys.includes(quality)),
    }
  }
  return normalized
}

const toRequestOptions = (options: any = {}) => {
  const { method = 'get', timeout, headers, body, form, formData } = options
  let data: any
  const needleOptions: Record<string, any> = {
    headers,
  }
  if (runtime?.proxy.host) {
    needleOptions.proxy = {
      host: runtime.proxy.host,
      port: runtime.proxy.port,
    }
  }
  if (body != null) data = body
  else if (form != null) {
    data = form
    needleOptions.json = false
  } else if (formData != null) {
    data = formData
    needleOptions.json = false
  }
  needleOptions.response_timeout = typeof timeout == 'number' && timeout > 0 ? Math.min(timeout, 60_000) : 60_000
  return {
    method,
    data,
    needleOptions,
  }
}

const buildRequest = (url: string, options: any, callback: (err: Error | null, resp?: any, body?: any) => void, callbackContext?: any) => {
  const { method, data, needleOptions } = toRequestOptions(options)
  const normalizedUrl = normalizeExternalUrl(url)
  runtimeDebug('request:start', {
    method,
    url: normalizedUrl,
    rawUrl: normalizedUrl == url ? undefined : url,
    hasBody: data != null,
    bodyPreview: safePreview(data),
    headers: safePreview(needleOptions.headers),
  })
  let requestRef: { abort: () => void } | null = null
  requestRef = needle.request(method, normalizedUrl, data, needleOptions, (err, resp, body) => {
    try {
      if (err) {
        runtimeDebug('request:error', {
          method,
          url: normalizedUrl,
          message: err.message,
        })
        callback.call(callbackContext, err, null, null)
        return
      }
      body = resp.body = resp.raw.toString()
      try {
        resp.body = JSON.parse(resp.body)
      } catch {}
      body = resp.body
      runtimeDebug('request:done', {
        method,
        url: normalizedUrl,
        statusCode: resp.statusCode,
        finalUrl: (resp as any).finalUrl,
        httpVersion: (resp as any).httpVersion,
        bodyPreview: safePreview(body),
      })
      callback.call(callbackContext, null, {
        statusCode: resp.statusCode,
        statusMessage: resp.statusMessage,
        headers: resp.headers,
        bytes: resp.bytes,
        raw: resp.raw,
        body,
      }, body)
    } catch (error: any) {
      callback.call(callbackContext, error)
    }
  }).request
  return () => {
    try {
      requestRef?.abort()
    } catch {}
    requestRef = null
  }
}

const getExecutableScriptSource = (script: string) => {
  return script.includes('sourceURL=') ? script : `${script}\n//# sourceURL=user-api-runtime.js`
}

const getExecutableScript = (script: string) => {
  const executable = getExecutableScriptSource(script)
  return new Function(
    'window',
    'globalThis',
    'self',
    'global',
    'lx',
    'console',
    'setTimeout',
    'clearTimeout',
    'setInterval',
    'clearInterval',
    'Buffer',
    'fetch',
    'URL',
    'URLSearchParams',
    'TextEncoder',
    'TextDecoder',
    'atob',
    'btoa',
    'process',
    'require',
    'module',
    'exports',
    'crypto',
    'location',
    'navigator',
    'document',
    'performance',
    executable,
  )
}

export const getUserApiRuntimeStatus = () => {
  return runtime?.status ?? { status: false }
}

export const clearUserApiRuntime = () => {
  runtime?.cancelers.forEach(cancel => cancel())
  runtime?.destroyScriptWindow?.()
  runtime = null
}

const maskNodeGlobals = (target: Record<string, any>) => {
  Object.defineProperties(target, {
    global: {
      value: undefined,
      configurable: true,
      enumerable: false,
      writable: true,
    },
    process: {
      value: undefined,
      configurable: true,
      enumerable: false,
      writable: true,
    },
    require: {
      value: undefined,
      configurable: true,
      enumerable: false,
      writable: true,
    },
    module: {
      value: undefined,
      configurable: true,
      enumerable: false,
      writable: true,
    },
    exports: {
      value: undefined,
      configurable: true,
      enumerable: false,
      writable: true,
    },
    Buffer: {
      value: undefined,
      configurable: true,
      enumerable: false,
      writable: true,
    },
  })
}

const createIframeScriptWindow = (lxApi: any) => {
  if (typeof document == 'undefined') return null
  const iframe = document.createElement('iframe')
  iframe.style.display = 'none'
  iframe.setAttribute('aria-hidden', 'true')
  ;(document.body || document.documentElement).appendChild(iframe)
  const scriptWindow = iframe.contentWindow as (Window & typeof globalThis) | null
  if (!scriptWindow) {
    iframe.remove()
    return null
  }
  Object.defineProperty(scriptWindow, 'lx', {
    value: lxApi,
    configurable: true,
    enumerable: true,
    writable: true,
  })
  maskNodeGlobals(scriptWindow as any)
  return {
    scriptWindow,
    destroy() {
      iframe.remove()
    },
  }
}

export const loadUserApiRuntime = async(
  apiInfo: UserApiInfo,
  proxy: ProxyInfo,
  emitStatus: (status: LX.UserApi.UserApiStatus) => void,
  emitShowUpdateAlert: (info: LX.UserApi.UserApiUpdateInfo) => void,
) => {
  clearUserApiRuntime()
  if (!apiInfo.script) {
    const status = { status: false, message: '自定义源脚本不存在', apiInfo }
    runtime = {
      apiInfo,
      requestHandler: null,
      cancelers: new Map(),
      status,
      proxy,
      scriptContext: null,
      destroyScriptWindow: null,
      recentTraces: [],
    }
    emitStatus(status)
    return
  }

  let requestHandler: RuntimeEventHandler | null = null
  let isInited = false
  let isShowedUpdateAlert = false

  const lxApi = {
    version: '2.0.0',
    env: 'desktop',
    EVENT_NAMES,
    request(url: string, options: any, callback: (err: Error | null, resp?: any, body?: any) => void) {
      return buildRequest(url, options, callback, this)
    },
    on(eventName: string, handler: RuntimeEventHandler) {
      if (eventName != EVENT_NAMES.request) throw new Error(`The event is not supported: ${eventName}`)
      requestHandler = handler
      return Promise.resolve()
    },
    utils: {
      crypto: {
        aesEncrypt(buffer: Uint8Array | ArrayBuffer | string, mode: string, key: Uint8Array | ArrayBuffer | string, iv?: Uint8Array | ArrayBuffer | string) {
          return aesEncrypt(buffer, mode, key, iv)
        },
        rsaEncrypt(_buffer: Uint8Array | ArrayBuffer | string, _key: string) {
          return rsaEncrypt(_buffer, _key)
        },
        randomBytes(size: number) {
          const bytes = new Uint8Array(size)
          crypto.getRandomValues(bytes)
          const result = Buffer.from(bytes)
          runtimeDebug('randomBytes', {
            size,
            resultPreview: result.toString('hex').slice(0, 48),
          })
          return result
        },
        md5(str: string) {
          return md5(str)
        },
      },
      buffer: {
        from(...args: any[]) {
          const result = Buffer.from(args[0], args[1])
          runtimeDebug('buffer.from', {
            arg0Type: typeof args[0],
            encoding: typeof args[1] == 'string' ? args[1] : undefined,
            resultLength: result.length,
            resultPreview: result.toString('hex').slice(0, 48),
          })
          return result
        },
        bufToString(buf: Uint8Array | ArrayBuffer | string, format?: BufferEncoding) {
          const result = Buffer.from(buf as any, 'binary').toString(format)
          runtimeDebug('bufToString', {
            format: format ?? 'utf8',
            input: safePreview(buf),
            resultPreview: result.slice(0, 80),
          })
          return result
        },
      },
      zlib: {
        inflate(buf: Uint8Array | ArrayBuffer | string) {
          const input = Buffer.from(buf as any)
          runtimeDebug('zlib.inflate:input', {
            inputLength: input.length,
            inputPreview: input.toString('hex').slice(0, 48),
          })
          const result = Buffer.from(inflate(input))
          runtimeDebug('zlib.inflate:output', {
            resultLength: result.length,
            resultPreview: result.toString('hex').slice(0, 48),
          })
          return Promise.resolve(result)
        },
        deflate(data: Uint8Array | ArrayBuffer | string) {
          const input = Buffer.from(data as any)
          runtimeDebug('zlib.deflate:input', {
            inputLength: input.length,
            inputPreview: input.toString('hex').slice(0, 48),
          })
          const result = Buffer.from(deflate(input))
          runtimeDebug('zlib.deflate:output', {
            resultLength: result.length,
            resultPreview: result.toString('hex').slice(0, 48),
          })
          return Promise.resolve(result)
        },
      },
    },
    currentScriptInfo: {
      name: apiInfo.name,
      description: apiInfo.description,
      author: apiInfo.author,
      homepage: apiInfo.homepage,
      version: apiInfo.version,
      rawScript: apiInfo.script,
    },
    send(eventName: string, data: any) {
      return new Promise<void>((resolve, reject) => {
        switch (eventName) {
          case EVENT_NAMES.inited: {
            if (isInited) {
              reject(new Error('Script is inited'))
              return
            }
            isInited = true
            try {
              runtime!.scriptContext = this
              const sources = normalizeSources(data?.sources)
              runtime!.status = data?.status === false
                ? { status: false, message: typeof data?.message == 'string' ? data.message : '脚本初始化失败', apiInfo }
                : { status: true, apiInfo: { ...apiInfo, sources } }
              emitStatus(runtime!.status)
              resolve()
            } catch (error: any) {
              runtime!.status = { status: false, message: error.message, apiInfo }
              emitStatus(runtime!.status)
              reject(error)
            }
            return
          }
          case EVENT_NAMES.updateAlert: {
            if (isShowedUpdateAlert) {
              reject(new Error('The update alert can only be called once.'))
              return
            }
            isShowedUpdateAlert = true
            if (!data || typeof data.log != 'string') {
              reject(new Error('log is required.'))
              return
            }
            emitShowUpdateAlert({
              name: apiInfo.name,
              description: apiInfo.description,
              log: data.log.length > 1024 ? data.log.slice(0, 1024) + '...' : data.log,
              updateUrl: typeof data.updateUrl == 'string' ? data.updateUrl : undefined,
            })
            resolve()
            return
          }
          default:
            reject(new Error(`Unknown event name: ${eventName}`))
        }
      })
    },
  }

  runtime = {
    apiInfo,
    requestHandler,
    cancelers: new Map(),
    status: { status: false, message: 'initing', apiInfo },
    proxy,
    scriptContext: null,
    destroyScriptWindow: null,
    recentTraces: [],
  }

  const iframeContext = createIframeScriptWindow(lxApi)
  const scriptWindow = iframeContext?.scriptWindow ?? Object.create(typeof window != 'undefined' ? window : globalThis)
  if (iframeContext) {
    runtime.destroyScriptWindow = iframeContext.destroy
  } else {
    Object.defineProperty(scriptWindow, 'lx', {
      value: lxApi,
      configurable: true,
      enumerable: true,
      writable: true,
    })
    maskNodeGlobals(scriptWindow)
  }
  runtimeDebug('script:window', {
    iframe: !!iframeContext,
    crypto: typeof (scriptWindow as any).crypto,
    subtleDigest: typeof (scriptWindow as any).crypto?.subtle?.digest,
    process: typeof (scriptWindow as any).process,
    Buffer: typeof (scriptWindow as any).Buffer,
  })
  const runScript = getExecutableScript(apiInfo.script)
  try {
    if (iframeContext) {
      ;(scriptWindow as any).eval(getExecutableScriptSource(apiInfo.script))
    } else {
      runScript.call(
        scriptWindow,
        scriptWindow,
        scriptWindow,
        scriptWindow,
        undefined,
        lxApi,
        console,
        setTimeout,
        clearTimeout,
        setInterval,
        clearInterval,
        undefined,
        fetch,
        URL,
        URLSearchParams,
        TextEncoder,
        TextDecoder,
        atob,
        btoa,
        undefined,
        undefined,
        undefined,
        undefined,
        (scriptWindow as any).crypto,
        (scriptWindow as any).location,
        (scriptWindow as any).navigator,
        (scriptWindow as any).document,
        (scriptWindow as any).performance,
      )
    }
    runtime.requestHandler = requestHandler
    if (!isInited) {
      runtime.status = { status: false, message: '脚本未调用 lx.send(lx.EVENT_NAMES.inited, ...)', apiInfo }
      emitStatus(runtime.status)
    }
  } catch (error: any) {
    runtimeDebug('script:error', {
      message: error?.message,
      stack: error?.stack,
    })
    runtime.status = { status: false, message: error.message, apiInfo }
    emitStatus(runtime.status)
  }
}

export const requestUserApiRuntime = async(data: LX.UserApi.UserApiRequestParams) => {
  if (!runtime?.requestHandler) throw new Error('Request event is not defined')
  const { requestKey } = data
  return await new Promise<any>((resolve, reject) => {
    runtimeDebug('handler:input', {
      requestKey,
      source: data.data.source,
      action: data.data.action,
      info: safePreview(data.data.info),
    })
    runtime!.cancelers.set(requestKey, () => {
      runtime!.cancelers.delete(requestKey)
      reject(new Error('Cancel request'))
    })
    Promise.resolve(runtime!.requestHandler!.call(runtime!.scriptContext, {
      source: data.data.source,
      action: data.data.action,
      info: data.data.info,
    })).then(response => {
      runtime!.cancelers.delete(requestKey)
      switch (data.data.action) {
        case 'musicUrl': {
          const normalizedResponse = typeof response == 'string' ? normalizeExternalUrl(response) : response
          if (typeof normalizedResponse != 'string' || !/^https?:/.test(normalizedResponse)) throw new Error('failed')
          runtimeDebug('handler:output', {
            requestKey,
            action: data.data.action,
            response: normalizedResponse,
          })
          resolve({
            source: data.data.source,
            action: data.data.action,
            data: {
              type: data.data.info.type,
              url: normalizedResponse,
            },
          })
          break
        }
        case 'lyric':
          resolve({
            source: data.data.source,
            action: data.data.action,
            data: verifyLyricInfo(response),
          })
          break
        case 'pic': {
          const normalizedResponse = typeof response == 'string' ? normalizeExternalUrl(response) : response
          if (typeof normalizedResponse != 'string' || !/^https?:/.test(normalizedResponse)) throw new Error('failed')
          resolve({
            source: data.data.source,
            action: data.data.action,
            data: normalizedResponse,
          })
          break
        }
        default:
          throw new Error(`Unknown action: ${data.data.action}`)
      }
    }).catch((error: Error) => {
      runtimeDebug('handler:error', {
        requestKey,
        action: data.data.action,
        message: error.message,
        stack: error.stack,
        recentTraces: runtime?.recentTraces.slice(-8),
      })
      runtime!.cancelers.delete(requestKey)
      reject(error)
    })
  })
}

export const cancelUserApiRuntimeRequest = (requestKey: string) => {
  runtime?.cancelers.get(requestKey)?.()
}
