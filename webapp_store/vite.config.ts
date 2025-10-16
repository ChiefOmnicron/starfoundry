import { sentryVitePlugin } from "@sentry/vite-plugin";
import { defineConfig } from "vite";
import { tanstackRouter } from '@tanstack/router-plugin/vite';
import react from "@vitejs/plugin-react";

import { fileURLToPath, URL } from 'node:url';

export default defineConfig({
    plugins: [
        tanstackRouter({
            target: 'react',
            autoCodeSplitting: true,
        }),
        react(),
        sentryVitePlugin({
            org: "starfoundry",
            project: "sf-store",
            telemetry: false,
        })
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
        allowedHosts: ['store.dev.starfoundry.space'],
        host: '0.0.0.0',
        port: 1338,
    },

    build: {
        sourcemap: true
    }
});
