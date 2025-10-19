import { Title } from '@mantine/core'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/structures')({
    component: ProjectGroupIndex,
})

function ProjectGroupIndex() {
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

        <Outlet />
    </>
}
