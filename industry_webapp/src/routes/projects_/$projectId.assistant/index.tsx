import { createFileRoute } from '@tanstack/react-router';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { Solution } from '@/routes/projects_/$projectId.assistant/-components/Solution';
import { useFetchProject } from '@starfoundry/components/services/projects/fetch';
import type { Uuid } from '@starfoundry/components/services/utils';

export const Route = createFileRoute('/projects_/$projectId/assistant/')({
    component: RouteComponent,
});

function RouteComponent() {
    const { projectId } = Route.useParams();

    const {
        isPending,
        isError,
        data: project,
    } = useFetchProject(projectId);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    return <Solution
        project={project}
    />
}

export type ProjectAssistantGeneralInformation = {
    name: string,
    orderer: string,
    sellPrice: number,

    projectGroupId: Uuid,
}
