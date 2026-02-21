import { Center, Stack, Title } from '@mantine/core';
import { ProjectList } from '@starfoundry/components/list/ProjectList';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { useListProjects } from '@starfoundry/components/services/projects/list';
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute(
    '/project-groups_/$projectGroupId/projects',
)({
    component: RouteComponent,
});

function RouteComponent() {
    const { projectGroupId } = Route.useParams();

    const {
        isPending,
        isFetching,
        isError,
        data: projects
    } = useListProjects({
        project_group_id: projectGroupId,
    });

    if (isPending || isFetching) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    let content = () => {
        if (!projects || (projects && projects.length === 0)) {
            return <Center mt={50} data-cy="noData">
                <Stack>
                    <Title order={4}>No projects yet</Title>
                </Stack>
            </Center>
        } else if (projects.length > 0) {
            return <ProjectList
                projects={ projects }
            />
        }
    }

    return <>
        { content() }
    </>
}
