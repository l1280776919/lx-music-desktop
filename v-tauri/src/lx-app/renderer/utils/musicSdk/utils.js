import CryptoJS from 'crypto-js'
import { decodeName } from '@renderer/utils'

export const toMD5 = str => CryptoJS.MD5(str).toString()


const ipMap = new Map()
export const getHostIp = hostname => {
  const result = ipMap.get(hostname)
  if (typeof result === 'object') return result
  if (result === true) return
  ipMap.set(hostname, true)
  if (typeof window != 'undefined') return
}

export const dnsLookup = (hostname, options, callback) => {
  const result = getHostIp(hostname)
  if (result) return callback(null, result.address, result.family)
  callback(null, hostname, options?.family ?? 4)
}


/**
 * 格式化歌手
 * @param singers 歌手数组
 * @param nameKey 歌手名键值
 * @param join 歌手分割字符
 */
export const formatSingerName = (singers, nameKey = 'name', join = '、') => {
  if (Array.isArray(singers)) {
    const singer = []
    singers.forEach(item => {
      let name = item[nameKey]
      if (!name) return
      singer.push(name)
    })
    return decodeName(singer.join(join))
  }
  return decodeName(String(singers ?? ''))
}
