import { Button, Checkbox, Flex, Group, Stack, Table, Text, Title } from "@mantine/core";
import { CopyText } from "@internal/misc/CopyText";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable, type ColumnDef, type RowSelectionState } from "@tanstack/react-table";
import { EveIcon } from "@internal/misc/EveIcon";
import { useEffect, useMemo, useState } from "react";
import { LIST_PROJECT_JOBS, type ProjectJob, type ProjectJobGroup, type ProjectJobStatus } from "@internal/services/projects/listJobs"
import { useQueryClient } from "@tanstack/react-query";

export function ProjectJobList({
    jobs,

    status,

    checkable = false,
    groupByHeader = false,
}: ProjectJobListProps) {
    const queryClient = useQueryClient();
    const [hasSelected, setHasSelected] = useState<boolean>(false);

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

    const onSelect = (selected: { [key: string]: boolean }) => {
        setHasSelected(Object.keys(selected).length > 0)
    }

    const refreshJobs = () => {
        queryClient.invalidateQueries({ queryKey: [LIST_PROJECT_JOBS] })
    }

    const buttonGroup = () => {
        if (checkable) {
            return <>
                <Group justify="flex-end">
                    <Button
                        disabled={!hasSelected}
                        variant="outline"
                    >
                        Create Build order
                    </Button>
                    <Button
                        disabled={!hasSelected}
                        variant="outline"
                    >
                        Check Materials
                    </Button>
                    <Button
                        variant="outline"
                        onClick={refreshJobs}
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
                        onClick={refreshJobs}
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
            />;
        }
    }

    return <>
        <Stack>
            {buttonGroup()}

            {content()}
        </Stack>
    </>
}

function ProjectJobListTable({
    jobs,

    checkable = false,
    onSelect = () => {},
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
                size: 2,
                maxSize: 2,
            }),
            columnHelper.display({
                id: 'icon',
                cell: props => <EveIcon
                    id={props.row.original.item.type_id}
                />,
                size: 4,
                maxSize: 4,
            }),
            columnHelper.display({
                id: 'name',
                cell: props => <CopyText
                    value={props.row.original.item.name}
                />,
                header: () => 'Name',
                size: 45,
            }),
            columnHelper.display({
                id: 'runs',
                cell: props => <CopyText
                    value={props.row.original.runs}
                />,
                header: () => 'Runs',
                size: 10,
            }),
            columnHelper.display({
                id: 'structure',
                cell: props => <CopyText
                    value={props.row.original.structure.name}
                />,
                header: () => 'Structure',
                size: 45,
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
            }
        },
        state: {
            rowSelection,
        },
    });

    useEffect(() => {
        onSelect(rowSelection || {});
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
    
    groupByHeader?: boolean;
    checkable?:     boolean;
}

export type ProjectJobListTableProps = {
    jobs: ProjectJob[];

    checkable?: boolean;
    onSelect?: (selected: { [key: string]: boolean }) => void;
}
