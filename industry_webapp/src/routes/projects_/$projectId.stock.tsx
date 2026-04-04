import { createFileRoute } from '@tanstack/react-router'
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { ProjectStockList } from '@starfoundry/components/project/ProjectStockList';
import { useFetchProject } from '@starfoundry/components/services/projects/fetch';

export const Route = createFileRoute('/projects_/$projectId/stock')({
    component: RouteComponent,
})

function RouteComponent() {
    const { projectId } = Route.useParams();

    const {
        isError,
        isPending,
        data: project,
    } = useFetchProject(projectId);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    return <>
        <ProjectStockList
            stock={project.stock}
        />
    </>
}
