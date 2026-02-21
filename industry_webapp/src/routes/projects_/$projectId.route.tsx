import { Title } from '@mantine/core'
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { fetchProjectQuery, useFetchProject } from '@starfoundry/components/services/projects/fetch';
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
