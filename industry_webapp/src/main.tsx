import '@mantine/core/styles.css';
import '@/style.css';

import { StrictMode } from 'react';
import { routeTree } from '@/routeTree.gen';
import { RouterProvider, createRouter } from '@tanstack/react-router';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

import ReactDOM from 'react-dom/client';
import { Route as LoginRoute } from '@/routes/auth/login';
import { AuthProvider, useAuth, type AuthContext } from '@starfoundry/components/auth/auth';

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

function App() {
    return (
        <AuthProvider
            loginRoute={LoginRoute.to}
        >
            <Inner />
        </AuthProvider>
    );
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
