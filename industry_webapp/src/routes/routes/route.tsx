import { Title } from '@mantine/core';
import { useDocumentTitle } from '@mantine/hooks';
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/routes')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
})

function RouteComponent() {
    useDocumentTitle('StarFoundry - Routes');

    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Routes
        </Title>

        <Outlet />
    </>
}
