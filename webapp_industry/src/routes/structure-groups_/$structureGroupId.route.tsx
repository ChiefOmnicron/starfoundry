import { createFileRoute, Outlet } from '@tanstack/react-router'
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { Title } from '@mantine/core'
import { fetchStructureGroupQuery, useFetchStructureGroup } from '@/services/structure-group/fetch';

export const Route = createFileRoute('/structure-groups_/$structureGroupId')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: StructureGroupHeader,
    loader: async ({ context, params }) => {
        const queryClient = context.queryClient;
        queryClient.prefetchQuery(fetchStructureGroupQuery(params.structureGroupId));
    }
})

export function StructureGroupHeader() {
    const { structureGroupId } = Route.useParams();

    const {
        isPending,
        isError,
        data: structureGroup,
    } = useFetchStructureGroup(structureGroupId);

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
            Structure Group '{ structureGroup?.name }'
        </Title>

        <Outlet />
    </>
}
