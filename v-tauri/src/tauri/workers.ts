import { basename, extname, readLxConfigFile, saveLxConfigFile, saveStrToFile } from './nodejs'

const normalizeText = (text: string) => text.trim().toLowerCase()

const mainWorker = {
  async langS2t(textBase64: string) {
    return textBase64
  },
  readLxConfigFile,
  saveLxConfigFile,
  async exportPlayListToText(path: string, lists: any[], isMerge: boolean) {
    const content = JSON.stringify({ isMerge, lists }, null, 2)
    await saveStrToFile(path, content)
  },
  async exportPlayListToCSV(path: string, lists: any[], isMerge: boolean, header: string) {
    const rows = lists.flatMap((list: any) => list.musics ?? [])
      .map((music: any) => `${music.name ?? ''},${music.singer ?? ''},${music.meta?.albumName ?? ''}`)
    await saveStrToFile(path, `${header}${rows.join('\n')}`)
  },
  async filterDuplicateMusic(list: any[]) {
    const seen = new Set<string>()
    return list.filter((item: any) => {
      const key = `${item.id}_${item.source}_${item.name}_${item.singer}`
      if (seen.has(key)) return false
      seen.add(key)
      return true
    })
  },
  async sortListMusicInfo(list: any[], sortType: string, sortField: string) {
    const sorted = [...list]
    sorted.sort((a: any, b: any) => {
      const av = String(a?.[sortField] ?? a?.meta?.[sortField] ?? '')
      const bv = String(b?.[sortField] ?? b?.meta?.[sortField] ?? '')
      return sortType == 'desc' ? bv.localeCompare(av) : av.localeCompare(bv)
    })
    return sorted
  },
  async createLocalMusicInfos(paths: string[]) {
    return paths.map(path => ({
      id: path,
      name: basename(path, extname(path)),
      singer: '',
      source: 'local',
      interval: '',
      meta: {
        albumName: '',
        filePath: path,
        songId: path,
        picUrl: '',
        ext: extname(path).replace(/^\./, ''),
      },
    }))
  },
  async searchListMusic(list: any[], text: string) {
    const keyword = normalizeText(text)
    if (!keyword) return list
    return list.filter((item: any) => {
      return [item.name, item.singer, item.meta?.albumName]
        .filter(Boolean)
        .some((value: string) => normalizeText(value).includes(keyword))
    })
  },
  async createSortedList(targetList: any[], position: number, ids: string[]) {
    const selected = targetList.filter((item: any) => ids.includes(item.id))
    const others = targetList.filter((item: any) => !ids.includes(item.id))
    return [...others.slice(0, position), ...selected, ...others.slice(position)]
  },
  async filterMusicList({ list, playerIndex }: any) {
    return {
      filteredList: list,
      canPlayList: list,
      playerIndex: playerIndex < 0 ? 0 : playerIndex,
    }
  },
  async getMusicFilePic() {
    return ''
  },
  async getMusicFileLyric() {
    return null
  },
}

const downloadWorker = {
  async writeMeta() {},
  async saveLrc() {},
  async updateUrl() {},
  async removeTask() {},
  async pauseTask() {},
  async startTask() {
    throw new Error('Tauri 版暂未实现下载任务')
  },
  async createDownloadTasks() {
    return []
  },
}

export const createWorkers = () => {
  return {
    main: mainWorker,
    download: downloadWorker,
  }
}
