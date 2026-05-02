<template>
  <div ref="dom_menu" :class="$style.menu">
    <ul :class="$style.list" role="toolbar">
      <li v-for="item in menus" :key="item.to" :class="$style.navItem" role="presentation">
        <router-link :class="[$style.link, {[$style.active]: $route.meta.name == item.name}]" role="tab" :aria-selected="$route.meta.name == item.name" :to="item.to" :aria-label="item.tips">
          <component :is="item.icon" :class="$style.icon" :size="20" stroke-width="2.5" />
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
import { Search, Disc, Trophy, Heart, Download, Settings } from 'lucide-vue-next'

export default {
  name: 'NavBar',
  setup() {
    const t = useI18n()
    const dom_menu = ref<HTMLElement>()

    const menus = computed(() => {
      return [
        {
          to: '/search',
          tips: t('search'),
          icon: Search,
          name: 'Search',
          enable: true,
        },
        {
          to: '/songList/list',
          tips: t('song_list'),
          icon: Disc,
          name: 'SongList',
          enable: true,
        },
        {
          to: '/leaderboard',
          tips: t('leaderboard'),
          icon: Trophy,
          name: 'Leaderboard',
          enable: true,
        },
        {
          to: '/list',
          tips: t('my_list'),
          icon: Heart,
          name: 'List',
          enable: true,
        },
        {
          to: '/download',
          tips: t('download'),
          icon: Download,
          enable: appSetting['download.enable'],
          name: 'Download',
        },
        {
          to: '/setting',
          tips: t('setting'),
          icon: Settings,
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
  transition: background-color 0.2s ease, color 0.2s ease, transform 0.2s ease;
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
    background-color: rgba(255, 255, 255, 0.035);
    color: var(--color-primary-dark-100);
    box-shadow: none;
    transform: translateX(2px);

    &::before {
      content: '';
      position: absolute;
      left: 8px;
      top: 50%;
      width: 3px;
      height: 18px;
      border-radius: 999px;
      transform: translateY(-50%);
      background-color: var(--color-primary);
      box-shadow: 0 0 10px var(--color-primary-alpha-300);
    }
    
    .icon {
      transform: scale(1.06);
    }
  }
}

.icon {
  width: 20px;
  height: 20px;
  flex: none;
  transition: transform 0.2s ease, color 0.2s ease;
  fill: none;
  stroke: currentColor;
}

.label {
  margin-left: 14px;
  font-size: 14px;
  font-weight: 500;
  .mixin-ellipsis-1();
}

</style>
