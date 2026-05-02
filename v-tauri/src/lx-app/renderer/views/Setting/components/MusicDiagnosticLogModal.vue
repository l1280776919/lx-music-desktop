<template lang="pug">
material-modal(:show="modelValue" width="92%" height="88%" teleport="#overlay-root" @close="handleClose")
  main(:class="$style.main")
    h2(:class="$style.title") {{ $t('setting__other_music_log_title') }}
    p(:class="$style.path")
      span(:class="$style.pathLabel") {{ $t('setting__other_music_log_path_label') }}
      span.auto-hidden.hover(:class="$style.pathText" @click="handleOpenLogPath") {{ logPath || '-' }}
    div(:class="$style.actions")
      base-btn.btn(min @click="refreshLog") {{ $t('setting__other_music_log_refresh_btn') }}
      base-btn.btn.gap-left(min :disabled="!logEntries.length" @click="handleCopyLog") {{ $t('setting__other_music_log_copy_btn') }}
      base-btn.btn.gap-left(min :disabled="!logPath" @click="handleOpenLogPath") {{ $t('setting__other_music_log_open_btn') }}
      base-btn.btn.gap-left(min :disabled="!logEntries.length" @click="handleClearLog") {{ $t('setting__other_music_log_clear_btn') }}
    div.scroll(:class="$style.panel" @scroll="handleScroll")
      template(v-if="logEntries.length")
        div.select(v-for="(entry, index) in logEntries" :key="`${index}_${entry.raw}`" :class="$style.row")
          span(:class="$style.timeText") [{{ entry.time }}]
          span(:class="[$style.levelText, $style[`level_${entry.levelClass}`]]") {{ entry.level }}
          span(v-if="entry.target" :class="$style.target") [{{ entry.target }}]
          span(:class="$style.message") {{ entry.content }}
        div(v-if="hasMore" :class="$style.loadMore")
          base-btn.btn(min @click="loadMoreLog" :disabled="loadingMore") {{ loadingMore ? '...' : '加载更多' }}
      div(v-else :class="$style.empty") {{ $t('setting__other_music_log_empty') }}
</template>

<script>
import { computed, ref, watch } from '@common/utils/vueTools'
import { clipboardWriteText } from '@common/utils/electron'
import {
  clearAppLog,
  getAppLogPath,
  openDirInExplorer,
  readAppLog,
} from '@renderer/utils/ipc'
import { dialog } from '@renderer/plugins/Dialog'
import { useI18n } from '@renderer/plugins/i18n'

const LOG_LINE_RE = /^(\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z)?(?:[+-]\d{2}:\d{2})?)\s+([A-Z]+)\s+(.*)$/

