import { createFileRoute, Outlet } from '@tanstack/react-router'
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { Title } from '@mantine/core'
import { fetchIndustryHubQuery, useFetchIndustryHub } from '@/services/industry-hub/fetch';

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
