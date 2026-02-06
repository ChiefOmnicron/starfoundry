import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { fetchProjectQuery, useFetchProject } from '@/services/projects/fetch';
import { Title } from '@mantine/core'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/projects_/$projectId')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
    loader: async ({ context, params }) => {
        const queryClient = context.queryClient;
        queryClient.prefetchQuery(fetchProjectQuery(params.projectId));
    }
})

function RouteComponent() {
    const { projectId } = Route.useParams();

    const {
        isPending,
        isError,
        data: project,
    } = useFetchProject(projectId)

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Project '{ project.name }'
        </Title>

        <Outlet />
    </>
}
