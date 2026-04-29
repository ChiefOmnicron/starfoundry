import { Accordion, Button, Checkbox, Table } from '@mantine/core';
import { CopyText } from '@starfoundry/components/misc/CopyText';
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable, type RowSelectionState } from '@tanstack/react-table';
import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { EveIcon } from '@starfoundry/components/misc/EveIcon';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { useEffect, useState } from 'react';
import { ProjectJobAction, type ProjectJobMinimal } from '@starfoundry/components/project/ProjectJobAction';
import { Route as AssignmentOverviewRoute } from '@/routes/jobs_/$assignmentId.index';
import { useIsFirstRender } from '@mantine/hooks';
import { useListProjectAllJobs } from '@starfoundry/components/services/projects/listAllJobs';
import type { ProjectJob } from '@starfoundry/components/services/projects/listJobs';
import type { Uuid } from '@starfoundry/components/services/utils';


export const Route = createFileRoute('/jobs/')({
    component: RouteComponent,
});

function RouteComponent() {
    const navigation = useNavigate();
    const isFirstRender = useIsFirstRender();
    const [selectedRows, setSelectedRows] = useState<ProjectJobMinimal[]>([]);

    const {
        isPending,
        isError,
        isFetching,
        data: projects,
    } = useListProjectAllJobs();

    if ((isPending || isFetching) && isFirstRender) {
        return LoadingAnimation();
    } else if (isError) {
        return LoadingError();
    }

    const onSelect = (projectId: Uuid, projectJobs: ProjectJobMinimal[]) => {
        let tmp = selectedRows.filter(x => x.project_id !== projectId);
        setSelectedRows([...tmp, ...projectJobs]);
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
                            <ProjectJobListWrapper
                                key={x.header}
                                jobs={x.entries}
                                onSelect={(y: ProjectJobMinimal[]) => {
                                    onSelect(x.project_id, y);
                                }}
                            />
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

        <Accordion
            defaultValue={(projects || []).map(x => x.project_id)}
            variant="contained"
            multiple
        >
            {entries()}
        </Accordion>
    </>
}

// Wrapper so that every project can independently can load the jobs
function ProjectJobListWrapper({
    jobs,
    onSelect = () => {},
}: ProjectJobListWrapperProps) {
    const [rowSelection, setRowSelection] = useState<RowSelectionState>({});
    const [started, setStarted] = useState<Uuid[]>([]);

    const columnHelper = createColumnHelper<ProjectJob>();
    const columns = [
        columnHelper.display({
            id: 'check',
            header: ({table}) => <Checkbox
                checked={table.getIsAllRowsSelected()}
                indeterminate={table.getIsSomeRowsSelected()}
                onChange={table.getToggleAllRowsSelectedHandler()}
            />,
            cell: ({row}) => <Checkbox
                checked={row.getIsSelected()}
                onChange={row.getToggleSelectedHandler()}
            />,
            size: 1,
            maxSize: 1,
        }),
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
                disabled={started.indexOf(row.original.id) > -1}
            />,
            header: () => 'Name',
            size: 20,
        }),
        columnHelper.display({
            id: 'runs',
            cell: ({ row }) => <CopyText
                value={row.original.runs}
                disabled={started.indexOf(row.original.id) > -1}
            />,
            header: () => 'Runs',
            size: 3,
            maxSize: 3,
        }),
        columnHelper.display({
            id: 'structure',
            cell: ({ row }) => <CopyText
                value={row.original.structure.name}
                disabled={started.indexOf(row.original.id) > -1}
            />,
            header: () => 'Structure',
            size: 10,
        }),
        columnHelper.display({
            id: 'action',
            cell: ({ row }) => {
                return <Button
                    onClick={() => {
                        setStarted([...started, row.original.id]);
                    }}
                    disabled={started.indexOf(row.original.id) > -1}
                >
                    Started
                </Button>;
            },
            meta: {
                align: 'right',
            },
            size: 1,
            maxSize: 1,
        }),
    ];
    
    const table = useReactTable<ProjectJob>({
        columns: columns,
        data: jobs,
        autoResetPageIndex: false,
        onRowSelectionChange: (selected) => setRowSelection(selected),
        getCoreRowModel: getCoreRowModel(),
        getRowId: row => row.id,
        state: {
            rowSelection,
        },
    });

    // must stay, otherwise the selection change is not properly triggered
    useEffect(() => {
        onSelect(
            table
                .getSelectedRowModel()
                .rows
                .map(x => x.original)
                .map(x => {
                    return {
                        project_id: x.project_id,
                        job_id: x.id
                    }
                })
            );
    }, [rowSelection]);

    return <>
        <Table.ScrollContainer minWidth={100} maxHeight={500}>
            <Table stickyHeader striped data-cy="data">
                <Table.Thead>
                    {
                        table
                            .getHeaderGroups()
                            .map(headerGroup => (
                                <Table.Tr key={headerGroup.id}>
                                    {
                                        headerGroup
                                            .headers
                                            .map(header => (
                                                <Table.Th
                                                    key={header.id}
                                                    style={{
                                                        width: `${header.getSize()}%`
                                                    }}
                                                >
                                                    {
                                                        flexRender(
                                                            header.column.columnDef.header,
                                                            header.getContext()
                                                        )
                                                    }
                                                </Table.Th>
                                            ))
                                    }
                                </Table.Tr>
                            ))
                    }
                </Table.Thead>

                <Table.Tbody>
                    {
                        table
                            .getRowModel()
                            .rows
                            .map(row => (
                                <Table.Tr key={row.id}>
                                    {
                                        row
                                            .getVisibleCells()
                                            .map(cell => (
                                                <Table.Td key={cell.id}>
                                                    {
                                                        flexRender(
                                                            cell.column.columnDef.cell,
                                                            cell.getContext()
                                                        )
                                                    }
                                                </Table.Td>
                                            ))
                                    }
                                </Table.Tr>
                            ))
                    }
                </Table.Tbody>
            </Table>
        </Table.ScrollContainer>
    </>
}

type ProjectJobListWrapperProps = {
    jobs:           ProjectJob[],

    onSelect?: (selected: ProjectJobMinimal[]) => void;
}
