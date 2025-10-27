import { Text, Title } from '@mantine/core'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/products')({
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
            Store
        </Title>
        <Text
            size='md'
            pb="md"
        >
            For all your capital and capital fitting needs
        </Text>

        <Outlet />
    </>
}
