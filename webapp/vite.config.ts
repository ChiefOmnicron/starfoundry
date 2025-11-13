import { fileURLToPath, URL } from 'node:url';

import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import vueDevTools from 'vite-plugin-vue-devtools';
import conditionalCompile from 'vite-plugin-conditional-compiler';

// https://vite.dev/config/
export default defineConfig({
    plugins: [
        vue(),
        vueDevTools(),
        conditionalCompile(),
    ],
    resolve: {
        alias: {
            '@': fileURLToPath(new URL('./src', import.meta.url)),
        },
    },
    server: {
        hmr: {
            path: '/ws',
        },
        allowedHosts: ['industry.dev.starfoundry.space'],
        host: '0.0.0.0',
        port: 1337,
    },
});
