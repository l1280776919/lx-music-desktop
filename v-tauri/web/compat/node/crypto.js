import CryptoJS from 'crypto-js'

export const createHash = (algorithm) => {
  const algo = String(algorithm || '').toLowerCase()
  let buf = ''
  return {
    update(data) {
      buf += String(data ?? '')
      return this
    },
    digest(encoding) {
      if (algo !== 'md5') throw new Error(`crypto.createHash(${algorithm}) not supported`)
      const hex = CryptoJS.MD5(buf).toString()
      if (!encoding || encoding === 'hex') return hex
      throw new Error(`crypto digest encoding ${encoding} not supported`)
    },
  }
}

export default {
  createHash,
}

