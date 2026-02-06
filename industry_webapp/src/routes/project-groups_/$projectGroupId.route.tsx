import { createFileRoute, Outlet } from '@tanstack/react-router'
import { fetchProjectGroupMemberSelfQuery } from '@/services/project-group/fetchMemberSelf';
import { fetchProjectGroupQuery, useFetchProjectGroup } from '@/services/project-group/fetch';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { Title } from '@mantine/core'

export const Route = createFileRoute('/project-groups_/$projectGroupId')({
    beforeLoad: async ({ context }) => {
        if (!await context.auth.isAuthenticated()) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
    loader: async ({ context, params }) => {
        const queryClient = context.queryClient;
        queryClient.prefetchQuery(fetchProjectGroupMemberSelfQuery(params.projectGroupId));
        queryClient.prefetchQuery(fetchProjectGroupQuery(params.projectGroupId));
    }
})

function RouteComponent() {
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
        return LoadingError();
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