export default {
  name: 'MusicDiagnosticLogModal',
  props: {
    modelValue: {
      type: Boolean,
      required: true,
    },
  },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    const t = useI18n()
    const logEntries = ref([])
    const logPath = ref('')
    const offset = ref(0)
    const hasMore = ref(false)
    const loadingMore = ref(false)

    const parseLogLine = (line) => {
      const match = line.match(LOG_LINE_RE)
      if (!match) {
        return {
          raw: line,
          time: '-',
          level: 'TEXT',
          levelClass: 'text',
          target: '',
          content: line,
        }
      }

      const [, time, level, rest] = match
      const targetMatch = rest.match(/^([^:]+):\s*(.*)$/)
      let target = targetMatch ? targetMatch[1] : ''
      let content = targetMatch ? targetMatch[2] : rest

      const logTargetMatch = content.match(/^log_target="([^"]+)"\s*(.*)$/)
      if (logTargetMatch) {
        target = logTargetMatch[1]
        content = logTargetMatch[2]
      }

      return {
        raw: line,
        time: time.replace('T', ' '),
        level,
        levelClass: level.toLowerCase(),
        target,
        content,
      }
    }
    const buildLogEntries = (lines) => {
      return lines.map(parseLogLine)
    }
    const buildDisplayLogText = (entries) => {
      return entries.map(entry => [entry.time, entry.level, entry.target ? `[${entry.target}]` : '', entry.content].filter(Boolean).join('  ')).join('\n')
    }

    const refreshLog = async() => {
      const [path, lines] = await Promise.all([
        getAppLogPath(),
        readAppLog(0, 500),
      ])
      logPath.value = path
      logEntries.value = buildLogEntries(lines)
      offset.value = lines.length
      hasMore.value = lines.length === 500
    }

    const loadMoreLog = async() => {
      if (loadingMore.value || !hasMore.value) return
      loadingMore.value = true
      try {
        const lines = await readAppLog(offset.value, 500)
        logEntries.value.push(...buildLogEntries(lines))
        offset.value += lines.length
        hasMore.value = lines.length === 500
      } finally {
        loadingMore.value = false
      }
    }

    const handleScroll = (e) => {
      const target = e.target
      if (target.scrollHeight - target.scrollTop - target.clientHeight < 50) {
        void loadMoreLog()
      }
    }

    watch(() => props.modelValue, value => {
      if (value) void refreshLog()
    }, { immediate: true })

    const handleClose = () => {
      emit('update:modelValue', false)
    }
    const handleCopyLog = () => {
      clipboardWriteText(buildDisplayLogText(logEntries.value))
    }
    const handleOpenLogPath = () => {
      if (!logPath.value) return
      void openDirInExplorer(logPath.value)
    }
    const handleClearLog = async() => {
      if (!await dialog.confirm({
        message: t('setting__other_music_log_clear_confirm'),
        cancelButtonText: t('cancel_button_text'),
        confirmButtonText: t('setting__other_music_log_clear_btn'),
      })) return
      await clearAppLog()
      await refreshLog()
    }

    return {
      logEntries,
      hasMore,
      loadingMore,
      logPath,
      refreshLog,
      loadMoreLog,
      handleScroll,
      handleClose,
      handleCopyLog,
      handleOpenLogPath,
      handleClearLog,
    }
  },
}
</script>

<style lang="less" module>
.main {
  height: 100%;
  min-height: 520px;
  display: flex;
  flex-flow: column nowrap;
  padding: 18px 20px;
  background: #111;
  color: #eee;
}

.title {
  font-size: 18px;
  font-weight: 600;
  line-height: 1.3;
}

.path {
  margin-top: 14px;
  line-height: 1.5;
  font-size: 13px;
  color: #aaa;
}

.pathLabel {
  font-weight: 600;
}

.pathText {
  text-decoration: underline;
  color: #60a5fa;
}

.actions {
  margin-top: 12px;
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.panel {
  flex: 1 1 auto;
  margin-top: 14px;
  overflow: auto;
  border-radius: 10px;
  border: 1px solid #333;
  background: #1a1a1a;
  padding: 10px;
}

.row {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding: 4px 6px;
  font-family: Consolas, "Courier New", monospace;
  font-size: 13px;
  line-height: 1.5;
  word-break: break-word;
  white-space: pre-wrap;
}

.row:hover {
  background: #2a2a2a;
  border-radius: 4px;
}

.timeText {
  color: #888;
  flex-shrink: 0;
}

.levelText {
  font-weight: 700;
  flex-shrink: 0;
  min-width: 48px;
}

.level_trace,
.level_debug,
.level_text {
  color: #999;
}

.level_info {
  color: #4ade80;
}

.level_warn {
  color: #facc15;
}

.level_error {
  color: #f87171;
}

.target {
  font-weight: 700;
  color: #60a5fa;
  flex-shrink: 0;
}

.message {
  color: #eee;
}

.empty {
  padding: 32px 20px;
  text-align: center;
  font-size: 14px;
  color: #999;
}

.loadMore {
  padding: 14px;
  display: flex;
  justify-content: center;
}
</style>

