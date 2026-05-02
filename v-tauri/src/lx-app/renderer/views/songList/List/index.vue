<template>
  <div :class="$style.container">
    <div :class="$style.header">
      <div :class="$style.left">
        <tag-list :source="source" :tag-id="tagId" :sort-id="sortId" />
        <sort-tab :source="source" :tag-id="tagId" :sort-id="sortId" />
      </div>
      <base-btn :class="$style.btn" outline min @click="visibleOpenSongListModal = true">{{ $t('songlist__import_input_show_btn') }}</base-btn>
      <base-selection :model-value="source" :class="$style.select" :list="sourceList" item-key="id" item-name="name" @update:model-value="handleToggleSource" />
    </div>
    <list-view :source="source" :tag-id="tagId" :sort-id="sortId" :page="page" />
    <open-list-modal v-model="visibleOpenSongListModal" :source-list="sourceList" />
  </div>
</template>

<script lang="ts">
import { computed, ref } from '@common/utils/vueTools'
import { getSongListSetting, setSongListSetting } from '@renderer/utils/data'
import TagList from './components/TagList.vue'
import SortTab from './components/SortTab.vue'
import OpenListModal from './components/OpenListModal.vue'
import ListView from './ListView.vue'
import { sources, listInfo, isVisibleListDetail } from '@renderer/store/songList/state'
import { sourceNames } from '@renderer/store'
import { useRoute, useRouter } from '@common/utils/vueRouter'

const source = ref<LX.OnlineSource>('kw')
const tagId = ref<string>('')
const sortId = ref<string>('')
const page = ref<number>(1)


interface Query {
  source?: string
  tagId?: string
  sortId?: string
  page?: string
}

const verifyQueryParams = async function(this: any, to: { query: Query, path: string }, from: any, next: (route?: { path: string, query: Query }) => void) {
  let _source = to.query.source
  let _tagId = to.query.tagId
  let _sortId = to.query.sortId
  let _page: string | undefined = to.query.page

  if (isVisibleListDetail.value) {
    next({ path: '/songList/detail', query: {} })
    return
  } else if (_source == null) {
    if (listInfo.key) {
      _source = listInfo.source
      _tagId = listInfo.tagId
      _sortId = listInfo.sortId
      _page = listInfo.page.toString()
    } else {
      const setting = await getSongListSetting()
      _source = setting.source
      _tagId = setting.tagId
      _sortId = setting.sortId
      _page = '1'
    }

    next({
      path: to.path,
      query: { ...to.query, source: _source, tagId: _tagId, sortId: _sortId, page: _page },
    })
    return
  }
  next()
  source.value = _source as LX.OnlineSource
  tagId.value = _tagId ?? ''
  sortId.value = _sortId ?? ''
  page.value = _page ? parseInt(_page) : 1
  void setSongListSetting({ source: _source, tagId: _tagId, sortId: _sortId })
}


export default {
  components: {
    TagList,
    SortTab,
    ListView,
    OpenListModal,
  },
  beforeRouteEnter: verifyQueryParams,
  beforeRouteUpdate: verifyQueryParams,
  setup() {
    const visibleOpenSongListModal = ref(false)

    const sourceList = computed(() => {
      return sources.map(s => ({ id: s, name: sourceNames.value[s] }))
    })
    const router = useRouter()
    const route = useRoute()
    const handleToggleSource = (id: LX.OnlineSource) => {
      if (id == source.value) return
      void router.replace({
        path: route.path,
        query: {
          source: id,
          tagId: '',
        },
      })
    }

    return {
      source,
      tagId,
      sortId,
      page,
      sourceList,
      handleToggleSource,
      visibleOpenSongListModal,
    }
  },
}
</script>

<style lang="less" module>
@import '@renderer/assets/styles/layout.less';

.container {
  height: 100%;
  display: flex;
  flex-flow: column nowrap;
  position: relative;
  min-height: 0;
  padding: 16px 18px 18px;
  gap: 14px;
  box-sizing: border-box;
}
.header {
  flex: none;
  width: 100%;
  display: flex;
  flex-flow: row nowrap;
  align-items: center;
  gap: 12px;
  padding: 12px 14px;
  border-radius: 20px;
  background-color: rgba(255, 255, 255, 0.28);
  border: 1px solid rgba(255, 255, 255, 0.36);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.38);
}
.left {
  flex: auto;
  display: flex;
  flex-flow: row nowrap;
  min-width: 0;
  gap: 8px;
}

.btn {
  color: var(--color-font);
  transition: color @transition-fast;
  background: rgba(255, 255, 255, 0.42) !important;
  border-radius: 12px !important;
  border-color: rgba(255, 255, 255, 0.4) !important;
  &:hover {
    color: var(--color-primary-font-hover);
    background: rgba(255, 255, 255, 0.68) !important;
  }
}


.select {
  font-size: 12px;
  width: auto;
  flex: none;
  padding: 0 5px;

  &:hover {
    :global(.icon) {
      opacity: 1;
    }
  }


  :global {
    .label-content {
      background-color: rgba(255, 255, 255, 0.42) !important;
      transition: color @transition-fast;
      color: var(--color-font);
      border-radius: 12px;
      padding: 0 12px;
      &:hover {
        background-color: rgba(255, 255, 255, 0.68) !important;
        color: var(--color-primary-font-hover);
        .icon {
          opacity: 1;
        }
      }
    }
    // .label {
    //   color: var(--color-font) !important;
    // }
    .icon {
      svg {
        width: .8em;
      }
      // opacity: .6;
      // transition: color @transition-fast;
      // color: var(--color-font-label);
    }

    .selection-list {
      max-height: 500px;
      box-shadow: 0 10px 24px rgba(0, 0, 0, .08);
      border-radius: 14px;
      li {
        text-align: center;
        line-height: 38px;
        font-size: 13px;
        &:hover {
          background-color: rgba(255, 255, 255, 0.75);
        }
        &:active {
          background-color: rgba(255, 255, 255, 0.9);
        }
      }
    }
  }
}

</style>
