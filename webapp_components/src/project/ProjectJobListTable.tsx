import { Alert, Button, Checkbox, Group } from "@mantine/core";
import { CopyTable } from "@internal/misc/CopyTable";
import { CopyText } from "@internal/misc/CopyText";
import { Countdown } from "@internal/misc/Countdown";
import { createColumnHelper, getCoreRowModel, useReactTable, type RowSelectionState } from "@tanstack/react-table";
import { EveIcon } from "@internal/misc/EveIcon";
import { JobStatusBadge } from "./JobStatusBadge";
import { Nakamura } from "@internal/misc/Nakamura";
import { ProjectJobEditModal } from "./ProjectJobEditModal";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";
import type { ProjectJob } from "@internal/services/projects/listJobs";
import type { Uuid } from "@internal/services/utils";
import type { ProjectJobMinimal } from "./ProjectJobAction";
import { TableWrapper } from "@internal/wrapper/Table";

export function ProjectJobListTable({
    projectId,
    jobs,

    showCost = false,
    showStatus = false,
    showRemaining = false,
    showStarted = false,

    checkable = false,
    onSelect = () => {},

    onDelete = () => {},

    editable = false,
    showQuickFix = false,
}: ProjectJobListTableProps) {
    const [rowSelection, setRowSelection] = useState<RowSelectionState>({});

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
                    return <Group>
                        <Button
                            onClick={() => {
                                setEditJob(row.original);
                                editJobModalOpen();
                            }}
                            variant="subtle"
                        >
                            Edit
                        </Button>
                        <Button
                            color="red.9"
                            onClick={() => {
                                console.log(onDelete, row.original.id)
                                onDelete(row.original.id)
                            }}
                            variant="subtle"
                        >
                            Delete
                        </Button>
                    </Group>;
                } else if (showStarted) {
                    return <Button
                        onClick={() => {
                            setStarted([...started, row.original.id]);
                        }}
                        disabled={started.indexOf(row.original.id) > -1}
                    >
                        Started
                    </Button>;
                }
            },
            meta: {
                align: 'right',
            },
            size: 10,
            maxSize: 10,
        }),
    ];

    const table = useReactTable<ProjectJob>({
        columns: columns,
        data: jobs,
        autoResetPageIndex: false,
        onRowSelectionChange: (selected) => {
            setRowSelection(selected);
        },
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
        />

        <TableWrapper
            table={table}
            scrollable
        />
    </>
}

export type ProjectJobListTableProps = {
    projectId:      Uuid;
    jobs:           ProjectJob[];

    showCost?:      boolean;
    showStatus?:    boolean;
    showRemaining?: boolean;
    showStarted?:   boolean;

    scrollable?:    boolean;

    checkable?: boolean;
    onSelect?: (selected: ProjectJobMinimal[]) => void;

    onDelete?: (jobId: Uuid) => void;

    editable?:      boolean;
    showQuickFix?:  boolean;
}

