import { hasTauriContext, safeInvoke } from '../runtime'

type Callback = (err: Error | null, resp?: any, body?: any) => void

const toObject = async(response: Response) => {
  const raw = Buffer.from(await response.arrayBuffer())
  const text = raw.toString()
  let body: any = text
  try {
    body = JSON.parse(text)
  } catch {}
  return {
    statusCode: response.status,
    statusMessage: response.statusText,
    headers: Object.fromEntries(response.headers.entries()),
    bytes: raw.length,
    raw,
    body,
  }
}

const toResponse = (statusCode: number, statusMessage: string, headers: Record<string, string>, bytes: number, text: string, raw?: number[]) => {
  const rawBuffer = raw ? Buffer.from(raw) : Buffer.from(text)
  const responseText = rawBuffer.toString()
  let body: any = responseText
  try {
    body = JSON.parse(responseText)
  } catch {}
  return {
    statusCode,
    statusMessage,
    headers,
    bytes: rawBuffer.length || bytes,
    raw: rawBuffer,
    body,
  }
}

const normalizeHeaders = (headers: Record<string, unknown>) => {
  return Object.fromEntries(
    Object.entries(headers)
      .filter(([, value]) => value != null)
      .map(([key, value]) => [key, String(value)]),
  )
}

const normalizeProxy = (proxy: Record<string, unknown> | undefined) => {
  if (!proxy) return undefined
  const host = proxy.host == null ? '' : String(proxy.host)
  const port = proxy.port == null ? '' : String(proxy.port)
  if (!host) return undefined
  return { host, port }
}

const request = (method: string, url: string, data: any, options: any = {}, callback: Callback) => {
  const controller = new AbortController()
  const headers = { ...(options.headers ?? {}) }
  let body: BodyInit | undefined
  let requestBody: string | undefined

  if (data != null && method.toUpperCase() != 'GET') {
    if (options.json === false && typeof data == 'string') {
      body = data
      requestBody = data
    } else if (data instanceof FormData) body = data
    else if (typeof data == 'string') {
      body = data
      requestBody = data
    } else if (options.json === false && data && typeof data == 'object') {
      headers['Content-Type'] ??= 'application/x-www-form-urlencoded'
      requestBody = new URLSearchParams(Object.entries(data).reduce<Record<string, string>>((result, [key, value]) => {
        if (value == null) return result
        result[key] = String(value)
        return result
      }, {})).toString()
      body = requestBody
    } else {
      headers['Content-Type'] ??= 'application/json'
      requestBody = JSON.stringify(data)
      body = requestBody
    }
  }

  if (hasTauriContext() && !(body instanceof FormData)) {
    void safeInvoke<{ statusCode: number, statusMessage: string, headers: Record<string, string>, bytes: number, body: string, raw?: number[], finalUrl: string, httpVersion: string }>('http_request', {
      payload: {
        method: method.toUpperCase(),
        url,
        headers: normalizeHeaders(headers),
        body: requestBody,
        timeout: options.response_timeout ?? options.timeout ?? 15000,
        proxy: normalizeProxy(options.proxy),
      },
    }).then((response) => {
      const resp = toResponse(response.statusCode, response.statusMessage, response.headers, response.bytes, response.body, response.raw)
      ;(resp as any).finalUrl = response.finalUrl
      ;(resp as any).httpVersion = response.httpVersion
      callback(null, resp, resp.body)
    }).catch((error: Error) => {
      callback(error)
    })
    return {
      request: {
        abort() {},
      },
    }
  }

  void fetch(url, {
    method: method.toUpperCase(),
    headers,
    body,
    signal: controller.signal,
  }).then(async(response) => {
    const resp = await toObject(response)
    callback(null, resp, resp.body)
  }).catch((error: Error) => {
    callback(error)
  })

  return {
    request: {
      abort() {
        controller.abort()
      },
    },
  }
}

export default {
  request,
}
