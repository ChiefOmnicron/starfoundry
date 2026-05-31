import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { ProjectJobList, type ProjectJobListProps } from '@starfoundry/components/project/ProjectJobList';
import { Route as AssignmentOverview } from '@/routes/jobs_/$assignmentId.index';
import { Tabs } from '@mantine/core';
import { useIsFirstRender } from '@mantine/hooks';
import { LIST_PROJECT_JOBS, useListProjectJobsRefresh } from '@starfoundry/components/services/projects/listJobs'
import type { Uuid } from '@starfoundry/components/services/utils';
import { useMutation } from '@tanstack/react-query';
import { deleteJob } from '@starfoundry/components/services/projects/deleteJob';

export const Route = createFileRoute('/projects_/$projectId/jobs')({
    component: RouteComponent,
})

function RouteComponent() {
    const navigation = useNavigate();
    const { projectId } = Route.useParams();
    const isFirstRender = useIsFirstRender();

    const {
        isPending,
        isError,
        isFetching,
        data: jobs,
    } = useListProjectJobsRefresh(projectId, {});

    const deleteJobMutation = useMutation({
        mutationFn: (id: Uuid) => deleteJob(projectId, id),
        onSuccess: (_data, _variables, _result, context) => {
            context.client.invalidateQueries({ queryKey: [LIST_PROJECT_JOBS] })
        }
    });

    const content = (props: ProjectJobListProps) => {
        if ((isPending || isFetching) && isFirstRender) {
            return <LoadingAnimation />;
        }
        if (isError) {
            return <LoadingError />;
        }

        return <ProjectJobList
            {...props}
        />
    }

    const onCreated = (id: Uuid) => {
        navigation({
            to: AssignmentOverview.to,
            params: {
                assignmentId: id
            },
        })
    }

    const onDelete = (id: Uuid) => {
        deleteJobMutation.mutate(id);
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
                        projectId: projectId,
                        jobs: jobs || [],
                        status: 'READY_TO_START',
                        checkable: true,
                        //showQuickFix: true,
                        showStarted: true,
                        onCreated,
                        onDelete,
                    })
                }
            </Tabs.Panel>
            <Tabs.Panel value="building">
                {
                    content({
                        projectId: projectId,
                        jobs: jobs || [],
                        status: 'BUILDING',
                        showCost: true,
                        showRemaining: true,
                        onCreated,
                        onDelete,
                    })
                }
            </Tabs.Panel>
            <Tabs.Panel value="all">
                {
                    content({
                        projectId: projectId,
                        jobs: jobs || [],
                        groupByHeader: true,
                        editable: true,
                        showStatus: true,
                        showCost: true,
                        onCreated,
                        onDelete,
                    })
                }
            </Tabs.Panel>
        </Tabs>
    </>;
}
