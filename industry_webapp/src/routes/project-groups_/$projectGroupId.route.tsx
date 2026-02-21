import { createFileRoute, Outlet } from '@tanstack/react-router'
import { Title } from '@mantine/core'
import { fetchProjectGroupQuery, useFetchProjectGroup } from '@starfoundry/components/services/project-group/fetch';
import { fetchProjectGroupMemberSelfQuery } from '@starfoundry/components/services/project-group/fetchMemberSelf';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';

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
