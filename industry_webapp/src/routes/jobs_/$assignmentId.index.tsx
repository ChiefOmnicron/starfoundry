import { createFileRoute } from '@tanstack/react-router';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { Accordion, Button, Title } from '@mantine/core';
import { useIsFirstRender } from '@mantine/hooks';
import { LIST_JOB_ASSIGNMENT, useListJobAssignmentsRefresh, type ProjectJobAssignment, type ProjectJobAssignmentGroup } from '@starfoundry/components/services/projects/listJobAssignments';
import { createColumnHelper, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { CopyText } from '@starfoundry/components/misc/CopyText';
import { EveIcon } from '@starfoundry/components/misc/EveIcon';
import { TableWrapper } from '@starfoundry/components/wrapper/Table';
import type { Uuid } from '@starfoundry/components/services/utils';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { updateJobOrder } from '@starfoundry/components/services/projects/updateJobOrder';

export interface QueryParams {
    deleted?: boolean;
}

export const Route = createFileRoute('/jobs_/$assignmentId/')({
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
    const { assignmentId } = Route.useParams();
    const isFirstRender = useIsFirstRender();

    const {
        isPending,
        isError,
        isFetching,
        data: jobs,
    } = useListJobAssignmentsRefresh(assignmentId);

    if ((isPending || isFetching) && isFirstRender) {
        return LoadingAnimation();
    } else if (isError) {
        return LoadingError();
    }

    const entries = () => {
        if (!jobs) {
            return <></>;
        }

        return jobs
            .map(x => <>
                    <JobAssignmentWrapper
                        assignmentId={assignmentId}
                        project={x.header}
                        jobs={x.entries}
                    />
                </>
            )
    }

    return <>
        <Title order={1}>Jobs ready to be started</Title>

        <Accordion
            defaultValue={(jobs || []).map(x => x.header)}
            variant="contained"
            multiple
        >
            {entries()}
        </Accordion>
    </>
}

// Wrapper so that every project can independently can load the jobs
function JobAssignmentWrapper({
    assignmentId,
    project,
    jobs,
}: JobAssignmentWrapperProps) {
    const queryClient = useQueryClient();

    const updateEntryMutation = useMutation({
        mutationFn: async (jobId: Uuid) => {
            return await updateJobOrder(assignmentId, jobId);
        },
        onMutate: async (newEntry, context) => {
            const updated: ProjectJobAssignmentGroup[] = context
                .client
                .getQueryData([LIST_JOB_ASSIGNMENT]) || [];

            updated
                .map(x => {
                    if (x.header === project) {
                        x
                            .entries
                            .map(y => {
                                if (y.id === newEntry) {
                                    y.started = true;
                                }

                                return y;
                            });
                    }

                    return x;
                });
            context.client.setQueryData([LIST_JOB_ASSIGNMENT], () => [...updated])
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [LIST_JOB_ASSIGNMENT ]});
        },
    });

    const columnHelper = createColumnHelper<ProjectJobAssignment>();
    const columns = [
        columnHelper.display({
            id: 'icon',
            cell: ({ row }) => <EveIcon
                id={row.original.item.type_id}
            />,
            size: 1,
            maxSize: 1,
        }),
        columnHelper.display({
            id: 'name',
            cell: ({ row }) => <CopyText
                value={row.original.item.name}
                disabled={row.original.started}
            />,
            header: () => 'Name',
            size: 20,
        }),
        columnHelper.display({
            id: 'runs',
            cell: ({ row }) => <CopyText
                value={row.original.runs}
                disabled={row.original.started}
            />,
            header: () => 'Runs',
            size: 3,
            maxSize: 3,
        }),
        columnHelper.display({
            id: 'structure',
            cell: ({ row }) => <CopyText
                value={row.original.structure_name}
                disabled={row.original.started}
            />,
            header: () => 'Structure',
            size: 10,
        }),
        columnHelper.display({
            id: 'action',
            cell: ({ row }) => <Button
                    onClick={() => {
                        updateEntryMutation.mutate(row.original.id)
                    }}
                    disabled={row.original.started}
                >
                    Started
                </Button>,
            size: 1,
            maxSize: 1,
        }),
    ];

    const table = useReactTable<ProjectJobAssignment>({
        columns: columns,
        data: jobs,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
        getRowId: row => row.id,
    });

    return <>
        <Accordion.Item
            key={project}
            value={project}
        >
            <Accordion.Control>
                {project}
            </Accordion.Control>
            <Accordion.Panel>
                <TableWrapper
                    table={table}
                    scrollable={true}
                />
            </Accordion.Panel>
        </Accordion.Item>
    </>
}

type JobAssignmentWrapperProps = {
    assignmentId:   Uuid,
    project:        string,
    jobs:           ProjectJobAssignment[],
}
