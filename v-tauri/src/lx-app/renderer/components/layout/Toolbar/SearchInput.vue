<template>
  <material-search-input v-model="searchText" :placeholder="`${t('search')}...`" :list="tipList" :visible-list="visibleList" @event="handleEvent" />
</template>

<script>
import music from '@renderer/utils/musicSdk'
import { debounce } from '@common/utils'
import {
  ref,
  watch,
  nextTick,
} from '@common/utils/vueTools'
import { useRouter, useRoute } from '@common/utils/vueRouter'
import { useI18n } from '@root/lang'
import { appSetting } from '@renderer/store/setting'
import { searchText as _searchText } from '@renderer/store/search/state'
import { setSearchText } from '@renderer/store/search/action'
import { getSearchSetting } from '@renderer/utils/data'

export default {
  setup() {
    const t = useI18n()
    const searchText = ref('')
    const visibleList = ref(false)
    const tipList = ref([])
    let isFocused = false

    const route = useRoute()
    const router = useRouter()

    watch(() => route.name, (newValue, oldValue) => {
      if (oldValue == 'Search' && newValue != 'SongListDetail') {
        setTimeout(() => {
          if (appSetting['odc.isAutoClearSearchInput'] && searchText.value) searchText.value = ''
          if (appSetting['odc.isAutoClearSearchList']) setSearchText('')
        })
      }
    })

    watch(_searchText, (newValue, oldValue) => {
      searchText.value = newValue
      if (newValue !== searchText.value) searchText.value = newValue
    })
    watch(searchText, () => {
      handleTipSearch()
    })


    const tipSearch = debounce(async() => {
      const { temp_source } = await getSearchSetting()
      if (!music[temp_source]?.tipSearch) return
      if (searchText.value === '') {
        tipList.value = []
        music[temp_source].tipSearch.cancelTipSearch()
        return
      }
      music[temp_source].tipSearch.search(searchText.value).then(list => {
        tipList.value = list
      }).catch(() => {})
    }, 160)

    const handleTipSearch = () => {
      if (!visibleList.value && isFocused) visibleList.value = true
      tipSearch()
    }

    const handleSearch = () => {
      visibleList.value &&= false
      const text = searchText.value.trim()
      if (!text && route.path != '/search') {
        setSearchText('')
        return
      }
      const query = route.path == '/search'
        ? {
            ...route.query,
            text,
            page: 1,
          }
        : { text }
      if (route.path == '/search' && route.query.text === text && `${route.query.page ?? 1}` === '1') return
      const action = route.path == '/search' ? router.replace : router.push
      action({
        path: '/search',
        query,
      }).catch(_ => _)
    }

    const handleEvent = ({ action, data }) => {
      switch (action) {
        case 'focus':
          isFocused = true
          visibleList.value ||= true
          if (searchText.value) handleTipSearch()
          break
        case 'blur':
          isFocused = false
          setTimeout(() => {
            visibleList.value &&= false
          }, 50)
          break
        case 'submit':
          handleSearch()
          break
        case 'listClick':
          searchText.value = tipList.value[data]
          void nextTick(handleSearch)
      }
    }

    return {
      t,
      searchText,
      visibleList,
      tipList,
      handleEvent,
    }
  },
}

</script>
