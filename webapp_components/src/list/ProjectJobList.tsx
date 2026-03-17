import { Button, Checkbox, Flex, Group, Stack, Table, Text, Title } from "@mantine/core";
import { CheckResourcesModal } from "@internal/project/CheckResourcesModal";
import { CopyText } from "@internal/misc/CopyText";
import { Countdown } from "@internal/misc/Countdown";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable, type ColumnDef, type RowSelectionState } from "@tanstack/react-table";
import { EveIcon } from "@internal/misc/EveIcon";
import { JobStatusBadge } from "@internal/project/JobStatusBadge";
import { LIST_PROJECT_JOBS, type ProjectJob, type ProjectJobGroup, type ProjectJobStatus } from "@internal/services/projects/listJobs"
import { Nakamura } from "@internal/misc/Nakamura";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useMemo, useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { CreateBuildOrderModal } from "@internal/project/CreateBuildOrderModal";

export function ProjectJobList({
    jobs,

    status,

    showCost = false,
    showStatus = false,
    showRemaining = false,

    checkable = false,
    groupByHeader = false,

    editable = false,
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
                            jobs={entries}

                            checkable={checkable}
                            onSelect={onSelect}

                            editable={editable}

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
                jobs={flattened}

                checkable={checkable}
                onSelect={onSelect}

                editable={editable}

                showCost={showCost}
                showStatus={showStatus}
                showRemaining={showRemaining}
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
    jobs,

    showCost = false,
    showStatus = false,
    showRemaining = false,

    checkable = false,
    onSelect = () => {},

    editable = false,
}: ProjectJobListTableProps) {
    const [rowSelection, setRowSelection] = useState<RowSelectionState>({})

    const columnHelper = createColumnHelper<ProjectJob>();
    const columns = useMemo<ColumnDef<ProjectJob>[]>(
        () => [
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
                cell: props => <EveIcon
                    id={props.row.original.item.type_id}
                />,
                size: 1,
                maxSize: 1,
            }),
            columnHelper.display({
                id: 'name',
                cell: props => <CopyText
                    value={props.row.original.item.name}
                />,
                header: () => 'Name',
                size: 20,
            }),
            columnHelper.display({
                id: 'runs',
                cell: props => <CopyText
                    value={props.row.original.runs}
                />,
                header: () => 'Runs',
                size: 3,
                maxSize: 3,
            }),
            columnHelper.display({
                id: 'structure',
                cell: props => <CopyText
                    value={props.row.original.structure.name}
                />,
                header: () => 'Structure',
                size: 15,
            }),
            columnHelper.display({
                id: 'status',
                cell: props => <JobStatusBadge
                        jobStatus={props.row.original.status}
                        size="md"
                    />,
                header: () => 'Status',
                size: 8,
                maxSize: 8,
            }),
            columnHelper.display({
                id: 'cost',
                cell: props => <CopyText
                        value={props.row.original.cost}
                        number
                    />,
                header: () => 'Cost',
                size: 5,
                maxSize: 5,
            }),
            columnHelper.display({
                id: 'countdown',
                cell: props => <Countdown
                        endDate={props.row.original.end_date || ''}
                    />,
                header: () => 'Remaining',
                size: 10,
                maxSize: 10,
            }),
            columnHelper.display({
                id: 'remaining',
                cell: props => <Nakamura
                        endDate={props.row.original.end_date || ''}
                    />,
                header: () => 'End date',
                size: 10,
                maxSize: 10,
            }),
            columnHelper.display({
                id: 'edit',
                cell: _ => <Button>
                        Edit
                    </Button>,
                header: () => '',
                size: 5,
                maxSize: 5,
            }),
        ],
        []
    );

    const table = useReactTable<ProjectJob>({
        columns: columns,
        data: jobs,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
        onRowSelectionChange: setRowSelection,
        getRowId: row => row.id,
        initialState: {
            columnVisibility: {
                check: checkable,
                edit: editable,
                cost: showCost,
                status: showStatus,
                countdown: showRemaining,
                remaining: showRemaining,
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
        <Table striped data-cy="data">
            <Table.Thead>
            {table.getHeaderGroups().map(headerGroup => (
                <Table.Tr key={headerGroup.id}>
                    {headerGroup.headers.map(header => (
                        <Table.Th
                            key={header.id}
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
                    <Table.Tr key={row.id}>
                        {
                            row.getVisibleCells().map(cell => (
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
                ))}
            </Table.Tbody>
        </Table>
    </>
}

export type ProjectJobListProps = {
    jobs: ProjectJobGroup[];

    status?:        ProjectJobStatus,

    showCost?:      boolean;
    showStatus?:    boolean;
    showRemaining?: boolean;

    groupByHeader?: boolean;
    checkable?:     boolean;
    editable?:      boolean;
}

export type ProjectJobListTableProps = {
    jobs: ProjectJob[];

    showCost?:      boolean;
    showStatus?:    boolean;
    showRemaining?: boolean;

    checkable?: boolean;
    onSelect?: (selected: ProjectJob[]) => void;

    editable?: boolean;
}
