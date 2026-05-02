<template>
  <div :class="$style.leaderboard">
    <div :class="$style.lists">
      <div :class="$style.listsSelect">
        <base-selection :model-value="source" :class="$style.select" :list="sourceList" item-key="id" item-name="name" @update:model-value="handleToggleSource" />
      </div>
      <BoardList ref="boardListRef" :board-id="boardId" :source="source" @show-menu="$refs.musicListRef?.hideMenu()" />
    </div>
    <div :class="$style.list">
      <MusicList ref="musicListRef" :source="source" :board-id="boardId" @show-menu="$refs.boardListRef?.hideMenu()" />
    </div>
  </div>
</template>

<script>
import { computed, ref } from '@common/utils/vueTools'
import { getLeaderboardSetting, setLeaderboardSetting } from '@renderer/utils/data'
import BoardList from './BoardList/index.vue'
import MusicList from './MusicList/index.vue'
import { sources } from '@renderer/store/leaderboard/state'
import { sourceNames } from '@renderer/store'
import { useRoute, useRouter } from '@common/utils/vueRouter'


const source = ref('')
const boardId = ref(null)

const verifyQueryParams = async function(to, from, next) {
  let _source = to.query.source
  let _boardId = to.query.boardId

  if (_source == null) {
    const setting = await getLeaderboardSetting()
    if (_source == null) {
      _source = setting.source
      _boardId = setting.boardId
    }
    next({
      path: to.path,
      query: { ...to.query, source: _source, boardId: _boardId },
    })
    return
  }
  next()
  source.value = _source
  boardId.value = _boardId
  void setLeaderboardSetting({ source: _source, boardId: _boardId })
}


export default {
  components: {
    BoardList,
    MusicList,
  },
  beforeRouteEnter: verifyQueryParams,
  beforeRouteUpdate: verifyQueryParams,
  setup() {
    const musicListRef = ref(null)
    const boardListRef = ref(null)
    const sourceList = computed(() => {
      return sources.map(s => ({ id: s, name: sourceNames.value[s] }))
    })
    const router = useRouter()
    const route = useRoute()
    const handleToggleSource = (id) => {
      void router.replace({
        path: route.path,
        query: {
          source: id,
        },
      })
    }

    return {
      source,
      boardId,
      sourceList,
      handleToggleSource,
      musicListRef,
      boardListRef,
    }
  },
}
</script>

<style lang="less" module>
@import '@renderer/assets/styles/layout.less';

.leaderboard {
  height: 100%;
  min-height: 0;
  display: flex;
  position: relative;
  gap: 14px;
  padding: 16px 18px 18px;
  box-sizing: border-box;
}
.header {
  flex: none;
  width: 100%;
  display: flex;
  flex-flow: row nowrap;

}
.tab {
  flex: auto;
}
.select {
  flex: none;
  width: 80px;
}
.content {
  flex: auto;
  display: flex;
  overflow: hidden;
  flex-flow: column nowrap;
}

.lists {
  flex: none;
  width: clamp(180px, 15.5%, 220px);
  display: flex;
  flex-flow: column nowrap;
  min-height: 0;
  border-radius: 18px;
  background-color: rgba(255, 255, 255, 0.38);
  border: 1px solid rgba(255, 255, 255, 0.42);
  box-shadow: 0 10px 24px rgba(0, 0, 0, 0.04);
  overflow: hidden;
}
.listsHeader {
  position: relative;
}

.listsSelect {
  font-size: 12px;

  &:hover {
    :global(.icon) {
      opacity: 1;
    }
  }

  >:global(.content) {
    display: block;
    width: 100%;
  }
  :global(.label-content) {
    background-color: rgba(255, 255, 255, 0.4) !important;
    line-height: 38px;
    height: 38px;
    border-radius: 14px;
    margin: 10px;
    &:hover {
      background: rgba(255, 255, 255, 0.62) !important;
    }
  }
  :global(.label) {
    color: var(--color-font) !important;
  }
  :global(.icon) {
    opacity: .6;
    transition: opacity .3s ease;
  }

  :global(.selection-list) {
    max-height: 500px;
    box-shadow: 0 10px 24px rgba(0, 0, 0, .08);
    border-radius: 14px;
    li {
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
  // line-height: 38px;
  // padding: 0 10px;
  border-bottom: var(--color-list-header-border-bottom);
  flex: none;
}

.list {
  position: relative;
  overflow: hidden;
  height: 100%;
  min-height: 0;
  flex: auto;
  display: flex;
  flex-flow: column nowrap;
  border-radius: 22px;
  background-color: rgba(255, 255, 255, 0.24);
  border: 1px solid rgba(255, 255, 255, 0.34);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.36);
  // .noItem {

  // }
}

</style>
