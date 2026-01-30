import { Text, Title } from '@mantine/core'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/industry-hubs')({
    component: RouteComponent,
})

function RouteComponent() {
    return <>
        <Title
            data-cy="header"
            order={1}
            style={{
                marginBottom: '15px'
            }}
        >
            Industry Hubs
        </Title>
        <Text
            size='md'
            pb="md"
        >
            List of all industry hubs you have added or are shared with you
        </Text>

        <Outlet />
    </>
}
