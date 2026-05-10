import { createFileRoute } from '@tanstack/react-router'
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { ProjectStockList } from '@starfoundry/components/project/ProjectStockList';
import { useFetchProject } from '@starfoundry/components/services/projects/fetch';
import { FittingModal } from './-components/FittingModal';
import { useDisclosure } from '@mantine/hooks';
import { Button, Group, Stack } from '@mantine/core';

export const Route = createFileRoute('/projects_/$projectId/stock')({
    component: RouteComponent,
})

function RouteComponent() {
    const { projectId } = Route.useParams();

    const [fittingsModalOpened, {
        open: openFittingsModal,
        close: closeFittingsModal,
    }] = useDisclosure(false);

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
        <FittingModal
            entries={project.stock}
            fitName={`${project.name}-stock`}

            opened={fittingsModalOpened}
            close={closeFittingsModal}
        />

        <Stack>
            <Group
                justify='flex-end'
            >
                <Button
                    onClick={openFittingsModal}
                >
                    Save as fit
                </Button>
            </Group>

            <ProjectStockList
                stock={project.stock}
            />
        </Stack>
    </>
}
