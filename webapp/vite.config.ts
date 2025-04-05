import { fileURLToPath, URL } from 'node:url';
import { sentryVitePlugin } from "@sentry/vite-plugin";

import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import vueDevTools from 'vite-plugin-vue-devtools';
import conditionalCompile from "vite-plugin-conditional-compiler";

// https://vite.dev/config/
export default defineConfig({
    build: {
        sourcemap: true,
    },
    plugins: [
        vue(),
        vueDevTools(),
        conditionalCompile(),
        sentryVitePlugin({
            org: 'starfoundry',
            project: 'starfoundry',
            authToken: process.env.SENTRY_AUTH_TOKEN,
        }),
    ],
    resolve: {
        alias: {
            '@': fileURLToPath(new URL('./src', import.meta.url))
        },
    },
    server: {
        hmr: {
            path: '/ws'
        },
        allowedHosts: [
            'industry.dev.starfoundry.space'
        ],
        host: '0.0.0.0',
        port: 1337,
    }
});
