import { createFileRoute, Outlet } from '@tanstack/react-router'
import { Title } from '@mantine/core'
import { fetchIndustryHubQuery, useFetchIndustryHub } from '@starfoundry/components/services/industry-hub/fetch';
import { useDocumentTitle } from '@mantine/hooks';

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
        data: industryHub,
    } = useFetchIndustryHub(industryHubId);
    useDocumentTitle(`Industry Hub - ${(industryHub || { name: '' }).name}`);


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
