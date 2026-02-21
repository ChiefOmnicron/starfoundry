import { createFileRoute, Outlet } from '@tanstack/react-router'
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { Title } from '@mantine/core'
import { fetchStructureQuery, useFetchStructure } from '@starfoundry/components/services/structure/fetch';

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
        isPending,
        isError,
        data: structure
    } = useFetchStructure(structureId);

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
            Structure '{ structure?.name }'
        </Title>

        <Outlet />
    </>
}
