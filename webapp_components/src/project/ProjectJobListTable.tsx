import { Alert, Button, Checkbox, Group, Table } from "@mantine/core";
import { CopyTable } from "../misc/CopyTable";
import { CopyText } from "../misc/CopyText";
import { Countdown } from "../misc/Countdown";
import { createColumnHelper, useTable, type RowSelectionState, columnSizingFeature, columnVisibilityFeature, rowSelectionFeature, tableFeatures, flexRender } from "@tanstack/react-table";
import { EveIcon } from "../misc/EveIcon";
import { JobStatusBadge } from "./JobStatusBadge";
import { memo, useEffect, useState } from "react";
import { Nakamura } from "../misc/Nakamura";
import { ProjectJobEditModal } from "./ProjectJobEditModal";
import { useDisclosure } from "@mantine/hooks";
import type { ProjectJob } from "../services/projects/listJobs";
import type { ProjectJobMinimal } from "./ProjectJobAction";
import type { Uuid } from "../services/utils";

export const ProjectJobListTable = function ProjectJobListTableImp({
    projectId,
    jobs,

    showCost = false,
    showStatus = false,
    showRemaining = false,
    showStarted = false,
    showEdit = false,
    showDelete = false,

    checkable = false,
    onSelect = () => {},

    onDelete = () => {},

    onJobSplit = () => {}
}: ProjectJobListTableProps) {
    const [rowSelection, setRowSelection] = useState<RowSelectionState>({});

    const [editJobModalOpened, { open: editJobModalOpen, close: editJobModalClose }] = useDisclosure(false);
    const [editJob, setEditJob] = useState<ProjectJob>({} as ProjectJob);

    const [started, setStarted] = useState<Uuid[]>([]);

    const features = tableFeatures({
        columnSizingFeature,
        columnVisibilityFeature,
        rowSelectionFeature,
    });
    const columnHelper = createColumnHelper<typeof features, ProjectJob>();
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
            cell: ({ row }) => <>
                <Group>
                    {
                        showEdit
                        ?   <>
                                <Button
                                    onClick={() => {
                                        setEditJob(row.original);
                                        editJobModalOpen();
                                    }}
                                    variant="subtle"
                                >
                                    Edit
                                </Button>
                            </>
                        :   <></>
                    }

                    {
                        showDelete
                        ?   <>
                                <Button
                                    color="red.9"
                                    onClick={() => {
                                        onDelete(row.original.id)
                                    }}
                                    variant="subtle"
                                >
                                    Delete
                                </Button>
                            </>
                        :   <></>
                    }

                    {
                        showStarted
                        ?   <>
                                <Button
                                    onClick={() => {
                                        setStarted([...started, row.original.id]);
                                    }}
                                    disabled={started.indexOf(row.original.id) > -1}
                                >
                                    Started
                                </Button>
                            </>
                        :   <></>
                    }
                </Group>
            </>,
            meta: {
                align: 'right',
            },
            size: 10,
            maxSize: 10,
        }),
    ];

    const table = useTable<typeof features, ProjectJob>({
        features:           features,
        columns:            columns,
        data:               jobs,
        onRowSelectionChange: (selected) => {
            setRowSelection(selected);
        },
        //getRowCanExpand: () => true,
        getRowId: row => row.id,
        initialState: {
            columnVisibility: {
                check: checkable,
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

    // must stay, otherwise the selection change is not properly triggered
    useEffect(() => {
        onSelect(
            projectId,
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

    const emptyTable = () => {
        if (jobs.length === 0) {
            return <Alert
                variant="light"
                title="No jobs"
            >
                There are no jobs currently available.
            </Alert>
        }
    }

    if (jobs.length === 0) {
        return emptyTable();
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

            onJobSplit={onJobSplit}
        />

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
                                            .map(header => {
                                                return <Table.Th
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
                                            })
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

export const ProjectJobListTableMemo = memo(ProjectJobListTable);

export type ProjectJobListTableProps = {
    projectId:      Uuid;
    jobs:           ProjectJob[];

    showCost?:      boolean;
    showStatus?:    boolean;
    showRemaining?: boolean;
    showStarted?:   boolean;
    showEdit?:      boolean;
    showDelete?:    boolean;

    scrollable?:    boolean;

    checkable?: boolean;
    onSelect?: (projectId: Uuid, selected: ProjectJobMinimal[]) => void;

    onDelete?: (jobId: Uuid) => void;

    onJobSplit?: () => void;
}
