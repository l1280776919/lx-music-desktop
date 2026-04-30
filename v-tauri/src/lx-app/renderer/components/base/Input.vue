<template>
  <input
    ref="dom_input"
    :class="$style.input"
    :type="type"
    :placeholder="placeholder"
    :value="modelValue"
    :disabled="disabled"
    tabindex="0"
    @input="handleInput"
    @change="$emit('change', $event.target.value.trim())"
    @keyup.enter="$emit('submit', $event.target.value.trim())"
    @contextmenu="handleContextMenu"
  >
</template>

<script>
import { clipboardReadText } from '@common/utils/electron'

export default {
  props: {
    placeholder: {
      type: String,
      default: '',
    },
    disabled: {
      type: Boolean,
      default: false,
    },
    modelValue: {
      type: [String, Number],
      default: '',
    },
    type: {
      type: String,
      default: 'text',
    },
    trim: {
      type: Boolean,
      default: true,
    },
    stopContentEventPropagation: {
      type: Boolean,
      default: true,
    },
    autoPaste: {
      type: Boolean,
      default: true,
    },
    // bindValue: {
    //   type: Boolean,
    //   default: true,
    // },
  },
  emits: ['update:modelValue', 'submit', 'change'],
  methods: {
    handleInput(event) {
      let value = event.target.value
      if (this.trim) {
        value = value.trim()
        event.target.value = value
      }
      this.$emit('update:modelValue', value)
    },
    focus() {
      this.$refs.dom_input.focus()
    },
    handleContextMenu(event) {
      if (this.stopContentEventPropagation) event.stopPropagation()
      if (!this.autoPaste) return
      let dom_input = this.$refs.dom_input
      if (dom_input.selectionStart === null) return
      let str = clipboardReadText()
      str = str.trim()
      str = str.replace(/\t|\r\n|\n|\r/g, ' ')
      str = str.replace(/\s+/g, ' ')
      const text = dom_input.value
      // if (dom_input.selectionStart == dom_input.selectionEnd) {
      const value = text.substring(0, dom_input.selectionStart) + str + text.substring(dom_input.selectionEnd, text.length)
      event.target.value = value
      this.$emit('update:modelValue', value)
      // } else {
      //   clipboardWriteText(text.substring(dom_input.selectionStart, dom_input.selectionEnd))
      // }
    },
  },
}
</script>


<style lang="less" module>
@import '@renderer/assets/styles/layout.less';

.input {
  display: inline-block;
  border: 1px solid transparent;
  border-radius: 6px;
  padding: 8px 12px;
  color: var(--color-font);
  outline: none;
  transition: all 0.2s ease;
  background-color: var(--color-primary-light-100-alpha-800);
  font-size: 14px;
  box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.02);

  &::-webkit-outer-spin-button,
  &::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  &::placeholder {
    color: var(--color-font-label);
  }

  &[disabled] {
    opacity: .4;
    cursor: default;
  }

  &:hover {
    background-color: var(--color-primary-light-100-alpha-700);
  }
  
  &:focus {
    background-color: var(--color-primary-light-1000);
    border-color: var(--color-primary);
    box-shadow: 0 0 0 3px var(--color-primary-alpha-800);
  }
}

.min {
  padding: 4px 8px;
  font-size: 12px;
  border-radius: 4px;
}

</style>
