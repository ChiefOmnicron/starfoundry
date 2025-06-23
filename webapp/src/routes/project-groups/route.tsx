import { Text, Title } from '@mantine/core'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/project-groups')({
    component: ProjectGroupIndex,
})

function ProjectGroupIndex() {
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
