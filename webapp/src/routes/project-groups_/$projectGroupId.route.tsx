import LoadingAnimation from '@/components/LoadingAnimation';
import { fetchProjectGroupQuery, useFetchProjectGroup } from '@/services/project-group/fetch';
import { Alert, Tabs, Title } from '@mantine/core'
import { createFileRoute, Outlet, useNavigate } from '@tanstack/react-router'

export const Route = createFileRoute('/project-groups_/$projectGroupId')({
    component: ProjectGroupId,
    loader: async ({ context, params }) => {
        const queryClient = context.queryClient;
        queryClient.prefetchQuery(fetchProjectGroupQuery(params.projectGroupId));
    }
})

function ProjectGroupId() {
    const navigation = useNavigate({ from: Route.fullPath });
    const { projectGroupId } = Route.useParams();

    const {
        isPending,
        isError,
        data: projectGroups
    } = useFetchProjectGroup(projectGroupId);

    let openTab = 'overview';
    if (Route.children) {
        openTab = (Route.children as any)[0]._path;
    }

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return <Alert
            variant='light'
            color='red'
            title='Unknown loading error'
            data-cy="error"
        >
            There was an unknown error while loading the data. Please try again later.
        </Alert>
    }

    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Project Group '{ projectGroups?.name }'
        </Title>

        <Tabs
            value={ openTab }
            onChange={(value) => navigation(
                {
                    to: `/project-groups/${projectGroupId}/${value}`
                }
            )}
        >
            <Tabs.List>
                <Tabs.Tab value="overview">Overview</Tabs.Tab>
                <Tabs.Tab value="projects">Projects</Tabs.Tab>
                <Tabs.Tab value="structures">Structures</Tabs.Tab>
                <Tabs.Tab value="members">Members</Tabs.Tab>
                <Tabs.Tab value="defaults">Defaults</Tabs.Tab>
            </Tabs.List>
        </Tabs>


        <Outlet />
    </>
}
