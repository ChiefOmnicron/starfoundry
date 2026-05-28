import { Text, Title } from '@mantine/core';
import { useDocumentTitle } from '@mantine/hooks';
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/price-calculation')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
  component: RouteComponent,
})

function RouteComponent() {
    useDocumentTitle('StarFoundry - Price Calculation');

    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Price Calculation
        </Title>
        <Text
            size='md'
            pb="md"
        >
            Calculate the build cost
        </Text>

        <Outlet />
    </>
}
