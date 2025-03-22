import { createApp } from 'vue'

import Main from './main.vue'

const setup = () => {
  const app = createApp(Main)

  // 挂载页面
  app.mount('#app')
}
setup()
