import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/projects_/$projectId/overview')({
    component: RouteComponent,
});

function RouteComponent() {
    return <>
    </>
}
