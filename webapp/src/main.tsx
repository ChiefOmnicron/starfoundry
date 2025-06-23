import '@mantine/core/styles.css';
import '@/style.css';

import { StrictMode } from 'react';
import { routeTree } from '@/routeTree.gen';
import { RouterProvider, createRouter } from '@tanstack/react-router';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

import ReactDOM from 'react-dom/client';

const queryClient = new QueryClient();

// Create a new router instance
const router = createRouter({
    routeTree,
    context: {
        queryClient,
    }
});

// Register the router instance for type safety
declare module '@tanstack/react-router' {
    interface Register {
        router: typeof router
    }
}

// Render the app
const rootElement = document.getElementById('root')!

if (!rootElement.innerHTML) {
    const root = ReactDOM.createRoot(rootElement)
    root.render(
        <StrictMode>
            <QueryClientProvider client={queryClient}>
                <RouterProvider router={router} context={{ queryClient }} />
            </QueryClientProvider>
        </StrictMode>,
    )
}

export type RouterContext = {
    queryClient: QueryClient;
};
