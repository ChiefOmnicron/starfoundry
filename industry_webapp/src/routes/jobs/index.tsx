import { createFileRoute } from '@tanstack/react-router';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { ProjectJobList } from '@starfoundry/components/project/ProjectJobList';
import { Stack, Title } from '@mantine/core';
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
    const {
        isPending,
        isError,
        isFetching,
        data: projects
    } = useListProjects({
        status: 'IN_PROGRESS',
    });

    if (isPending || isFetching) {
        return LoadingAnimation();
    } else if (isError) {
        return LoadingError();
    }

    const entries = () => {
        return projects
            .map(x => <>
                    <ProjectJobListWrapper
                        project={x}
                    />
                </>
            )
    }

    return <>
        <Stack>
            {entries()}
        </Stack>
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
        data: jobs,
    } = useListProjectJobsRefresh(
        project.id,
        {
            startable: true,
        }
    );

    if ((isPending || isFetching) && isFirstRender) {
        return <LoadingAnimation />;
    }

    if (jobs) {
        console.log(jobs)
        if (jobs.length === 0) {
            return <></>;
        }

        return <>
            <Title order={2}>{project.name}</Title>

            <ProjectJobList
                projectId={project.id}
                jobs={jobs}
                status='READY_TO_START'
                checkable={true}
                showStarted={true}
            />
        </>
    }
}

type ProjectJobListWrapperProps = {
    project: ProjectListMinimal,
}
