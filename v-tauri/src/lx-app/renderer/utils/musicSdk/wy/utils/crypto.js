// https://github.com/Binaryify/NeteaseCloudMusicApi/blob/master/util/crypto.js
import CryptoJS from 'crypto-js'

const iv = Buffer.from('0102030405060708')
const presetKey = Buffer.from('0CoJUm6Qyw8W8jud')
const linuxapiKey = Buffer.from('rFgB&h#%2?^eDg:Q')
const base62 = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'
const eapiKey = Buffer.from('e82ckenh8dichen8')
const modulus = BigInt('0x00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7')
const exponent = BigInt('0x010001')

const uint8ArrayToWordArray = uint8Array => {
  const words = []
  for (let i = 0; i < uint8Array.length; i += 1) {
    words[i >>> 2] |= uint8Array[i] << (24 - (i % 4) * 8)
  }
  return CryptoJS.lib.WordArray.create(words, uint8Array.length)
}

const bufferToWordArray = buffer => uint8ArrayToWordArray(Uint8Array.from(buffer))

const wordArrayToBuffer = wordArray => {
  const { words, sigBytes } = wordArray
  const result = new Uint8Array(sigBytes)
  for (let i = 0; i < sigBytes; i += 1) {
    result[i] = (words[i >>> 2] >>> (24 - (i % 4) * 8)) & 0xff
  }
  return Buffer.from(result)
}

const getAesMode = mode => {
  switch (mode) {
    case 'aes-128-cbc':
      return CryptoJS.mode.CBC
    case 'aes-128-ecb':
      return CryptoJS.mode.ECB
    default:
      throw new Error(`Unsupported AES mode: ${mode}`)
  }
}

const aesEncrypt = (buffer, mode, key, aesIv) => {
  const encrypted = CryptoJS.AES.encrypt(bufferToWordArray(buffer), bufferToWordArray(key), {
    iv: aesIv ? bufferToWordArray(aesIv) : undefined,
    mode: getAesMode(mode),
    padding: CryptoJS.pad.Pkcs7,
  })
  return wordArrayToBuffer(encrypted.ciphertext)
}

const aesDecrypt = (cipherBuffer, mode, key, aesIv) => {
  const decrypted = CryptoJS.AES.decrypt({
    ciphertext: bufferToWordArray(cipherBuffer),
  }, bufferToWordArray(key), {
    iv: aesIv ? bufferToWordArray(aesIv) : undefined,
    mode: getAesMode(mode),
    padding: CryptoJS.pad.Pkcs7,
  })
  return wordArrayToBuffer(decrypted)
}

const modPow = (base, exp, mod) => {
  let result = 1n
  let currentBase = base % mod
  let currentExp = exp
  while (currentExp > 0n) {
    if (currentExp & 1n) result = (result * currentBase) % mod
    currentExp >>= 1n
    currentBase = (currentBase * currentBase) % mod
  }
  return result
}

const createSecretKey = size => {
  const bytes = new Uint8Array(size)
  if (globalThis.crypto?.getRandomValues) {
    globalThis.crypto.getRandomValues(bytes)
  } else {
    for (let i = 0; i < size; i += 1) bytes[i] = Math.floor(Math.random() * 256)
  }
  return Buffer.from(Array.from(bytes, n => base62.charCodeAt(n % 62)))
}

const rsaEncrypt = buffer => {
  const reversedHex = Buffer.from(Uint8Array.from(buffer).reverse()).toString('hex')
  const encrypted = modPow(BigInt(`0x${reversedHex}`), exponent, modulus)
  return encrypted.toString(16).padStart(256, '0')
}

export const weapi = object => {
  const text = JSON.stringify(object)
  const secretKey = createSecretKey(16)
  return {
    params: aesEncrypt(Buffer.from(aesEncrypt(Buffer.from(text), 'aes-128-cbc', presetKey, iv).toString('base64')), 'aes-128-cbc', secretKey, iv).toString('base64'),
    encSecKey: rsaEncrypt(secretKey),
  }
}

export const linuxapi = object => {
  const text = JSON.stringify(object)
  return {
    eparams: aesEncrypt(Buffer.from(text), 'aes-128-ecb', linuxapiKey).toString('hex').toUpperCase(),
  }
}

export const eapi = (url, object) => {
  const text = typeof object === 'object' ? JSON.stringify(object) : object
  const message = `nobody${url}use${text}md5forencrypt`
  const digest = CryptoJS.MD5(message).toString()
  const data = `${url}-36cd479b6b5-${text}-36cd479b6b5-${digest}`
  return {
    params: aesEncrypt(Buffer.from(data), 'aes-128-ecb', eapiKey).toString('hex').toUpperCase(),
  }
}

export const eapiDecrypt = cipherBuffer => {
  return aesDecrypt(cipherBuffer, 'aes-128-ecb', eapiKey).toString()
}
