import { Title } from '@mantine/core'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/admin/products')({
  component: RouteComponent,
})

function RouteComponent() {
    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Admin - Products
        </Title>

        <Outlet />
    </>
}
