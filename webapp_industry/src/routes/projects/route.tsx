import { Text, Title } from '@mantine/core'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/projects')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: ProjectGroupIndex,
})

function ProjectGroupIndex() {
    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Projects
        </Title>
        <Text
            size='md'
            pb="md"
        >
            Every project has the goal of producing one or more end products.
        </Text>

        <Outlet />
    </>
}
