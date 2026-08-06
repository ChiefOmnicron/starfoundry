import { Title } from '@mantine/core'
import { useDocumentTitle } from '@mantine/hooks';
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/bulk_buy')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
})

function RouteComponent() {
    useDocumentTitle('StarFoundry - Bulk Buy');

    return <>
        <Title
            data-cy="header"
            order={1}
            style={{
                marginBottom: '15px'
            }}
        >
            Bulk Buy
        </Title>

        <Outlet />
    </>
}
