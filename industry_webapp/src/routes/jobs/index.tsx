import { createFileRoute } from '@tanstack/react-router';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { ProjectJobList } from '@starfoundry/components/project/ProjectJobList';
import { Accordion, Title } from '@mantine/core';
import { useIsFirstRender } from '@mantine/hooks';
import { useListProjectJobsRefresh } from '@starfoundry/components/services/projects/listJobs';
import { useListProjects, type ProjectListMinimal } from '@starfoundry/components/services/projects/list';

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
    const isFirstRender = useIsFirstRender();

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

    const entries = () => {
        if (!projects) {
            return <></>;
        }

        return projects
            .map(x => <>
                    <ProjectJobListWrapper
                        key={x.id}
                        project={x}
                    />
                </>
            )
    }

    return <>
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
                    <ProjectJobList
                        projectId={project.id}
                        jobs={jobs}
                        status='READY_TO_START'
                        checkable={true}
                        showStarted={true}
                    />
                </Accordion.Panel>
            </Accordion.Item>
        </>
    }
}

type ProjectJobListWrapperProps = {
    project: ProjectListMinimal,
}
