import { createFileRoute } from '@tanstack/react-router'
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { ProjectGroupDefaultsMarket } from './-components/Market';
import { Title } from '@mantine/core';
import { useListProjectGroupDefaultBlacklist } from '@/services/project-group/listDefaultBlacklist';

export const Route = createFileRoute(
    '/project-groups_/$projectGroupId/defaults',
)({
    component: ProjectGroupDefaults,
})

export function ProjectGroupDefaults() {
    const { projectGroupId } = Route.useParams();

    const {
        isError: isErrorBlacklist,
        isPending: isPendingBlacklist,
        data: defaultBlacklist,
    } = useListProjectGroupDefaultBlacklist(projectGroupId);

    if (isPendingBlacklist) {
        return LoadingAnimation();
    }

    if (isErrorBlacklist && !defaultBlacklist) {
        return LoadingError();
    }

    return <>
        <Title order={2}>
            Market
        </Title>

        <ProjectGroupDefaultsMarket
            projectGroupId={projectGroupId}
        />
    </>
}
