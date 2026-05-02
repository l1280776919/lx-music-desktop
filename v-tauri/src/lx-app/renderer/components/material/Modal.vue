<template>
  <teleport :to="teleport">
    <div v-if="showModal" ref="dom_container" :class="$style.container">
      <transition enter-active-class="animated fadeIn" leave-active-class="animated fadeOut">
        <div v-show="showContent" :class="[$style.modal, {[$style.filter]: filter}]" @click="bgClose && close()">
          <transition :enter-active-class="inClass" :leave-active-class="outClass" @after-enter="$emit('after-enter', $event)" @after-leave="handleAfterLeave">
            <div v-show="showContent" :class="$style.content" :style="contentStyle" @click.stop>
              <header :class="$style.header">
                <button v-if="closeBtn" type="button" @click="close">
                  <svg version="1.1" xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink" height="100%" viewBox="0 0 212.982 212.982" space="preserve">
                    <use xlink:href="#icon-delete" />
                  </svg>
                </button>
              </header>
              <slot />
            </div>
          </transition>
        </div>
      </transition>
    </div>
  </teleport>
</template>

<script>
import { getRandom } from '@common/utils/common'
import { nextTick } from '@common/utils/vueTools'
import { appSetting } from '@renderer/store/setting'

let modalCount = 0
export default {
  props: {
    show: {
      type: Boolean,
      default: false,
    },
    closeBtn: {
      type: Boolean,
      default: true,
    },
    bgClose: {
      type: Boolean,
      default: false,
    },
    teleport: {
      type: String,
      default: '#overlay-root',
    },
    maxWidth: {
      type: String,
      default: '76%',
    },
    minWidth: {
      type: String,
      default: '280px',
    },
    maxHeight: {
      type: String,
      default: '76%',
    },
    width: {
      type: String,
      default: 'auto',
    },
    height: {
      type: String,
      default: 'auto',
    },
  },
  emits: ['after-enter', 'after-leave', 'close'],
  data() {
    return {
      animates: [
        [['fadeIn', 'fadeInUp'], ['fadeOut', 'fadeOutDown']],
        [['zoomIn', 'fadeIn'], ['zoomOut', 'fadeOut']],
        [['slideInUp', 'fadeIn'], ['slideOutDown', 'fadeOut']],

        // ['flipInX', 'flipOutX'],
        // ['flipInY', 'flipOutY'],
        // ['lightSpeedIn', 'lightSpeedOut'],
        // ['rotateInDownLeft', 'rotateOutDownLeft'],
        // ['rotateInDownRight', 'rotateOutDownRight'],
        // ['rotateInUpLeft', 'rotateOutUpLeft'],
        // ['rotateInUpRight', 'rotateOutUpRight'],
        // // ['rollIn', 'rollOut'],
        // // ['zoomIn', 'zoomOut'],
        // ['zoomInDown', 'zoomOutDown'],
        // // ['zoomInLeft', 'zoomOutLeft'],
        // // ['zoomInRight', 'zoomOutRight'],
        // ['zoomInUp', 'zoomOutUp'],
        // ['slideInDown', 'slideOutDown'],
        // ['slideInLeft', 'slideOutLeft'],
        // ['slideInRight', 'slideOutRight'],
        // ['slideInUp', 'slideOutUp'],
        // // ['jackInTheBox', 'hinge'],
      ],
      // animateIn: [
      //   'flipInX',
      //   'flipInY',
      //   // 'fadeIn',
      //   // 'bounceIn',
      //   'lightSpeedIn',
      //   'rotateInDownLeft',
      //   'rotateInDownRight',
      //   'rotateInUpLeft',
      //   'rotateInUpRight',
      //   'rollIn',
      //   'zoomIn',
      //   'zoomInDown',
      //   'zoomInLeft',
      //   'zoomInRight',
      //   'zoomInUp',
      //   'slideInDown',
      //   'slideInLeft',
      //   'slideInRight',
      //   'slideInUp',
      //   'jackInTheBox',
      // ],
      // animateOut: [
      //   'flipOutX',
      //   'flipOutY',
      //   // 'fadeOut',
      //   // 'bounceOut',
      //   'lightSpeedOut',
      //   'rotateOutDownLeft',
      //   'rotateOutDownRight',
      //   'rotateOutUpLeft',
      //   'rotateOutUpRight',
      //   'rollOut',
      //   'zoomOut',
      //   'zoomOutDown',
      //   'zoomOutLeft',
      //   'zoomOutRight',
      //   'zoomOutUp',
      //   'slideOutDown',
      //   'slideOutLeft',
      //   'slideOutRight',
      //   'slideOutUp',
      //   'hinge',
      // ],
      inClass: 'animated-fast fadeInUp',
      outClass: 'animated-fast fadeOutDown',
      showModal: false,
      showContent: false,
      modalCount: false,
      isAddedClass: false,
      // ai: 0,
    }
  },
  computed: {
    contentStyle() {
      return {
        maxWidth: this.maxWidth,
        minWidth: this.minWidth,
        width: this.width,
        height: this.height,
        maxHeight: this.maxHeight,
      }
    },
    filter() {
      return ['#root', '#overlay-root'].includes(this.teleport) || this.modalCount > 1
    },
  },
  watch: {
    show(val) {
      this.handleShowChange(val)
    },
  },
  mounted() {
    if (this.show) this.handleShowChange(true)
    this.setRandomAnimation()
  },
  beforeUnmount() {
    this.removeClass()
  },
  methods: {
    handleShowChange(val) {
      if (val) {
        // const dom = document.getElementById(this.teleport)
        // if (dom) {
        //   // dom.t
        // }
        this.setRandomAnimation()
        this.modalCount = ++modalCount
        this.showModal = true
        void nextTick(() => {
          const node = this.$refs.dom_container.parentNode
          const effectNode = node.id == 'overlay-root' ? document.getElementById('container') : node
          if (effectNode && !effectNode.classList.contains('show-modal')) {
            effectNode.classList.add('show-modal')
            this.isAddedClass = true
          }
          this.showContent = true
        })
      } else {
        if (modalCount > 0) this.modalCount = --modalCount
        this.removeClass()
        this.showContent = false
      }
    },
    removeClass() {
      if (!this.isAddedClass) return
      const node = this.$refs.dom_container?.parentNode
      const effectNode = node?.id == 'overlay-root' ? document.getElementById('container') : node
      if (modalCount < 1) effectNode?.classList.remove('show-modal')
      this.isAddedClass = false
    },
    setRandomAnimation() {
      if (appSetting['common.randomAnimate']) {
        const [animIn, animOut] = this.animates[getRandom(0, this.animates.length)]
        // const [animIn, animOut] = this.animates[this.ai]
        // if (++this.ai >= this.animates.length) this.ai = 0
        // console.log(animIn, animOut)
        // this.inClass = 'animated ' + animIn
        // this.outClass = 'animated ' + animOut
        this.inClass = 'animated-fast ' + animIn[getRandom(0, animIn.length)]
        this.outClass = 'animated-fast ' + animOut[getRandom(0, animOut.length)]
      }
    },
    close() {
      this.$emit('close')
    },
    handleAfterLeave(event) {
      this.$emit('after-leave', event)
      this.showModal = false
    },
  },
}
</script>


