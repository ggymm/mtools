import { createApp } from 'vue'

import App from './app.vue'

const setup = () => {
    const app = createApp(App)

    // 挂载页面
    app.mount('#app')
}
setup()
