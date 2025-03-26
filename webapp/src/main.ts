import { createApp } from 'vue'
import { Events } from '@/event_bus';

import App from './App.vue'
import router from './router'

export const events = new Events();

let app = createApp(App);

app
    .use(router)
    .mount('#app');
