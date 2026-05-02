<template>
  <div id="container" class="view-container">
    <layout-aside id="left" />
    <div id="right">
      <layout-toolbar id="toolbar" />
      <layout-view id="view" />
      <layout-play-bar id="player" />
    </div>
    <div id="overlay-root" />
    <LayoutToolbarControlBtns v-if="appSetting['common.controlBtnPosition'] != 'left' && !isShowPlayerDetail" id="window-controls" />
    <layout-icons />
    <layout-change-log-modal />
    <layout-update-modal />
    <layout-pact-modal />
    <layout-sync-mode-modal />
    <layout-sync-auth-code-modal />
    <layout-play-detail />
  </div>
</template>

<script setup>
import { onMounted } from '@common/utils/vueTools'
import LayoutToolbarControlBtns from '@renderer/components/layout/Toolbar/ControlBtns.vue'
import { appSetting } from '@renderer/store/setting'
import { isShowPlayerDetail } from '@renderer/store/player/state'
// import BubbleCursor from '@common/utils/effects/cursor-effects/bubbleCursor'
// import '@common/utils/effects/snow.min'
import useApp from '@renderer/core/useApp'

useApp()

onMounted(() => {
  document.getElementById('root').style.display = 'block'

  // const styles = getComputedStyle(document.documentElement)
  // window.lxData.bubbleCursor = new BubbleCursor({
  //   fillStyle: styles.getPropertyValue('--color-primary-alpha-900'),
  //   strokeStyle: styles.getPropertyValue('--color-primary-alpha-700'),
  // })
})

// onBeforeUnmount(() => {
//   window.lxData.bubbleCursor?.destroy()
// })

</script>


<style lang="less">
@import './assets/styles/index.less';
@import './assets/styles/layout.less';

html {
  height: 100vh;
}
html, body {
  // overflow: hidden;
  box-sizing: border-box;
  margin: 0;
  width: 100%;
}

body {
  user-select: none;
  height: 100%;
}
#root {
  width: 100%;
  height: 100%;
  position: relative;
  overflow: hidden;
  color: var(--color-font);
  background: var(--background-image) var(--background-image-position) no-repeat;
  background-size: var(--background-image-size);
  transition: background-color @transition-normal;
  background-color: var(--color-content-background);
  box-sizing: border-box;
  z-index: 0;

  /* 高级毛玻璃背景底色（网易云风格衍生，带柔和光晕） */
  &::before {
    content: '';
    position: absolute;
    top: -50%;
    left: -50%;
    width: 200%;
    height: 200%;
    background: 
      radial-gradient(circle at 20% 30%, var(--color-primary-light-300-alpha-500) 0%, transparent 40%),
      radial-gradient(circle at 80% 70%, var(--color-primary-light-400-alpha-400) 0%, transparent 40%),
      radial-gradient(circle at 50% 50%, var(--color-primary-light-600-alpha-300) 0%, transparent 60%);
    z-index: -1;
    pointer-events: none;
    opacity: .72;
    filter: blur(12px);
  }

  &::after {
    content: '';
    position: absolute;
    inset: 0;
    pointer-events: none;
    background:
      linear-gradient(135deg, rgba(255, 255, 255, 0.18), transparent 22%, transparent 78%, rgba(255, 255, 255, 0.1)),
      radial-gradient(circle at 82% 12%, rgba(255, 255, 255, 0.14), transparent 24%);
    opacity: .9;
  }
}

.disableAnimation * {
  transition: none !important;
  animation: none !important;
}

.transparent {
  background: transparent;
  padding: @shadow-app;
  // #waiting-mask {
  //   border-radius: @radius-border;
  //   left: @shadow-app;
  //   right: @shadow-app;
  //   top: @shadow-app;
  //   bottom: @shadow-app;
  // }
  #body {
    border-radius: @radius-border;
  }
  #root {
    box-shadow: 0 0 @shadow-app rgba(0, 0, 0, 0.5);
    border-radius: @radius-border;
  }
  // #container {
    // border-radius: @radius-border;
    // background-color: transparent;
  // }
}
.disableTransparent {
  background-color: var(--color-content-background);

  #body {
    border: none;
  }

  #right {
    border-radius: 0;
    box-shadow: none;
  }

  // #view { // 偏移5px距离解决非透明模式下右侧滚动条无法拖动的问题
  //   margin-right: 5Px;
  // }
}
.fullscreen {
  background-color: var(--color-content-background);

  #right {
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
  }
}

#container {
  position: relative;
  display: flex;
  height: 100%;
  background-color: transparent;
  --window-controls-safe-width: 112px;
}

#overlay-root {
  position: absolute;
  inset: 0;
  z-index: 40;
  pointer-events: none;
}

#overlay-root > * {
  pointer-events: auto;
}

#window-controls {
  position: absolute;
  top: 12px;
  right: 12px;
  z-index: 20;
}

#left {
  flex: none;
  width: @width-app-left;
  z-index: 1; /* 保证左侧区域的层级 */
}
#right {
  flex: auto;
  display: flex;
  flex-flow: column nowrap;
  transition: background-color @transition-normal, box-shadow @transition-normal;
  /* 高级毛玻璃主体内容区 */
  background:
    linear-gradient(180deg, rgba(255, 255, 255, 0.22), rgba(255, 255, 255, 0.08)),
    var(--color-main-background);
  backdrop-filter: blur(14px) saturate(135%);
  -webkit-backdrop-filter: blur(14px) saturate(135%);

  margin: 12px 12px 12px 0;
  border-radius: 16px;
  border: 1px solid rgba(128, 128, 128, 0.15);
  // overflow: hidden; // 移除溢出隐藏，否则绝对定位的关闭按钮可能被裁剪或导致 margin 计算异常
  box-shadow:
    0 16px 48px rgba(0, 0, 0, 0.08),
    0 2px 12px rgba(255, 255, 255, 0.08) inset,
    inset 0 0 0 1px rgba(255, 255, 255, 0.12);
  position: relative; // 恢复相对定位，我们让按钮相对于 #right 定位，但在内部微调 top/right 即可
  z-index: 2; // 提高层级，避免被下层覆盖

  &::before {
    content: '';
    position: absolute;
    inset: 0;
    border-radius: inherit;
    pointer-events: none;
    background: linear-gradient(180deg, rgba(255, 255, 255, 0.12), transparent 26%, transparent 74%, rgba(255, 255, 255, 0.06));
    opacity: .92;
  }
}
#toolbar, #player {
  flex: none;
}
#view {
  position: relative;
  flex: auto;
  // display: flex;
  min-height: 0;
}

.view-container {
  // transition: opacity @transition-normal; // 移除全局的 transition 避免初次进入卡顿
}
#container.show-modal > #left,
#container.show-modal > #right,
#container.show-modal > #window-controls {
  transition: opacity @transition-normal;
  opacity: .9;
}
#view.show-modal > .view-container {
  transition: opacity @transition-normal;
  opacity: .2;
}

</style>

