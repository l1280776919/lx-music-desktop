const noop = () => {}

const log = {
  error: console.error.bind(console),
  warn: console.warn.bind(console),
  info: console.info.bind(console),
  verbose: console.debug ? console.debug.bind(console) : console.log.bind(console),
  debug: console.debug ? console.debug.bind(console) : console.log.bind(console),
  silly: noop,
  transports: {},
}

export default log

