import { defineConfig } from "vite";
import { TanStackRouterVite } from '@tanstack/router-plugin/vite';
import react from "@vitejs/plugin-react";

import { fileURLToPath, URL } from 'node:url';

// https://vite.dev/config/
export default defineConfig({
    plugins: [
        TanStackRouterVite({
            target: 'react',
            autoCodeSplitting: true,
        }),
        react(),
    ],
    define: {
        '__APP_VERSION__': JSON.stringify(process.env.npm_package_version),
    },
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
