import { Tabs } from '@mantine/core';
import { ProjectJobList, type ProjectJobListProps } from '@starfoundry/components/list/ProjectJobList';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { useListProjectJobs } from '@starfoundry/components/services/projects/listJobs'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/projects_/$projectId/jobs')({
    component: RouteComponent,
})

function RouteComponent() {
    const { projectId } = Route.useParams();

    const {
        isPending,
        isError,
        isFetching,
        data: jobs,
    } = useListProjectJobs(projectId);

    const content = (props: ProjectJobListProps) => {
        if (isPending || isFetching) {
            return <LoadingAnimation />;
        }
        if (isError) {
            return <LoadingError />;
        }

        return <ProjectJobList
            {...props}
        />
    }

    return <>
        <Tabs defaultValue="ready_to_start">
            <Tabs.List>
                <Tabs.Tab value="ready_to_start">
                    Ready to start
                </Tabs.Tab>
                <Tabs.Tab value="building">
                    Active jobs
                </Tabs.Tab>
                <Tabs.Tab value="all">
                    All jobs
                </Tabs.Tab>
            </Tabs.List>

            <Tabs.Panel value="ready_to_start">
                {
                    content({
                        jobs: jobs || [],
                        status: 'READY_TO_START',
                        checkable: true,
                    })
                }
            </Tabs.Panel>
            <Tabs.Panel value="building">
                {
                    content({
                        jobs: jobs || [],
                        status: 'BUILDING',
                        showCost: true,
                        showRemaining: true,
                    })
                }
            </Tabs.Panel>
            <Tabs.Panel value="all">
                {
                    content({
                        jobs: jobs || [],
                        groupByHeader: true,
                        editable: true,
                        showStatus: true,
                        showCost: true,
                    })
                }
            </Tabs.Panel>
        </Tabs>
    </>;
}
