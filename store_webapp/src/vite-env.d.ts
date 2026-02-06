/// <reference types="vite/client" />
interface ImportMetaEnv {
    readonly VITE_APP_TITLE: string;
    readonly SENTRY_DSN: string;
}

interface ImportMeta {
    readonly env: ImportMetaEnv;
}
