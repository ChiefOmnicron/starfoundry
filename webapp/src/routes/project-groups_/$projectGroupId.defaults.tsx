import { createFileRoute } from '@tanstack/react-router'
import { LoadingError } from '@/components/LoadingError';
import { ProjectGroupMarket } from './-components/Market';
import { useFetchProjectGroupDefaults } from '@/services/project-group/fetch_defaults';
import LoadingAnimation from '@/components/LoadingAnimation';

export const Route = createFileRoute(
    '/project-groups_/$projectGroupId/defaults',
)({
    component: ProjectGroupDefaults,
})

export function ProjectGroupDefaults() {
    const { projectGroupId } = Route.useParams();

    const {
        isError,
        isPending,
        data: projectGroupDefaults,
    } = useFetchProjectGroupDefaults(projectGroupId);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError && !projectGroupDefaults) {
        return LoadingError();
    }

    return <>
        <ProjectGroupMarket
            entries={projectGroupDefaults.markets}
        />
    </>
}
