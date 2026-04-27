import { Title } from '@mantine/core'
import { useDocumentTitle } from '@mantine/hooks';
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/jobs')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
})

function RouteComponent() {
    useDocumentTitle('StarFoundry - Jobs');

    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Industry Jobs over all projects
        </Title>

        <Outlet />
    </>
}
