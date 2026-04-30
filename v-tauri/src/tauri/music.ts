import { basename, checkPath, extname, getFileStats, joinPath } from './nodejs'

export const checkDownloadFileAvailable = async(musicInfo: LX.Download.ListItem, savePath: string): Promise<boolean> => {
  return musicInfo.isComplate &&
    (await checkPath(musicInfo.metadata.filePath) || await checkPath(joinPath(savePath, musicInfo.metadata.fileName)))
}

export const checkLocalFileAvailable = async(musicInfo: LX.Music.MusicInfoLocal): Promise<boolean> => {
  return checkPath(musicInfo.meta.filePath)
}

export const checkMusicFileAvailable = async(musicInfo: LX.Music.MusicInfo | LX.Download.ListItem, savePath: string): Promise<boolean> => {
  if ('progress' in musicInfo) return checkDownloadFileAvailable(musicInfo, savePath)
  if (musicInfo.source == 'local') return checkLocalFileAvailable(musicInfo)
  return true
}

export const getDownloadFilePath = async(musicInfo: LX.Download.ListItem, savePath: string): Promise<string> => {
  if (musicInfo.isComplate) {
    if (await checkPath(musicInfo.metadata.filePath)) return musicInfo.metadata.filePath
    const path = joinPath(savePath, musicInfo.metadata.fileName)
    if (await checkPath(path)) return path
  }
  return ''
}

export const getLocalFilePath = async(musicInfo: LX.Music.MusicInfoLocal): Promise<string> => {
  return (await checkPath(musicInfo.meta.filePath)) ? musicInfo.meta.filePath : ''
}

export const getMusicFilePath = async(musicInfo: LX.Music.MusicInfo | LX.Download.ListItem, savePath: string): Promise<string> => {
  if ('progress' in musicInfo) return getDownloadFilePath(musicInfo, savePath)
  if (musicInfo.source == 'local') return getLocalFilePath(musicInfo)
  return ''
}

export const createLocalMusicInfo = async(path: string): Promise<LX.Music.MusicInfoLocal | null> => {
  if (!await checkPath(path)) return null
  const ext = extname(path)
  return {
    id: path,
    name: basename(path, ext).trim(),
    singer: '',
    source: 'local',
    interval: '',
    meta: {
      albumName: '',
      filePath: path,
      songId: path,
      picUrl: '',
      ext: ext.replace(/^\./, ''),
    },
  }
}

export const getLocalMusicFilePic = async(path: string) => {
  const filePath = new RegExp('\\' + extname(path) + '$')
  let picPath = path.replace(filePath, '.jpg')
  let stats = await getFileStats(picPath)
  if (stats) return picPath
  picPath = path.replace(filePath, '.png')
  stats = await getFileStats(picPath)
  if (stats) return picPath
  return null
}

export const getLocalMusicFileLyric = async(_path: string): Promise<LX.Music.LyricInfo | null> => {
  return null
}
