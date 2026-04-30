import { log } from './utils'

const ignoreErrorMessage = [
  'Possible side-effect in debug-evaluate',
  'Unexpected end of input',
]

const handleError = (err: any) => {
  if (ignoreErrorMessage.includes(err?.message)) return
  console.error('An uncaught error occurred!')
  console.error(err)
  log.error(err)
}

const handleRejection = (reason: any, promise?: Promise<unknown>) => {
  console.error('Unhandled Rejection at: Promise ', promise)
  console.error(' reason: ', reason)
  log.error(reason)
}

if (typeof process != 'undefined' && typeof process.on == 'function') {
  process.on('uncaughtException', handleError)
  process.on('unhandledRejection', (reason, p) => {
    handleRejection(reason, p)
  })
} else if (typeof window != 'undefined') {
  window.addEventListener('error', event => {
    handleError(event.error ?? new Error(event.message))
  })
  window.addEventListener('unhandledrejection', event => {
    handleRejection(event.reason)
  })
}
