import { createMainWorker, createDownloadWorker } from './worker-utils'

export default () => {
  return {
    main: createMainWorker(),
    download: createDownloadWorker(),
  }
}
