<template>
  <div :class="$style.container">
    <div :class="$style.header">
      <base-tab v-model="source" :list="sources" @change="handleSourceChange" />
      <base-tab v-model="searchType" :list="searchTypes" @change="handleTypeChange" />
    </div>
    <div :class="$style.main">
      <song-list-list v-if="searchType == 'songlist'" v-show="searchText" :page="page" :source-id="source" />
      <music-list v-else v-show="searchText" :page="page" :source-id="source" />
      <blank-view :visible="!searchText" :source="source" />
    </div>
  </div>
</template>

<script>
import { useRoute, useRouter } from '@common/utils/vueRouter'
import { searchText } from '@renderer/store/search/state'
import { getSearchSetting, setSearchSetting } from '@renderer/utils/data'
import { sources as _sources } from '@renderer/store/search/music'

import MusicList from './MusicList/index.vue'
import SongListList from './SongListList/index.vue'
import BlankView from './components/BlankView.vue'
import { computed, ref, watch } from '@common/utils/vueTools'
import { sourceNames } from '@renderer/store'

const source = ref('kw')
const searchType = ref(null)
const page = ref(1)

export default {
  components: {
    MusicList,
    SongListList,
    BlankView,
  },
  setup() {
    const route = useRoute()
    const router = useRouter()

    watch(() => route.query, async(query) => {
      let nextSource = typeof query.source == 'string' ? query.source : null
      let nextType = typeof query.type == 'string' ? query.type : null
      const nextText = typeof query.text == 'string' ? query.text : ''
      const nextPage = Number.parseInt(`${query.page ?? 1}`, 10) || 1

      if (nextSource == null || nextType == null) {
        const setting = await getSearchSetting()
        nextSource ??= setting.source
        nextType ??= setting.type
        await router.replace({
          path: route.path,
          query: {
            ...query,
            source: nextSource,
            type: nextType,
            page: nextPage,
          },
        })
        return
      }

      source.value = nextSource
      searchType.value = nextType
      page.value = nextPage
      if (searchText.value !== nextText) searchText.value = nextText
      void setSearchSetting({ source: nextSource, type: nextType })
    }, {
      immediate: true,
    })

    const sources = _sources.map(id => {
      return {
        id,
        label: sourceNames.value[id],
      }
    })
    const handleSourceChange = (id) => {
      if (source.value == id && page.value == 1) return
      void router.replace({
        path: route.path,
        query: {
          ...route.query,
          source: id,
          page: 1,
        },
      })
    }

    const searchTypes = computed(() => {
      return [
        { label: window.i18n.t('search__type_music'), id: 'music' },
        { label: window.i18n.t('search__type_songlist'), id: 'songlist' },
      ]
    })
    const handleTypeChange = (type) => {
      if (searchType.value == type && page.value == 1) return
      void router.replace({
        path: route.path,
        query: {
          ...route.query,
          type,
          page: 1,
        },
      })
    }


    return {
      sources,
      source,
      handleSourceChange,
      searchTypes,
      searchType,
      handleTypeChange,
      page,
      searchText,
    }
  },
}


</script>

<style lang="less" module>
.container {
  display: flex;
  flex-flow: column nowrap;
  height: 100%;
  min-height: 0;
}

.header {
  // padding: 5px 0;
  flex: none;
  display: flex;
  flex-flow: row nowrap;
  justify-content: space-between;
}

.main {
  position: relative;
  display: flex;
  flex: auto;
  min-height: 0;
  overflow: hidden;
}
</style>
