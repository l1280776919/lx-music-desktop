<template>
  <div :class="$style.view">
    <router-view v-slot="{ Component }">
      <!-- 移除会导致初次进入卡顿的过度动画，或者改用轻量的渐变 -->
      <!-- <transition name="fade" mode="out-in"> -->
        <component :is="Component" class="view-container" />
      <!-- </transition> -->
    </router-view>
  </div>
</template>

<style lang="less" module>
@import '@renderer/assets/styles/layout.less';

.view {
  position: relative;
  display: flex;
  width: 100%;
  height: 100%;
  min-height: 0;
  overflow: hidden;
  z-index: 1;
  > :global(.view-container) {
    position: relative !important;
    flex: auto;
    min-height: 0;
    height: 100%;
    width: 100%;
  }
  // background: #fff;
  // overflow: hidden;
}

</style>
<style>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
