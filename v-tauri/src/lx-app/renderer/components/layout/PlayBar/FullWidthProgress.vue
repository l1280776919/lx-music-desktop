<template>
  <div :class="$style.player">
    <div :class="$style.leftContent">
      <div :class="$style.picContent" :aria-label="$t('player__pic_tip')" @contextmenu="handleToMusicLocation" @click="showPlayerDetail">
        <img v-if="musicInfo.pic" :src="musicInfo.pic" decoding="async" @error="imgError">
        <div v-else :class="$style.emptyPic">L<span>X</span></div>
      </div>
      <div :class="$style.infoContent">
        <div :class="$style.title" :aria-label="title + $t('copy_tip')" @click="handleCopy(title)">
          {{ title }}
        </div>
        <div :class="$style.status">{{ statusText }}</div>
      </div>
    </div>
    
    <div :class="$style.centerContent">
      <div :class="$style.playBtnContent">
        <div :class="$style.playBtn" :aria-label="$t('player__prev')" @click="playPrev()">
          <svg version="1.1" xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink" height="100%" viewBox="0 0 1024 1024" space="preserve">
            <use xlink:href="#icon-prevMusic" />
          </svg>
        </div>
        <div :class="[$style.playBtn, $style.playPauseBtn]" :aria-label="isPlay ? $t('player__pause') : $t('player__play')" @click="togglePlay">
          <svg v-if="isPlay" version="1.1" xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink" height="100%" viewBox="0 0 1024 1024" space="preserve" style="margin-left: 0;">
            <use xlink:href="#icon-pause" />
          </svg>
          <svg v-else version="1.1" xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink" height="100%" viewBox="0 0 1024 1024" space="preserve">
            <use xlink:href="#icon-play" />
          </svg>
        </div>
        <div :class="$style.playBtn" :aria-label="$t('player__next')" @click="playNext()">
          <svg version="1.1" xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink" height="100%" viewBox="0 0 1024 1024" space="preserve">
            <use xlink:href="#icon-nextMusic" />
          </svg>
        </div>
      </div>
      
      <div :class="$style.progressContent">
        <div :class="$style.timeStr">{{ nowPlayTimeStr }}</div>
        <div :class="$style.progress">
          <common-progress-bar v-if="!isShowPlayerDetail" :class-name="$style.progressBar" :progress="progress" :handle-transition-end="handleTransitionEnd" :is-active-transition="isActiveTransition" />
        </div>
        <div :class="$style.timeStr">{{ maxPlayTimeStr }}</div>
      </div>
    </div>
    
    <div :class="$style.rightContent">
      <control-btns />
    </div>
  </div>
</template>

<script>
import { computed } from '@common/utils/vueTools'
import { useRouter } from '@common/utils/vueRouter'
import { clipboardWriteText } from '@common/utils/electron'
import ControlBtns from './ControlBtns.vue'
// import PlayProgress from './PlayProgress'
import usePlayProgress from '@renderer/utils/compositions/usePlayProgress'
// import { lyric } from '@renderer/core/share/lyric'
import {
  statusText,
  musicInfo,
  isShowPlayerDetail,
  isPlay,
  playInfo,
  playMusicInfo,
} from '@renderer/store/player/state'
import {
  setMusicInfo,
  setShowPlayerDetail,
} from '@renderer/store/player/action'
import { appSetting } from '@renderer/store/setting'
import { togglePlay, playNext, playPrev } from '@renderer/core/player'
import { LIST_IDS } from '@common/constants'

export default {
  name: 'CorePlayBar',
  components: {
    ControlBtns,
    // PlayProgress,
  },
  setup() {
    const router = useRouter()

    const {
      nowPlayTimeStr,
      maxPlayTimeStr,
      progress,
      isActiveTransition,
      handleTransitionEnd,
    } = usePlayProgress()

    const showPlayerDetail = () => {
      if (!playMusicInfo.musicInfo) return
      setShowPlayerDetail(true)
    }
    const handleCopy = (text) => {
      clipboardWriteText(text)
    }

    const imgError = () => {
      // console.log(e)
      setMusicInfo({ pic: null })
    }

    const handleToMusicLocation = () => {
      const listId = playMusicInfo.listId
      if (!listId || listId == LIST_IDS.DOWNLOAD || !playMusicInfo.musicInfo) return
      if (playInfo.playIndex == -1) return
      void router.push({
        path: '/list',
        query: {
          id: listId,
          scrollIndex: playInfo.playIndex,
        },
      })
    }

    const title = computed(() => {
      return musicInfo.name
        ? appSetting['download.fileName'].replace('歌名', musicInfo.name).replace('歌手', musicInfo.singer)
        : ''
    })

    // onBeforeUnmount(() => {
    // window.eventHub.emit(eventPlayerNames.setTogglePlay)
    // })

    return {
      musicInfo,
      nowPlayTimeStr,
      maxPlayTimeStr,
      progress,
      isActiveTransition,
      handleTransitionEnd,
      handleCopy,
      imgError,
      statusText,
      title,
      showPlayerDetail,
      isPlay,
      togglePlay,
      playNext,
      playPrev,
      handleToMusicLocation,
      isShowPlayerDetail,
    }
  },
}
</script>


