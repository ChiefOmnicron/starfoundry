import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/projects_/$projectId/assistant')({
    component: RouteComponent,
})

function RouteComponent() {
    return <Outlet />;
}
