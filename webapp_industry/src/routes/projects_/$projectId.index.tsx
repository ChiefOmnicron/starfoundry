import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/projects_/$projectId/')({
    component: RouteComponent,
});

export function RouteComponent() {
    return <>
    </>
}
