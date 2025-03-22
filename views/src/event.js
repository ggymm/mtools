export const sendDragWindowEvent = () => {
  window['ipc'].postMessage(
    JSON.stringify({
      event: 'drag-window'
    })
  )
}
