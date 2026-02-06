import { createFileRoute, Outlet } from '@tanstack/react-router'
import { Text, Title } from '@mantine/core'

export const Route = createFileRoute('/projects/assistant')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
})

function RouteComponent() {

    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Create Project Assistant
        </Title>

        <Text>
            Assistant for creating new projects.
        </Text>

        <Outlet />
    </>
}
