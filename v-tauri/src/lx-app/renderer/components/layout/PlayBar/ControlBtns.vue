<template>
  <div :class="$style.controlBtn">
    <!-- <common-volume-bar /> -->
    <button :class="$style.titleBtn" :aria-label="$t('player__add_music_to')" @click="addMusicTo">
      <Plus :size="20" stroke-width="2.5" />
    </button>
    <button :class="$style.titleBtn" :aria-label="toggleDesktopLyricBtnTitle" @click="toggleDesktopLyric" @contextmenu="toggleLockDesktopLyric">
      <Mic2 v-show="appSetting['desktopLyric.enable']" :size="20" stroke-width="2.5" />
      <Mic v-show="!appSetting['desktopLyric.enable']" :size="20" stroke-width="2.5" />
    </button>
    <common-volume-btn />
    <common-toggle-play-mode-btn />
    <common-list-add-modal v-model:show="isShowAddMusicTo" :music-info="playMusicInfo.musicInfo ?? null" />
  </div>
</template>

<script>
import { ref } from '@common/utils/vueTools'
import useToggleDesktopLyric from '@renderer/utils/compositions/useToggleDesktopLyric'
import { musicInfo, playMusicInfo } from '@renderer/store/player/state'
import { appSetting } from '@renderer/store/setting'
import { Plus, Mic, Mic2 } from 'lucide-vue-next'

export default {
  components: { Plus, Mic, Mic2 },
  setup() {
    const isShowAddMusicTo = ref(false)
    const {
      toggleDesktopLyricBtnTitle,
      toggleDesktopLyric,
      toggleLockDesktopLyric,
    } = useToggleDesktopLyric()
    const addMusicTo = () => {
      if (!musicInfo.id) return
      isShowAddMusicTo.value = true
    }
    return {
      appSetting,
      isShowAddMusicTo,
      toggleDesktopLyricBtnTitle,
      toggleDesktopLyric,
      toggleLockDesktopLyric,
      addMusicTo,
      playMusicInfo,
    }
  },
}
</script>

<style lang="less" module>
@import '@renderer/assets/styles/layout.less';

.controlBtn {
  padding-left: 20px;
  padding-right: 10px;
  flex: none;
  display: flex;
  flex-flow: row nowrap;
  gap: 10px;

  button {
    color: var(--color-button-font);
  }
}

.titleBtn {
  flex: none;
  height: 100%;
  width: 24px;
  transition: @transition-fast;
  transition-property: color, opacity;
  // color: var(--color-button-font);
  display: flex;
  flex-flow: column nowrap;
  justify-content: center;
  align-items: center;
  background-color: transparent;
  border: none;
  width: 24px;
  padding: 0;

  opacity: .6;
  cursor: pointer;
  transition: background-color 0.2s ease, opacity 0.2s ease, transform 0.1s ease;
  border-radius: 50%;
  height: 24px;

  svg {
    filter: drop-shadow(0 0 1px rgba(0, 0, 0, 0.2));
    fill: none;
    stroke: currentColor;
  }
  &:hover {
    opacity: 1;
    background-color: rgba(0, 0, 0, 0.05);
  }
  &:active {
    opacity: 1;
    transform: scale(0.92);
  }
}


</style>
