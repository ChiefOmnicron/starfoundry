import { createFileRoute, Outlet } from '@tanstack/react-router'
import { Title } from '@mantine/core'
import { fetchIndustryHubQuery, useFetchIndustryHub } from '@starfoundry/components/services/industry-hub/fetch';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';

export const Route = createFileRoute('/industry-hubs_/$industryHubId')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
    loader: async ({ context, params }) => {
        const queryClient = context.queryClient;
        queryClient.prefetchQuery(fetchIndustryHubQuery(params.industryHubId));
    }
})

function RouteComponent() {
    const { industryHubId } = Route.useParams();

    const {
        isPending,
        isError,
        data: industryHub,
    } = useFetchIndustryHub(industryHubId);

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
            Industry Hub '{ industryHub?.name }'
        </Title>

        <Outlet />
    </>
}
