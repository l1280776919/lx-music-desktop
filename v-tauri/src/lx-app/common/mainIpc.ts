const noop = () => {}

export function mainOn(_name: string, _listener: LX.IpcMainEventListener): void
export function mainOn<T>(_name: string, _listener: LX.IpcMainEventListenerParams<T>): void
export function mainOn<T>(_name: string, _listener: LX.IpcMainEventListenerParams<T>): void { noop() }

export function mainOnce(_name: string, _listener: LX.IpcMainEventListener): void
export function mainOnce<T>(_name: string, _listener: LX.IpcMainEventListenerParams<T>): void
export function mainOnce<T>(_name: string, _listener: LX.IpcMainEventListenerParams<T>): void { noop() }

export const mainOff = (_name: string, _listener: (...args: any[]) => void) => {
  noop()
}

export const mainOffAll = (_name: string) => {
  noop()
}

export function mainHandle(_name: string, _listener: LX.IpcMainInvokeEventListener): void
export function mainHandle<T>(_name: string, _listener: LX.IpcMainInvokeEventListenerParams<T>): void
export function mainHandle<V>(_name: string, _listener: LX.IpcMainInvokeEventListenerValue<V>): void
export function mainHandle<T, V>(_name: string, _listener: LX.IpcMainInvokeEventListenerParamsValue<T, V>): void
export function mainHandle<T, V>(_name: string, _listener: LX.IpcMainInvokeEventListenerParamsValue<T, V>): void { noop() }

export function mainHandleOnce(_name: string, _listener: LX.IpcMainInvokeEventListener): void
export function mainHandleOnce<T>(_name: string, _listener: LX.IpcMainInvokeEventListenerParams<T>): void
export function mainHandleOnce<V>(_name: string, _listener: LX.IpcMainInvokeEventListenerValue<V>): void
export function mainHandleOnce<T, V>(_name: string, _listener: LX.IpcMainInvokeEventListenerParamsValue<T, V>): void
export function mainHandleOnce<T, V>(_name: string, _listener: LX.IpcMainInvokeEventListenerParamsValue<T, V>): void { noop() }

export const mainHandleRemove = (_name: string) => {
  noop()
}

export function mainSend(_window: any, _name: string): void
export function mainSend<T>(_window: any, _name: string, _params: T): void
export function mainSend<T>(_window: any, _name: string, _params?: T): void {
  noop()
}
