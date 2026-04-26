import { Button, Checkbox, Flex, Group, Stack, Table, Text, Title } from "@mantine/core";
import { CheckResourcesModal } from "@internal/project/CheckResourcesModal";
import { CopyTable } from "@internal/misc/CopyTable";
import { CopyText } from "@internal/misc/CopyText";
import { Countdown } from "@internal/misc/Countdown";
import { CreateBuildOrderModal } from "@internal/project/CreateBuildOrderModal";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable, type RowSelectionState } from "@tanstack/react-table";
import { EveIcon } from "@internal/misc/EveIcon";
import { JobStatusBadge } from "@internal/project/JobStatusBadge";
import { LIST_PROJECT_JOBS, type ProjectJob, type ProjectJobGroup, type ProjectJobStatus } from "@internal/services/projects/listJobs"
import { Nakamura } from "@internal/misc/Nakamura";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { ProjectJobEditModal } from "./ProjectJobEditModal";
import type { Uuid } from "@internal/services/utils";

export function ProjectJobList({
    projectId,
    jobs,

    status,

    showCost = false,
    showStatus = false,
    showRemaining = false,
    showStarted = false,

    checkable = false,
    groupByHeader = false,

    editable = false,
    showQuickFix = false,
}: ProjectJobListProps) {
    const queryClient = useQueryClient();
    const [selectedRows, setSelectedRows] = useState<ProjectJob[]>([]);

    const [checkResourcesModalOpened, { open: checkResourcesModalOpen, close: checkResourcesModalClose }] = useDisclosure(false);
    const [createBuildOrderModalOpened, { open: createBuildOrderModalOpen, close: createBuildOrderModalClose }] = useDisclosure(false);

    const header = (
        header: string,
    ): string => {
        switch (header) {
            case 'INTERMEDIATE_REACTIONS':
                return 'Intermediate Reactions';
            case 'COMPOSITE_REACTIONS':
                return 'Composite Reactions';
            case 'BIOCHEM_REACTIONS':
                return 'Biochemical Reactions';
            case 'HYBRID_REACTIONS':
                return 'Hybrid Reactions';
            case 'CONSTRUCTION_COMPONENTS':
                return 'Construction Components';
            case 'ADVANCED_CAPITAL_CONSTRUCTION_COMPONENTS':
                return 'Advanced Capital Construction Components';
            case 'CAPITAL_CONSTRUCTION_COMPONENTS':
                return 'Capital Construction Components';
            case 'TOOLS':
                return 'Tools';
            case 'T1_STUFF':
                return 'T1 Stuff';
            case 'T2_STUFF':
                return 'T2 Stuff';
            case 'CHARGES':
                return 'Charges';
            case 'SHIPS':
                return 'Ships';
            default:
                return 'Unknown'
        }
    }

    const onSelect = (projectJobs: ProjectJob[]) => {
        setSelectedRows(projectJobs)
    }

    const refreshJobsClick = () => {
        queryClient.invalidateQueries({ queryKey: [LIST_PROJECT_JOBS] })
    }

    const createBuildOrder = () => {
        createBuildOrderModalOpen();
    }

    const checkResourcesClick = () => {
        checkResourcesModalOpen();
    }

    const buttonGroup = () => {
        const hasSelectedRows = selectedRows.length > 0;

        if (checkable) {
            return <>
                <Group justify="flex-end">
                    <Button
                        disabled={!hasSelectedRows}
                        variant="outline"
                        onClick={createBuildOrder}
                    >
                        Create Build order
                    </Button>
                    <Button
                        disabled={!hasSelectedRows}
                        variant="outline"
                        onClick={checkResourcesClick}
                    >
                        Check Materials
                    </Button>
                    <Button
                        variant="outline"
                        onClick={refreshJobsClick}
                    >
                        Refresh
                    </Button>
                </Group>
            </>
        } else {
            return <>
                <Group justify="flex-end">
                    <Button
                        variant="outline"
                        onClick={refreshJobsClick}
                    >
                        Refresh
                    </Button>
                </Group>
            </>
        }
    }

    const content = () => {
        if (groupByHeader) {
            return jobs
                .filter(x => {
                    if (!status) {
                        return true;
                    }

                    return !!x.entries.find(y => y.status === status);
                })
                .map(x => {
                    let entries = x.entries;
                    if (status) {
                        entries = x
                            .entries
                            .filter(y => y.status === status);
                    }

                    return <>
                        <Title order={3}>{header(x.header)}</Title>
                        <ProjectJobListTable
                            projectId={projectId}
                            jobs={entries}

                            checkable={checkable}
                            onSelect={onSelect}

                            editable={editable}
                            showQuickFix={showQuickFix}

                            showCost={showCost}
                            showStatus={showStatus}
                            showRemaining={showRemaining}
                        />
                    </>
                })
        } else {
            const flattened = jobs
                .flatMap(x => x.entries)
                .filter(x => {
                    if (!status) {
                        return true;
                    }

                    return x.status === status;
                });
            return <ProjectJobListTable
                projectId={projectId}
                jobs={flattened}

                checkable={checkable}
                onSelect={onSelect}

                editable={editable}
                showQuickFix={showQuickFix}

                showCost={showCost}
                showStatus={showStatus}
                showRemaining={showRemaining}
                showStarted={showStarted}
            />;
        }
    }

    return <>
        <CheckResourcesModal
            jobIds={selectedRows.map(x => x.id)}
            close={checkResourcesModalClose}
            opened={checkResourcesModalOpened}
        />

        <CreateBuildOrderModal
            jobIds={selectedRows.map(x => x.id)}
            close={createBuildOrderModalClose}
            opened={createBuildOrderModalOpened}
        />

        <Stack>
            {buttonGroup()}

            {content()}
        </Stack>
    </>
}

