<template>
  <button
    :class="[$style.btn, {[$style.min]: min}, {[$style.outline]: outline}]"
    tabindex="0"
    :disabled="disabled"
  >
    <slot />
  </button>
</template>

<script>
export default {
  props: {
    min: {
      type: Boolean,
    },
    outline: {
      type: Boolean,
      default: false,
    },
    disabled: {
      type: Boolean,
      default: false,
    },
  },
}
</script>


<style lang="less" module>
@import '@renderer/assets/styles/layout.less';

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  padding: 8px 18px;
  color: var(--color-button-font);
  outline: none;
  transition: all 0.2s ease;
  background-color: var(--color-button-background);
  font-size: 14px;
  font-weight: 500;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05);

  &[disabled] {
    opacity: .4;
    cursor: default;
    box-shadow: none;
  }

  &.outline {
    background-color: transparent;
    border: 1px solid var(--color-primary-alpha-400);
    box-shadow: none;
    
    &:hover {
      border-color: var(--color-primary);
    }
  }

  &:hover:not([disabled]) {
    background-color: var(--color-button-background-hover);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    transform: translateY(-1px);
  }
  &:active:not([disabled]) {
    background-color: var(--color-button-background-active);
    transform: translateY(1px);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  }
}

.min {
  padding: 4px 12px;
  font-size: 12px;
  border-radius: 4px;
}

</style>
