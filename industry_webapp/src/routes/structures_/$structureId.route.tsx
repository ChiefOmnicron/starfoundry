import { createFileRoute, Outlet } from '@tanstack/react-router'
import { fetchStructureQuery, useFetchStructure } from '@starfoundry/components/services/structure/fetch';
import { Title } from '@mantine/core'
import { useDocumentTitle } from '@mantine/hooks';

export const Route = createFileRoute('/structures_/$structureId')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
    loader: async ({ context, params }) => {
        const queryClient = context.queryClient;
        queryClient.prefetchQuery(fetchStructureQuery(params.structureId));
    }
})

function RouteComponent() {
    const { structureId } = Route.useParams();

    const {
        data: structure
    } = useFetchStructure(structureId);

    useDocumentTitle(`StarFoundry - ${(structure || { name: 'StarFoundry' }).name}`);

    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Structure '{ structure?.name }'
        </Title>

        <Outlet />
    </>
}
