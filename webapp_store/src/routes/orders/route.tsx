import { Text, Title } from '@mantine/core'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/orders')({
    beforeLoad: async ({ context }) => {
        if (!await context.auth.isAuthenticated()) {
            throw context.auth.login();
        }
    },
    component: StoreHeaderComponent,
})

function StoreHeaderComponent() {
    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Orders
        </Title>
        <Text
            size='md'
            pb="md"
        >
            All your current and past orders
        </Text>

        <Outlet />
    </>
}
