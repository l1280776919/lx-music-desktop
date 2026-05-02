<template lang="pug">
div(:class="$style.header")
  div(ref="dom_btns" :class="$style.controBtn")
    button(ref="dom_hide_btn" type="button" :class="$style.hide" :aria-label="$t('player__hide_detail_tip')" ignore-tip :title="$t('player__hide_detail_tip')" @click="hide")
      ChevronsDown(:class="$style.controBtnIcon" :size="15" :stroke-width="2.2")
    button(ref="dom_fullscreen_btn" type="button" :class="$style.fullscreenExit" :aria-label="$t('fullscreen_exit')" ignore-tip :title="$t('fullscreen_exit')" @click="fullscreenExit")
      Minimize2(:class="$style.controBtnIcon" :size="15" :stroke-width="2.2")
    button(type="button" :class="$style.min" :aria-label="$t('min')" ignore-tip :title="$t('min')" @click="minWindow")
      Minus(:class="$style.controBtnIcon" :size="15" :stroke-width="2.4")

    //- button(type="button" :class="$style.max" @click="max")
    button(type="button" :class="$style.close" :aria-label="$t('close')" ignore-tip :title="$t('close')" @click="closeWindow")
      X(:class="$style.controBtnIcon" :size="15" :stroke-width="2.2")
</template>


<script setup>
import { ChevronsDown, Minimize2, Minus, X } from 'lucide-vue-next'
import { onMounted, onBeforeUnmount, ref, useCssModule } from '@common/utils/vueTools'
import { isFullscreen } from '@renderer/store'
import { setShowPlayerDetail } from '@renderer/store/player/action'
import { closeWindow, minWindow, setFullScreen } from '@renderer/utils/ipc'

const dom_btns = ref()
const cssModule = useCssModule()

const handle_focus = () => {
  if (!dom_btns.value) return
  for (const node of dom_btns.value.childNodes) {
    if (node.tagName != 'BUTTON') continue
    node.classList.remove(cssModule.hover)
  }
}
const getBtnEl = (el) => el.tagName == 'BUTTON' || !el ? el : getBtnEl(el.parentNode)
const handle_mouseover = (event) => {
  const btn = getBtnEl(event.target)
  if (!btn) return
  btn.classList.add(cssModule.hover)
}
const handle_mouseout = (event) => {
  const btn = getBtnEl(event.target)
  if (!btn) return
  btn.classList.remove(cssModule.hover)
}


onMounted(() => {
  window.app_event.on('focus', handle_focus)
  dom_btns.value.addEventListener('mouseover', handle_mouseover)
  dom_btns.value.addEventListener('mouseout', handle_mouseout)
})
onBeforeUnmount(() => {
  window.app_event.off('focus', handle_focus)
  dom_btns.value.removeEventListener('mouseover', handle_mouseover)
  dom_btns.value.removeEventListener('mouseout', handle_mouseout)
})

const dom_hide_btn = ref()
const hide = () => {
  dom_hide_btn.value?.classList.remove(cssModule.hover)
  setShowPlayerDetail(false)
}
const dom_fullscreen_btn = ref()
const fullscreenExit = () => {
  dom_fullscreen_btn.value?.classList.remove(cssModule.hover)
  void setFullScreen(false).then((fullscreen) => {
    isFullscreen.value = fullscreen
  })
}

</script>


<style lang="less" module>
@import '@renderer/assets/styles/layout.less';

@control-btn-width: @height-toolbar * .26;

:global(.fullscreen) {
  .header {
    -webkit-app-region: no-drag;
    align-self: flex-start;
    .controBtn {
      .close, .min {
        display: none;
      }
      .fullscreenExit {
        display: flex;
      }
    }
  }
}
.header {
  position: relative;
  flex: 0 0 @height-toolbar;
  -webkit-app-region: no-drag;
  width: 100%;
  align-self: flex-start;

  .controBtn {
    position: absolute;
    top: 10px;
    right: 14px;
    display: flex;
    gap: 8px;
    padding: 8px;
    border-radius: 999px;
    background-color: rgba(255, 255, 255, 0.18);
    border: 1px solid rgba(255, 255, 255, 0.28);
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.08);
    backdrop-filter: blur(18px) saturate(140%);
    -webkit-backdrop-filter: blur(18px) saturate(140%);

    button {
      position: relative;
      background: rgba(255, 255, 255, 0.3);
      border: none;
      outline: none;
      padding: 0;
      cursor: pointer;
      display: flex;
      justify-content: center;
      align-items: center;
      width: 34px;
      height: 34px;
      border-radius: 999px;
      color: rgba(0, 0, 0, 0.56);
      transition: background-color 0.18s ease, transform 0.18s ease, color 0.18s ease;

      &:active {
        transform: scale(0.94);
      }
    }

    .fullscreenExit {
      display: none;
    }
  }

  .controBtn {
    button {
      &.hover {
        background-color: rgba(255, 255, 255, 0.72);
        color: rgba(0, 0, 0, 0.78);

        &.close {
          background-color: rgba(255, 95, 86, 0.88);
          color: #fff;
        }
      }
    }
  }
}

.controBtnIcon {
  fill: none;
  stroke: currentColor;
}

</style>
