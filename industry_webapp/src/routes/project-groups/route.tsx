import { Text, Title } from '@mantine/core'
import { useDocumentTitle } from '@mantine/hooks';
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/project-groups')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
})

function RouteComponent() {
    useDocumentTitle('StarFoundry - Project Groups');

    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Project Groups
        </Title>
        <Text
            size='md'
            pb="md"
        >
            Project groups allow you to set defaults and collaborate with other capsuleers
        </Text>

        <Outlet />
    </>
}
