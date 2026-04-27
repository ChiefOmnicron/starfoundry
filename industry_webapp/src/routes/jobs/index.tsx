import { Accordion, Title } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { ProjectJobAction, type ProjectJobMinimal } from '@starfoundry/components/project/ProjectJobAction';
import { ProjectJobListTable } from '@starfoundry/components/project/ProjectJobListTable';
import { Route as AssignmentOverview } from '@/routes/jobs_/$assignmentId.index';
import { useIsFirstRender } from '@mantine/hooks';
import { useListProjectJobsRefresh, type ProjectJob } from '@starfoundry/components/services/projects/listJobs';
import { useListProjects, type ProjectListMinimal } from '@starfoundry/components/services/projects/list';
import { useState } from 'react';
import type { Uuid } from '@starfoundry/components/services/utils';

export interface QueryParams {
    deleted?: boolean;
}

export const Route = createFileRoute('/jobs/')({
    component: RouteComponent,
    validateSearch: (params: {
        deleted: boolean,
    }): QueryParams => {
        return {
            deleted: (params.deleted) || undefined
        };
    }
});

function RouteComponent() {
    const navigation = useNavigate();
    const isFirstRender = useIsFirstRender();
    const [selectedRows, setSelectedRows] = useState<ProjectJobMinimal[]>([]);

    const {
        isPending,
        isError,
        isFetching,
        data: projects
    } = useListProjects({
        status: 'IN_PROGRESS',
    });

    if ((isPending || isFetching) && isFirstRender) {
        return LoadingAnimation();
    } else if (isError) {
        return LoadingError();
    }

    const onSelect = (projectId: Uuid, projectJobs: ProjectJob[]) => {
        let tmp = selectedRows.filter(x => x.project_id !== projectId);
        let jobs = projectJobs
            .map(x => {
                return {
                    job_id:     x.id,
                    project_id: x.project_id,
                }
            })
        setSelectedRows([...tmp, ...jobs]);
    }

    const entries = () => {
        if (!projects) {
            return <></>;
        }

        return projects
            .map(x => <>
                    <ProjectJobListWrapper
                        key={x.id}
                        project={x}
                        onSelect={(y: ProjectJob[]) => {
                            onSelect(x.id, y);
                        }}
                    />
                </>
            )
    }

    return <>
        <ProjectJobAction
            selected={selectedRows}

            onCreated={(id: Uuid) => navigation({
                to: AssignmentOverview.to,
                params: {
                    assignmentId: id
                },
            })}
        />

        <Accordion
            defaultValue={(projects || []).map(x => x.id)}
            variant="contained"
            multiple
        >
            {entries()}
        </Accordion>
    </>
}

// Wrapper so that every project can independently can load the jobs
function ProjectJobListWrapper({
    project,
    onSelect,
}: ProjectJobListWrapperProps) {
    const isFirstRender = useIsFirstRender();

    const {
        isPending,
        isFetching,
        isError,
        data: jobs,
    } = useListProjectJobsRefresh(
        project.id,
        {
            startable: true,
        }
    );

    if ((isPending || isFetching) && isFirstRender) {
        return <>
            <Title order={2}>{project.name}</Title>
            {LoadingAnimation()}
        </>
    }

    if (isError) {
        return LoadingError();
    }

    if (jobs) {
        if (jobs.length === 0) {
            return <></>;
        }

        return <>
            <Accordion.Item
                key={project.id}
                value={project.id}
            >
                <Accordion.Control>
                    {project.name}
                </Accordion.Control>
                <Accordion.Panel>
                    <ProjectJobListTable
                        projectId={project.id}
                        jobs={jobs.flatMap(x => x.entries)}
                        checkable={true}
                        showStarted={true}
                        onSelect={onSelect}
                    />
                </Accordion.Panel>
            </Accordion.Item>
        </>
    }
}

type ProjectJobListWrapperProps = {
    project: ProjectListMinimal,

    onSelect?: (selected: ProjectJob[]) => void;
}
