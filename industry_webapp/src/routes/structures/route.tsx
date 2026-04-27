import { Text, Title } from '@mantine/core'
import { useDocumentTitle } from '@mantine/hooks';
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/structures')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
})

function RouteComponent() {
    useDocumentTitle('StarFoundry - Structures');

    return <>
        <Title
            data-cy="header"
            order={1}
            style={{
                marginBottom: '15px'
            }}
        >
            Structures
        </Title>
        <Text
            size='md'
            pb="md"
        >
            List of all structures you have added or are shared with you
        </Text>

        <Outlet />
    </>
}
