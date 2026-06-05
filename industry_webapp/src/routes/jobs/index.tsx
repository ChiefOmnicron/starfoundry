import { Accordion, Button } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { LIST_PROJECT_ALL_JOBS, useListProjectAllJobs } from '@starfoundry/components/services/projects/listAllJobs';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { NumberOfStartableJobs } from '@starfoundry/components/project/NumberOfStartableJobs';
import { ProjectJobAction, type ProjectJobMinimal } from '@starfoundry/components/project/ProjectJobAction';
import { ProjectJobListTableMemo } from '@starfoundry/components/project/ProjectJobListTable';
import { Route as AssignmentOverviewRoute } from '@/routes/jobs_/$assignmentId.index';
import { useIsFirstRender } from '@mantine/hooks';
import { useQueryClient } from '@tanstack/react-query';
import { useCallback, useState } from 'react';
import type { Uuid } from '@starfoundry/components/services/utils';


export const Route = createFileRoute('/jobs/')({
    component: RouteComponent,
});

function RouteComponent() {
    const navigation = useNavigate();
    const queryClient = useQueryClient();
    const isFirstRender = useIsFirstRender();
    const [selectedRows, setSelectedRows] = useState<ProjectJobMinimal[]>([]);

    const {
        isPending,
        isError,
        isFetching,
        data: projects,
    } = useListProjectAllJobs();

    const onSelect = useCallback((projectId: Uuid, projectJobs: ProjectJobMinimal[]) => {
        let tmp = selectedRows.filter(x => x.project_id !== projectId);
        setSelectedRows([...tmp, ...projectJobs]);
    }, [selectedRows]);

    const onJobSplit = useCallback(() => {
        queryClient.invalidateQueries({ queryKey: [LIST_PROJECT_ALL_JOBS] })
    }, []);

    if ((isPending || isFetching) && isFirstRender) {
        return LoadingAnimation();
    } else if (isError) {
        return LoadingError();
    }

    const entries = () => {
        if (!projects) {
            return <></>;
        }

        return projects
            .map(x => <>
                    <Accordion.Item
                        key={x.project_id}
                        value={x.project_id}
                    >
                        <Accordion.Control>
                            {x.header}
                        </Accordion.Control>
                        <Accordion.Panel>
                            <ProjectJobListTableMemo
                                projectId={x.project_id}
                                key={x.header}
                                jobs={x.entries}

                                checkable
                                onSelect={() => onSelect}

                                onJobSplit={onJobSplit}

                                showEdit
                                showStarted
                            />

                            <Button onClick={() => onSelect(x.project_id, selectedRows)}>
                                Add to selected
                            </Button>
                            <Button onClick={() => onSelect(x.project_id, [])}>
                                Reset
                            </Button>
                        </Accordion.Panel>
                    </Accordion.Item>
                </>
            )
    }

    return <>
        <ProjectJobAction
            selected={selectedRows}

            onCreated={(id: Uuid) => navigation({
                to: AssignmentOverviewRoute.to,
                params: {
                    assignmentId: id
                },
            })}
        />

        <NumberOfStartableJobs
            jobs={((projects || []).flatMap(x => x.entries))}
        />

        <Accordion
            defaultValue={(projects || []).map(x => x.project_id)}
            variant="contained"
            multiple
        >
            {entries()}
        </Accordion>
    </>
}
