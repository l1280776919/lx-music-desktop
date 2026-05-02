<template>
  <div :class="[$style.toolbar, { [$style.fullscreen]: isFullscreen }, appSetting['common.controlBtnPosition'] == 'left' ? $style.controlBtnLeft : $style.controlBtnRight]" @dblclick="handleToolbarDblclick">
    <div :class="$style.leading">
      <SearchInput />
    </div>
    <div v-if="appSetting['common.controlBtnPosition'] == 'left'" :class="$style.logo">L X</div>
    <div v-else :class="$style.safeZone" />
  </div>
</template>

<script setup>
import { isFullscreen } from '@renderer/store'
import { appSetting } from '@renderer/store/setting'
import { minMaxWindowToggle } from '@renderer/utils/ipc'
import SearchInput from './SearchInput.vue'

const handleToolbarDblclick = () => {
  if (isFullscreen.value) return
  minMaxWindowToggle()
}

</script>


<style lang="less" module>
@import '@renderer/assets/styles/layout.less';

.toolbar {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto auto;
  height: @height-toolbar;
  align-items: center;
  gap: 12px;
  padding: 0 16px;
  -webkit-app-region: drag;
  z-index: 2;
  background-color: transparent;
  // backdrop-filter: blur(15px) saturate(150%);
  // -webkit-backdrop-filter: blur(15px) saturate(150%);
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);

  &.fullscreen {
    -webkit-app-region: no-drag;
    .logo {
      display: none;
    }
  }

  &.controlBtnLeft {
    .control {
      display: none;
    }
  }
  // &.controlBtnRight {
  //   justify-content: space-between;
  // }
}

.leading {
  min-width: 0;
  display: flex;
  align-items: center;
}

.logo {
  box-sizing: border-box;
  padding: 0 8px;
  height: @height-toolbar;
  color: var(--color-primary);
  flex: none;
  text-align: center;
  line-height: @height-toolbar;
  font-weight: bold;
  justify-self: end;
}

.safeZone {
  width: var(--window-controls-safe-width, 112px);
  height: 100%;
  flex: none;
}

</style>