function ProjectJobListTable({
    projectId,
    jobs,

    showCost = false,
    showStatus = false,
    showRemaining = false,
    showStarted = false,

    checkable = false,
    onSelect = () => {},

    editable = false,
    showQuickFix = false,
}: ProjectJobListTableProps) {
    const [rowSelection, setRowSelection] = useState<RowSelectionState>({})

    const [editJobModalOpened, { open: editJobModalOpen, close: editJobModalClose }] = useDisclosure(false);
    const [editJob, setEditJob] = useState<ProjectJob>({} as ProjectJob);

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
            id: 'status',
            cell: ({ row }) => <JobStatusBadge
                    jobStatus={row.original.status}
                    size="md"
                />,
            header: () => 'Status',
            size: 8,
            maxSize: 8,
        }),
        columnHelper.display({
            id: 'cost',
            cell: ({ row }) => 
                row.original.cost
                    ?   <CopyText
                            value={row.original.cost}
                            number
                        />
                    : '-/-',
            header: () => 'Cost',
            size: 8,
            maxSize: 8,
        }),
        columnHelper.display({
            id: 'remaining',
            cell: ({ row }) => <Countdown
                    endDate={row.original.end_date || ''}
                />,
            header: () => 'Remaining',
            size: 10,
            maxSize: 10,
        }),
        columnHelper.display({
            id: 'endDate',
            cell: ({ row }) => <Nakamura
                    endDate={row.original.end_date || ''}
                />,
            header: () => 'End date (local)',
            size: 10,
            maxSize: 10,
        }),
        columnHelper.display({
            id: 'action',
            header: () => <CopyTable
                    value={jobs.map(x => `${x.item.name}\t${x.runs}\t${x.structure.name}`).join('\n')}
                />,
            cell: ({ row }) => {
                if (showQuickFix) {
                    return <Button
                        variant="outline"
                    >
                        Quick Fix
                    </Button>;
                } else if (editable) {
                    return <Button
                        onClick={() => {
                            setEditJob(row.original);
                            editJobModalOpen();
                        }}
                    >
                        Edit
                    </Button>;
                } else if (showStarted) {
                    return <Button
                        onClick={() => {
                            setStarted([...started, row.original.id]);
                        }}
                    >
                        Started
                    </Button>;
                }
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
        onRowSelectionChange: setRowSelection,
        getRowCanExpand: () => true,
        getCoreRowModel: getCoreRowModel(),
        getRowId: row => row.id,
        initialState: {
            columnVisibility: {
                check: checkable,
                edit: editable,
                cost: showCost,
                status: showStatus,
                remaining: showRemaining,
                endDate: showRemaining,
            }
        },
        state: {
            rowSelection,
        },
    });

    useEffect(() => {
        onSelect(table.getSelectedRowModel().rows.map(x => x.original));
    }, [rowSelection]);

    const emptyTable = () => {
        if (jobs.length === 0) {
            return <Table.Tr>
                <Table.Td colSpan={10}>
                    <Flex
                        justify="center"
                        align="center"
                    >
                        <Text>No matching jobs</Text>
                    </Flex>
                </Table.Td>
            </Table.Tr>
        }
    }

    return <>
        <ProjectJobEditModal
            projectId={projectId}
            job={editJob}
            close={() => {
                setEditJob({} as ProjectJob);
                editJobModalClose();
            }}
            opened={editJobModalOpened && Object.keys(editJob).length > 0}
        />

        <Table striped data-cy="data">
            <Table.Thead>
            {table.getHeaderGroups().map(headerGroup => (
                <Table.Tr key={headerGroup.id}>
                    {headerGroup.headers.map(header => (
                        <Table.Th
                            key={header.id}
                            ta={(header.column.columnDef.meta as any)?.align}
                            style={{
                                width: `${header.getSize()}%`
                            }}
                        >
                            {flexRender(
                                header.column.columnDef.header,
                                header.getContext()
                            )}
                        </Table.Th>
                    ))}
                </Table.Tr>
            ))}
            </Table.Thead>

            <Table.Tbody>
                { emptyTable() }

                {table.getRowModel().rows.map(row => (
                    <>
                        <Table.Tr key={row.id}>
                            {
                                row.getVisibleCells().map(cell => (
                                    <Table.Td
                                        key={cell.id}
                                    >
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
                    </>
                ))}
            </Table.Tbody>
        </Table>
    </>
}

export type ProjectJobListProps = {
    projectId:      Uuid;
    jobs:           ProjectJobGroup[];

    status?:        ProjectJobStatus,

    showCost?:      boolean;
    showStatus?:    boolean;
    showRemaining?: boolean;
    showStarted?:   boolean;

    groupByHeader?: boolean;
    checkable?:     boolean;
    editable?:      boolean;
    showQuickFix?:  boolean;
}

export type ProjectJobListTableProps = {
    projectId:      Uuid;
    jobs:           ProjectJob[];

    showCost?:      boolean;
    showStatus?:    boolean;
    showRemaining?: boolean;
    showStarted?:   boolean;

    checkable?: boolean;
    onSelect?: (selected: ProjectJob[]) => void;

    editable?:      boolean;
    showQuickFix?:  boolean;
}
