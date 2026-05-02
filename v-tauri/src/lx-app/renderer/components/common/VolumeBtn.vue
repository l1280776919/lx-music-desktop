<template>
  <material-popup-btn :class="$style.btnContent">
    <button :class="$style.btn" :aria-label="isMute ? $t('player__volume_muted') : `${$t('player__volume')}${parseInt(volume * 100)}%`" @wheel="handleWheel">
      <span :class="$style.icon" v-html="iconSvg" />
    </button>
    <template #content>
      <div :class="$style.setting">
        <div :class="$style.info">
          <span>{{ Math.trunc(volume * 100) }}%</span>
          <base-checkbox
            id="player__volume_mute"
            :model-value="isMute"
            :label="$t('player__volume_mute_label')"
            @update:model-value="saveVolumeIsMute($event)"
          />
        </div>
        <base-slider-bar :class="$style.slider" :value="volume" :min="0" :max="1" :step="0.01" @change="handleUpdateVolume" />
      </div>
    </template>
  </material-popup-btn>
</template>

<script setup>
import { computed } from '@common/utils/vueTools'
// import useNextTogglePlay from '@renderer/utils/compositions/useNextTogglePlay'
// import useToggleDesktopLyric from '@renderer/utils/compositions/useToggleDesktopLyric'
// import { musicInfo, playMusicInfo } from '@renderer/store/player/state'
import { saveVolumeIsMute } from '@renderer/store/setting'
import { volume, isMute } from '@renderer/store/player/volume'
import volumeMuteOutlineSvg from '@renderer/assets/svgs/volume-mute-outline.svg?raw'
import volumeOffOutlineSvg from '@renderer/assets/svgs/volume-off-outline.svg?raw'
import volumeLowOutlineSvg from '@renderer/assets/svgs/volume-low-outline.svg?raw'
import volumeMediumOutlineSvg from '@renderer/assets/svgs/volume-medium-outline.svg?raw'
import volumeHighOutlineSvg from '@renderer/assets/svgs/volume-high-outline.svg?raw'

const handleWheel = (event) => {
  window.app_event.setVolume(Math.round(volume.value * 100 + (-event.deltaY / 100 * 2)) / 100)
}

const handleUpdateVolume = (val) => {
  window.app_event.setVolume(val)
}

const iconSvg = computed(() => {
  return isMute.value
    ? volumeMuteOutlineSvg
    : volume.value == 0
      ? volumeOffOutlineSvg
      : volume.value < 0.3
        ? volumeLowOutlineSvg
        : volume.value < 0.7
          ? volumeMediumOutlineSvg
          : volumeHighOutlineSvg
})

</script>

<style lang="less" module>
@import '@renderer/assets/styles/layout.less';
.btnContent {
  flex: none;
  height: 100%;
  display: flex;
  align-items: center;
}

.btn {
  position: relative;
  justify-content: center;
  align-items: center;
  transition: color @transition-normal;
  cursor: pointer;
  background-color: transparent;
  border: none;
  width: 24px;
  height: 24px;
  display: flex;
  flex-flow: column nowrap;
  padding: 0;
  border-radius: 50%;
  transition: background-color 0.2s ease, opacity 0.2s ease, transform 0.1s ease;

  .icon {
    display: flex;
    width: 90%;
    height: 90%;
  }

  :global(.icon svg),
  .icon :global(svg) {
    transition: opacity @transition-fast;
    opacity: .6;
    filter: drop-shadow(0 0 1px rgba(0, 0, 0, 0.2));
    width: 100%;
    height: 100%;
  }
  &:hover {
    background-color: rgba(0, 0, 0, 0.05);
    .icon :global(svg) {
      opacity: 1;
    }
  }
  &:active {
    transform: scale(0.92);
    .icon :global(svg) {
      opacity: 1;
    }
  }
}

.setting {
  display: flex;
  flex-flow: column nowrap;
  padding: 2px 3px;
  gap: 8px;
  width: 140px;
}

.info {
  display: flex;
  flex-flow: row nowrap;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
  span {
    line-height: 1.2;
  }
}

.slider {
  width: 100%;
}

</style>
