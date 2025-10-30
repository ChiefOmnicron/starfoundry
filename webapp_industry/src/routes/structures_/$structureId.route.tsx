import LoadingAnimation from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { fetchStructureQuery, useFetchStructure } from '@/services/structure/fetch';
import { Title } from '@mantine/core'
import { createFileRoute, Outlet } from '@tanstack/react-router'

export const Route = createFileRoute('/structures_/$structureId')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: StructureHeader,
    loader: async ({ context, params }) => {
        const queryClient = context.queryClient;
        queryClient.prefetchQuery(fetchStructureQuery(params.structureId));
    }
})

export function StructureHeader() {
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
