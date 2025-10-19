import { createFileRoute } from '@tanstack/react-router'
import { LoadingError } from '@/components/LoadingError';
import { ProjectGroupMarket } from './-components/Market';
import { useListProjectGroupDefaultBlacklist } from '@/services/project-group/listDefaultBlacklist';
import { useListProjectGroupDefaultMarkets } from '@/services/project-group/listDefaultMarket';
import LoadingAnimation from '@/components/LoadingAnimation';

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

    const {
        isError: isErrorMarket,
        isPending: isPendingMarket,
        data: defaultMarket,
    } = useListProjectGroupDefaultMarkets(projectGroupId);

    if (isPendingBlacklist) {
        return LoadingAnimation();
    }
    if (isPendingMarket) {
        return LoadingAnimation();
    }

    if (isErrorBlacklist && !defaultBlacklist) {
        return LoadingError();
    }
    if (isErrorMarket && !defaultMarket) {
        return LoadingError();
    }

    return <>
        <ProjectGroupMarket
            entries={defaultMarket}
        />
    </>
}
