import { Title } from '@mantine/core';
import { useDocumentTitle } from '@mantine/hooks';
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/tags')({
  component: RouteComponent,
})

function RouteComponent() {
    useDocumentTitle('StarFoundry - Tags');

    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Tags
        </Title>

        <Outlet />
    </>
}
