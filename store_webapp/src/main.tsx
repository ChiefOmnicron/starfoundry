import '@mantine/core/styles.css';
import '@mantine/dates/styles.css';
import '@/style.css';

import { StrictMode } from 'react';
import { routeTree } from '@/routeTree.gen';
import { RouterProvider, createRouter } from '@tanstack/react-router';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { AuthProvider, useAuth, type AuthContext } from './auth';

import { createRoot } from 'react-dom/client';

import * as Sentry from "@sentry/react";

// Create a new router instance
const router = createRouter({
    routeTree,
    context: {
        auth: undefined!,
        queryClient: undefined!,
    },
});

if (import.meta.env.PROD) {
    Sentry.init({
        dsn: import.meta.env.VITE_SENTRY_STORE_DSN,
        sendDefaultPii: true,
        integrations: [
            Sentry.tanstackRouterBrowserTracingIntegration(router),
        ],
        tracesSampleRate: 0.1,
        tracePropagationTargets: [/^\//, /^https:\/\/store\.rcimade\.space\/api/],
        release: 'beta',
    });
}

// Register the router instance for type safety
declare module '@tanstack/react-router' {
    interface Register {
        router: typeof router
    }
}

function Inner() {
    const auth = useAuth();
    const queryClient = new QueryClient();

    return (
        <QueryClientProvider client={queryClient}>
            <RouterProvider router={router} context={{ auth, queryClient }} />
        </QueryClientProvider>
    )
}

function App() {
    return (
        <AuthProvider>
            <Inner />
        </AuthProvider>
    );
}

// Render the app
const rootElement = document.getElementById('root')!;
if (!rootElement.innerHTML) {
    const root = createRoot(
        rootElement,
        {
            // Callback called when an error is thrown and not caught by an ErrorBoundary.
            onUncaughtError: Sentry.reactErrorHandler((error, errorInfo) => {
                console.warn('Uncaught error', error, errorInfo.componentStack);
            }),
            // Callback called when React catches an error in an ErrorBoundary.
            onCaughtError: Sentry.reactErrorHandler(),
            // Callback called when React automatically recovers from errors.
            onRecoverableError: Sentry.reactErrorHandler(),
        }
    )
    root.render(
        <StrictMode>
            <App />
        </StrictMode>,
    )
}

export type RouterContext = {
    auth: AuthContext,
    queryClient: QueryClient;
};
