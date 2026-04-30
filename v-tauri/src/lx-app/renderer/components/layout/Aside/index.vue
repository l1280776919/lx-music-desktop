<template>
  <div :class="[$style.aside, { [$style.fullscreen]: isFullscreen }]">
    <ControlBtns v-if="appSetting['common.controlBtnPosition'] == 'left'" />
    <div v-else :class="$style.logo">L X</div>
    <NavBar />
  </div>
</template>

<script setup>
import { isFullscreen } from '@renderer/store'
import { appSetting } from '@renderer/store/setting'

import ControlBtns from './ControlBtns.vue'
import NavBar from './NavBar.vue'

</script>


<style lang="less" module>
@import '@renderer/assets/styles/layout.less';

.aside {
  transition: @transition-normal;
  transition-property: background-color;
  background-color: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(40px) saturate(220%);
  -webkit-backdrop-filter: blur(40px) saturate(220%);
  -webkit-app-region: drag;
  -webkit-user-select: none;
  display: flex;
  flex-flow: column nowrap;
  border-right: 1px solid rgba(255, 255, 255, 0.4);
  z-index: 10;
  box-shadow: 1px 0px 10px rgba(0, 0, 0, 0.02);

  &.fullscreen {
    -webkit-app-region: no-drag;
    .logo {
      display: none;
    }
  }
}

.logo {
  box-sizing: border-box;
  padding: 25px 20px 15px;
  height: auto;
  color: var(--color-primary);
  flex: none;
  text-align: left;
  line-height: normal;
  font-size: 24px;
  font-weight: 900;
  letter-spacing: 2px;
  text-shadow: 0 2px 8px rgba(0,0,0,0.1);
  display: flex;
  align-items: center;
  gap: 10px;

  &::before {
    content: '';
    display: inline-block;
    width: 28px;
    height: 28px;
    background-color: var(--color-primary);
    border-radius: 50%;
    box-shadow: 0 4px 10px var(--color-primary-alpha-400);
  }
}

</style>
