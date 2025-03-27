export const emitDragWindow = () => {
  window['ipc'].postMessage(
    JSON.stringify({
      event: 'main:drag:window'
    })
  )
}

export const emitToggleSetting = () => {
  window['ipc'].postMessage(
    JSON.stringify({
      event: 'main:toggle:setting'
    })
  )
}