<style lang="less" module>
@import '@renderer/assets/styles/layout.less';

.player {
  position: relative;
  height: @height-player;
  box-sizing: border-box;
  display: flex;
  flex-flow: row nowrap;
  align-items: center;
  justify-content: space-between;
  contain: strict;
  padding: 0 20px;
  z-index: 2;
  
  &:before {
    .mixin-after();
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(255, 255, 255, 0.85);
    opacity: 1;
    z-index: -1;
    backdrop-filter: blur(40px) saturate(220%);
    -webkit-backdrop-filter: blur(40px) saturate(220%);
    border-top: 1px solid rgba(255, 255, 255, 0.6);
    box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.02);
  }
}

.leftContent {
  flex: 1;
  display: flex;
  flex-flow: row nowrap;
  align-items: center;
  min-width: 0;
  height: 100%;
}

.centerContent {
  flex: 2;
  display: flex;
  flex-flow: column nowrap;
  align-items: center;
  justify-content: center;
  height: 100%;
  max-width: 600px;
}

.rightContent {
  flex: 1;
  display: flex;
  flex-flow: row nowrap;
  align-items: center;
  justify-content: flex-end;
  height: 100%;
}

.progressContent {
  width: 100%;
  display: flex;
  flex-flow: row nowrap;
  align-items: center;
  margin-top: 4px;
}

.timeStr {
  flex: none;
  font-size: 11px;
  color: var(--color-font-label);
  font-family: 'Courier New', Courier, monospace;
  width: 40px;
  text-align: center;
}

.progress {
  flex: auto;
  position: relative;
  height: 16px;
  display: flex;
  align-items: center;
  margin: 0 12px;
  cursor: pointer;
  
  .progressBar {
    width: 100%;
    height: 4px;
    border-radius: @radius-progress-border;
    transition: height 0.2s ease, box-shadow 0.2s ease;
  }
  
  &:hover .progressBar {
    height: 6px;
    box-shadow: 0 2px 8px var(--color-primary-alpha-400);
  }
}

.picContent {
  height: 56px;
  aspect-ratio: 1 / 1;

  flex: none;
  opacity: 1;
  transition: opacity @transition-fast;
  display: flex;
  justify-content: center;
  cursor: pointer;
  position: relative;
  z-index: 10;
  border-radius: 8px;

  &:hover {
    opacity: .9;
  }

  img {
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
    max-width: 100%;
    max-height: 100%;
    transition: @transition-normal;
    transition-property: border-color, box-shadow, transform;
    border-radius: 8px;
    object-fit: cover;
    
    &:hover {
      transform: translateY(-2px) scale(1.02);
      box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
    }
  }

  .emptyPic {
    background-color: var(--color-primary-light-900-alpha-200);
    border-radius: 8px;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-primary-light-400-alpha-200);
    user-select: none;
    font-size: 20px;
    font-family: Consolas, "Courier New", monospace;
    box-shadow: 0 2px 6px rgba(0,0,0,0.1);

    span {
      padding-left: 3px;
    }
  }
}

.infoContent {
  padding-left: 12px;
  flex: auto;
  display: flex;
  flex-flow: column nowrap;
  justify-content: center;
  align-items: flex-start;
  min-width: 0;
}

.title {
  max-width: 100%;
  font-size: 14px;
  font-weight: 600;
  color: var(--color-font);
  .mixin-ellipsis-1();
}
.status {
  padding-top: 4px;
  font-size: 12px;
  color: var(--color-font-label);
  .mixin-ellipsis-1();
  max-width: 100%;
}

.playBtnContent {
  flex: none;
  display: flex;
  flex-flow: row nowrap;
  align-items: center;
  justify-content: center;
  gap: 20px;
  height: 36px;
}

.playBtn {
  flex: none;
  height: 100%;
  aspect-ratio: 1 / 1;
  display: flex;
  justify-content: center;
  align-items: center;
  transition: @transition-fast;
  transition-property: color, opacity, background-color, transform, box-shadow;
  border-radius: 50%;
  color: var(--color-font);
  opacity: 0.85;
  cursor: pointer;

  svg {
    height: 50%;
    fill: currentColor;
    filter: drop-shadow(0 0 1px rgba(0, 0, 0, 0.1));
  }
  &:hover {
    opacity: 1;
    color: var(--color-primary);
    background-color: rgba(0, 0, 0, 0.05);
  }
  &:active {
    opacity: 0.8;
    transform: scale(0.95);
  }
}

.playPauseBtn {
  height: 120%;
  background-color: var(--color-primary);
  color: var(--color-primary-font-active, #fff);
  box-shadow: 0 4px 12px var(--color-primary-alpha-400);
  opacity: 1;

  svg {
    height: 45%;
    filter: none;
    margin-left: 2px; // slightly offset play icon visually
  }

  &:hover {
    background-color: var(--color-primary-hover, var(--color-primary));
    color: var(--color-primary-font-active, #fff);
    opacity: 0.95;
    box-shadow: 0 6px 16px var(--color-primary-alpha-500);
  }
}

</style>
