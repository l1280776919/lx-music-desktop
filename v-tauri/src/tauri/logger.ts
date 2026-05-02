import { hasTauriContext } from './runtime'
import { writeAppLog, writeAppLogBatch } from './ipc'

type AppLogLevel = 'trace' | 'debug' | 'info' | 'warn' | 'error'
type AppLogEntry = {
  level: AppLogLevel
  target?: string | null
  message: string
  context?: Record<string, unknown> | null
}

const rawConsole = {
  log: console.log.bind(console),
  info: console.info.bind(console),
  warn: console.warn.bind(console),
  error: console.error.bind(console),
  debug: console.debug.bind(console),
}

const queue: AppLogEntry[] = []
let flushTimer: ReturnType<typeof setTimeout> | null = null
let isFlushing = false
let consoleHookInstalled = false

const normalizeText = (value: unknown): string => {
  if (value == null) return ''
  if (value instanceof Error) return [value.name, value.message].filter(Boolean).join(' | ')
  if (typeof value == 'string') return value
  try {
    return JSON.stringify(value)
  } catch {
    return String(value)
  }
}

const scheduleFlush = () => {
  if (flushTimer != null) return
  flushTimer = setTimeout(() => {
    flushTimer = null
    void flushLogs()
  }, 300)
}

const flushLogs = async() => {
  if (isFlushing || !queue.length || !hasTauriContext()) return
  isFlushing = true
  const entries = queue.splice(0, 100)
  try {
    await writeAppLogBatch(entries)
  } catch (err) {
    rawConsole.warn('write app log batch failed', err)
  } finally {
    isFlushing = false
    if (queue.length) scheduleFlush()
  }
}

const pushLog = (entry: AppLogEntry) => {
  if (!hasTauriContext()) return
  queue.push(entry)
  if (queue.length >= 20) {
    void flushLogs()
    return
  }
  scheduleFlush()
}

export const appLogger = {
  trace(message: string, context?: Record<string, unknown> | null, target = 'frontend.app') {
    pushLog({ level: 'trace', target, message, context })
  },
  debug(message: string, context?: Record<string, unknown> | null, target = 'frontend.app') {
    pushLog({ level: 'debug', target, message, context })
  },
  info(message: string, context?: Record<string, unknown> | null, target = 'frontend.app') {
    pushLog({ level: 'info', target, message, context })
  },
  warn(message: string, context?: Record<string, unknown> | null, target = 'frontend.app') {
    pushLog({ level: 'warn', target, message, context })
  },
  error(message: string, context?: Record<string, unknown> | null, target = 'frontend.app') {
    pushLog({ level: 'error', target, message, context })
  },
}

const createConsoleHook = (level: AppLogLevel, original: (...args: any[]) => void) => {
  return (...args: any[]) => {
    original(...args)
    const message = args.map(arg => normalizeText(arg)).filter(Boolean).join(' ')
    pushLog({
      level,
      target: 'frontend.console',
      message,
    })
  }
}

export const installAppLogger = () => {
  if (consoleHookInstalled) return
  consoleHookInstalled = true

  console.log = createConsoleHook('info', rawConsole.log)
  console.info = createConsoleHook('info', rawConsole.info)
  console.warn = createConsoleHook('warn', rawConsole.warn)
  console.error = createConsoleHook('error', rawConsole.error)
  console.debug = createConsoleHook('debug', rawConsole.debug)

  if (typeof window != 'undefined') {
    window.addEventListener('beforeunload', () => {
      void flushLogs()
    })
  }

  void writeAppLog({
    level: 'info',
    target: 'frontend.app',
    message: 'frontend logger initialized',
    context: {
      userAgent: typeof navigator != 'undefined' ? navigator.userAgent : '',
    },
  }).catch(err => rawConsole.warn('init app logger failed', err))
}

export default {
  error: (...args: any[]) => appLogger.error(args.map(arg => normalizeText(arg)).join(' '), null, 'frontend.electron-log'),
  warn: (...args: any[]) => appLogger.warn(args.map(arg => normalizeText(arg)).join(' '), null, 'frontend.electron-log'),
  info: (...args: any[]) => appLogger.info(args.map(arg => normalizeText(arg)).join(' '), null, 'frontend.electron-log'),
  verbose: (...args: any[]) => appLogger.debug(args.map(arg => normalizeText(arg)).join(' '), null, 'frontend.electron-log'),
  debug: (...args: any[]) => appLogger.debug(args.map(arg => normalizeText(arg)).join(' '), null, 'frontend.electron-log'),
  log: (...args: any[]) => appLogger.info(args.map(arg => normalizeText(arg)).join(' '), null, 'frontend.electron-log'),
}
