export const dragWindow = () => {
  window['ipc'].postMessage(
    JSON.stringify({
      event: 'drag-window'
    })
  )
}
