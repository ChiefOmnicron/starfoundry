import '@mantine/core/styles.css';
import '@/style.css';

import { StrictMode } from 'react';
import { routeTree } from '@/routeTree.gen';
import { RouterProvider, createRouter } from '@tanstack/react-router';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { AuthProvider, useAuth, type AuthContext } from './auth';

import ReactDOM from 'react-dom/client';

// Create a new router instance
const router = createRouter({
    routeTree,
    context: {
        auth: undefined!,
        queryClient: undefined!,
    },
});

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
const rootElement = document.getElementById('root')!

if (!rootElement.innerHTML) {
    const root = ReactDOM.createRoot(rootElement)
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
