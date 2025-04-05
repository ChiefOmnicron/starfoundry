import { createApp } from 'vue'
import { Events } from '@/event_bus';

import * as Sentry from "@sentry/vue";

import App from './App.vue'
import router from './router'

export const events = new Events();

let app = createApp(App);

if (import.meta.env.VITE_SENTRY) {
    Sentry.init({
        app,
        dsn: import.meta.env.VITE_SENTRY
    });
}

app
    .use(router)
    .mount('#app');
