import { Center, Stack, Title } from '@mantine/core';
import { createFileRoute } from '@tanstack/react-router'
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { useListProjects } from '@/services/projects/list';
import { ProjectList } from '@/components/ProjectCard';

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

