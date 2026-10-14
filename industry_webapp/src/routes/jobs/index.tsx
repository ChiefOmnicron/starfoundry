import { Accordion } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { LIST_PROJECT_ALL_JOBS, useListProjectAllJobs, type ProjectAllJobFilter } from '@starfoundry/components/services/projects/listAllJobs';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { NumberOfStartableJobs } from '@starfoundry/components/project/NumberOfStartableJobs';
import { ProjectJobAction, type ProjectJobMinimal } from '@starfoundry/components/project/ProjectJobAction';
import { ProjectJobListTableMemo } from '@starfoundry/components/project/ProjectJobListTable';
import { ProjectJobSetting } from './-components/Settings';
import { Route as AssignmentOverviewRoute } from '@/routes/jobs_/$assignmentId.index';
import { TabFilter } from '@starfoundry/components/misc';
import { useCallback, useState } from 'react';
import { useIsFirstRender } from '@mantine/hooks';
import { LIST_FILTERS, useListFilters, type FilterType } from '@starfoundry/components/services/filter/list';
import { useQueryClient } from '@tanstack/react-query';
import type { Uuid } from '@starfoundry/components/services/utils';

const FILTER_TYP: FilterType = 'PROJECT_JOB';

export const Route = createFileRoute('/jobs/')({
    component: RouteComponent,
});

function RouteComponent() {
    const queryClient = useQueryClient();

    const {
        data: filters,
    } = useListFilters({
        typ: FILTER_TYP,
    });

    return <>
        <TabFilter
            filters={filters || []}
            renderComponent={(filter) => <>
                <ProjectJob
                    filter={filter}
                />
            </>}

            createFilter={(close) => <ProjectJobSetting
                    close={close}
                    onChange={() => {
                        queryClient.invalidateQueries({
                            queryKey: [LIST_FILTERS],
                        })
                    }}
                />
            }
            editFilter={(filter, close) => <ProjectJobSetting
                    close={close}
                    filter={filter}
                    onChange={() => {
                        queryClient.invalidateQueries({
                            queryKey: [LIST_FILTERS],
                        })
                    }}
                />
            }
        />
    </>
}

function ProjectJob({
    filter,
}: ProjectJobProps) {
    const navigation = useNavigate();
    const queryClient = useQueryClient();
    const isFirstRender = useIsFirstRender();
    const [selectedRows, setSelectedRows] = useState<ProjectJobMinimal[]>([]);

    const {
        isPending,
        isError,
        isFetching,
        data: projects,
    } = useListProjectAllJobs(filter);

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

    if (!projects) {
        return LoadingAnimation();
    }

    const entries = () => {
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
                            onSelect={onSelect}

                            onJobSplit={onJobSplit}

                            showEdit
                            showStarted
                        />
                    </Accordion.Panel>
                </Accordion.Item>
            </>)
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

type ProjectJobProps = {
    filter: ProjectAllJobFilter,
}
