import { createFileRoute, Outlet, redirect } from '@tanstack/react-router'
import { Route as StoreRoute } from '@/routes/products/index';

export const Route = createFileRoute('/admin')({
    beforeLoad: async ({ context }) => {
        if (!await context.auth.isAuthenticated()) {
            throw context.auth.login();
        }

        if (!await context.auth.isAdmin()) {
            throw redirect({
                to: StoreRoute.to,
            });
        }
    },
    component: AdminComponent,
})

function AdminComponent() {
    return <>
        <Outlet />
    </>
}
