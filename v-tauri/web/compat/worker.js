export default () => {
  const notSupported = async() => {
    throw new Error('worker not supported on tauri-web build yet')
  }

  return {
    main: {
      langS2t: async(b64) => b64,
      langT2s: async(b64) => b64,
      getMusicFilePic: notSupported,
      getMusicFileLyric: notSupported,
      parseLyric: notSupported,
    },
    download: {},
  }
}

