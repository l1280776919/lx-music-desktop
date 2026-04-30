<template>
  <div ref="dom_menu" :class="$style.menu">
    <ul :class="$style.list" role="toolbar">
      <li v-for="item in menus" :key="item.to" :class="$style.navItem" role="presentation">
        <router-link :class="[$style.link, {[$style.active]: $route.meta.name == item.name}]" role="tab" :aria-selected="$route.meta.name == item.name" :to="item.to" :aria-label="item.tips">
          <svg :class="$style.icon" version="1.1" xmlns="http://www.w3.org/2000/svg" xlink="http://www.w3.org/1999/xlink" :viewBox="item.iconSize" space="preserve">
            <use :xlink:href="item.icon" />
          </svg>
          <span :class="$style.label">{{ item.tips }}</span>
        </router-link>
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import { appSetting } from '@renderer/store/setting'
import { useI18n } from '@root/lang'
import { ref, computed } from '@common/utils/vueTools'
import { useIconSize } from '@renderer/utils/compositions/useIconSize'

export default {
  name: 'NavBar',
  setup() {
    const t = useI18n()
    const dom_menu = ref<HTMLElement>()
    const iconSize = useIconSize(dom_menu, 0.32)

    const menus = computed(() => {
      const size = iconSize.value
      return [
        {
          to: '/search',
          tips: t('search'),
          icon: '#icon-search-2',
          iconSize: '0 0 425.2 425.2',
          size,
          name: 'Search',
          enable: true,
        },
        {
          to: '/songList/list',
          tips: t('song_list'),
          icon: '#icon-album',
          iconSize: '0 0 425.2 425.2',
          size,
          name: 'SongList',
          enable: true,
        },
        {
          to: '/leaderboard',
          tips: t('leaderboard'),
          icon: '#icon-leaderboard',
          iconSize: '0 0 425.22 425.2',
          size,
          name: 'Leaderboard',
          enable: true,
        },
        {
          to: '/list',
          tips: t('my_list'),
          icon: '#icon-love',
          iconSize: '0 0 444.87 391.18',
          size,
          name: 'List',
          enable: true,
        },
        {
          to: '/download',
          tips: t('download'),
          icon: '#icon-download-2',
          iconSize: '0 0 425.2 425.2',
          size,
          enable: appSetting['download.enable'],
          name: 'Download',
        },
        {
          to: '/setting',
          tips: t('setting'),
          icon: '#icon-setting',
          iconSize: '0 0 493.23 436.47',
          size,
          enable: true,
          name: 'Setting',
        },
      ].filter(m => m.enable)
    })
    return {
      appSetting,
      menus,
      dom_menu,
    }
  },
}
</script>

<style lang="less" module>
@import '@renderer/assets/styles/layout.less';

.menu {
  flex: auto;
  padding: 15px 12px;
}
.list {
  -webkit-app-region: no-drag;
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.navItem {
  position: relative;
  width: 100%;
}
.link {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  padding: 12px 16px;
  box-sizing: border-box;
  transition: all 0.3s cubic-bezier(0.2, 0.8, 0.2, 1);
  color: var(--color-font);
  cursor: pointer;
  outline: none;
  border-radius: 10px;
  text-decoration: none;
  position: relative;
  overflow: hidden;

  &:hover {
    background-color: rgba(0, 0, 0, 0.04);
    transform: translateX(2px);
  }
  &.active {
    background-color: var(--color-primary-alpha-200);
    color: var(--color-primary-dark-100);
    box-shadow: 0 2px 8px var(--color-primary-alpha-200);
    transform: translateX(4px);
    font-weight: 600;
    
    .icon {
      fill: var(--color-primary-dark-100);
      transform: scale(1.1);
    }
  }
}

.icon {
  width: 20px;
  height: 20px;
  flex: none;
  fill: currentColor;
  transition: all 0.3s cubic-bezier(0.2, 0.8, 0.2, 1);
}

.label {
  margin-left: 14px;
  font-size: 14px;
  font-weight: inherit;
  .mixin-ellipsis-1();
}

</style>
