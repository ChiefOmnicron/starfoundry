import LoadingAnimation from '@/components/LoadingAnimation';
import { canWriteProjectGroupQuery } from '@/services/project-group/can_write_group';
import { fetchProjectGroupQuery, useFetchProjectGroup } from '@/services/project-group/fetch';
import { Alert, Title } from '@mantine/core'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/project-groups_/$projectGroupId')({
    component: ProjectGroupHeader,
    loader: async ({ context, params }) => {
        const queryClient = context.queryClient;
        queryClient.prefetchQuery(canWriteProjectGroupQuery(params.projectGroupId));
        queryClient.prefetchQuery(fetchProjectGroupQuery(params.projectGroupId));
    }
})

export function ProjectGroupHeader() {
    const { projectGroupId } = Route.useParams();

    const {
        isPending,
        isError,
        data: projectGroups
    } = useFetchProjectGroup(projectGroupId);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return <Alert
            variant='light'
            color='red'
            title='Unknown loading error'
            data-cy="error"
        >
            There was an unknown error while loading the data. Please try again later.
        </Alert>
    }

    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Project Group '{ projectGroups?.name }'
        </Title>

        <Outlet />
    </>
}