<style lang="less" module>

@import '@renderer/assets/styles/layout.less';

.container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 99;
}

.modal {
  width: 100%;
  height: 100%;
  // background-color: rgba(0, 0, 0, .2);
  // background-color: rgba(255, 255, 255, .6);
  // background-color: var(--color-primary-light-600-alpha-900);
  // backdrop-filter: blur(4px);
  // backdrop-filter: grayscale(70%);
  display: grid;
  align-items: center;
  justify-items: center;
  // will-change: transform;

  &.filter {
    background-color: rgba(0, 0, 0, 0.16);
  }

  // &:before {
  //   .mixin-after();
  //   position: absolute;
  //   left: 0;
  //   top: 0;
  //   width: 100%;
  //   height: 100%;
  //   background-color: var(--color-000);
  //   opacity: .6;
  // }
}

.content {
  position: relative;
  border-radius: @radius-border;
  box-shadow: 0 8px 32px rgba(0, 0, 0, .15);
  overflow: hidden;
  // max-height: 80%;
  // max-width: 76%;
  min-width: 220px;
  position: relative;
  display: flex;
  flex-flow: column nowrap;
  z-index: 100;
  background-color: var(--color-main-background);
  backdrop-filter: blur(14px) saturate(135%);
  -webkit-backdrop-filter: blur(14px) saturate(135%);
  border: 1px solid var(--color-primary-alpha-900);
}

.header {
  flex: none;
  background-color: var(--color-primary-light-100-alpha-100);
  display: flex;
  align-items: center;
  justify-content: flex-end;
  height: 18px;

  button {
    border: none;
    cursor: pointer;
    padding: 4px 7px;
    background-color: transparent;
    color: var(--color-primary-dark-500-alpha-500);
    outline: none;
    transition: background-color 0.2s ease;
    line-height: 0;

    svg {
      height: .7em;
    }

    &:hover {
      background-color: var(--color-primary-dark-100-alpha-600);
    }
    &:active {
      background-color: var(--color-primary-dark-200-alpha-600);
    }
  }
}

</style>
