(() => {
  const g = globalThis

  if (!g.process) g.process = {}
  if (!g.process.env) g.process.env = {}
  if (!('NODE_ENV' in g.process.env)) g.process.env.NODE_ENV = 'production'
  if (!g.process.platform) g.process.platform = 'linux'
  if (!g.process.versions) g.process.versions = {}
  if (!g.process.versions.app) g.process.versions.app = '0.0.0'

  if (!g.Buffer) {
    const encoder = new TextEncoder()
    const decoder = new TextDecoder()

    const hexToBytes = (hex) => {
      const clean = (hex || '').replace(/^0x/, '').toLowerCase()
      const len = clean.length
      const out = new Uint8Array(Math.ceil(len / 2))
      for (let i = 0; i < out.length; i++) {
        out[i] = parseInt(clean.slice(i * 2, i * 2 + 2), 16) || 0
      }
      return out
    }

    const bytesToHex = (bytes) => {
      let out = ''
      for (let i = 0; i < bytes.length; i++) out += bytes[i].toString(16).padStart(2, '0')
      return out
    }

    const bytesToBase64 = (bytes) => {
      let bin = ''
      for (let i = 0; i < bytes.length; i++) bin += String.fromCharCode(bytes[i])
      return btoa(bin)
    }

    const base64ToBytes = (b64) => {
      const bin = atob(b64)
      const out = new Uint8Array(bin.length)
      for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i)
      return out
    }

    class BufferPolyfill extends Uint8Array {
      static from(input, encoding) {
        if (input == null) return new BufferPolyfill()
        if (typeof input === 'string') {
          if (encoding === 'base64') return new BufferPolyfill(base64ToBytes(input))
          if (encoding === 'hex') return new BufferPolyfill(hexToBytes(input))
          return new BufferPolyfill(encoder.encode(input))
        }
        if (input instanceof ArrayBuffer) return new BufferPolyfill(new Uint8Array(input))
        if (ArrayBuffer.isView(input)) return new BufferPolyfill(new Uint8Array(input.buffer, input.byteOffset, input.byteLength))
        if (Array.isArray(input)) return new BufferPolyfill(Uint8Array.from(input))
        return new BufferPolyfill(encoder.encode(String(input)))
      }

      toString(encoding) {
        if (!encoding || encoding === 'utf8' || encoding === 'utf-8') return decoder.decode(this)
        if (encoding === 'base64') return bytesToBase64(this)
        if (encoding === 'hex') return bytesToHex(this)
        if (encoding === 'binary') {
          let out = ''
          for (let i = 0; i < this.length; i++) out += String.fromCharCode(this[i])
          return out
        }
        return decoder.decode(this)
      }
    }

    g.Buffer = BufferPolyfill
  }
})()

