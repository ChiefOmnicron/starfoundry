import { CopyText } from "../misc/CopyText";
import { createColumnHelper, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { EveIcon } from "@internal/misc/EveIcon";
import type { ReactElement } from "react";
import type { ProjectStock } from "@internal/services/projects/fetch";
import { TableWrapper } from "@internal/wrapper/Table";
import { CopyTable } from "@internal/misc/CopyTable";

export function ProjectStockList({
    stock,
}: ProjectStockListProp): ReactElement {
    const columnHelper = createColumnHelper<ProjectStock>();
    const columns = [
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
            size: 50,
        }),
        columnHelper.display({
            id: 'quantity',
            cell: props => <>
                <CopyText
                    value={props.row.original.quantity}
                    number
                />
            </>,
            header: () => 'Quantity',
            size: 5,
        }),
        columnHelper.display({
            id: 'cost',
            cell: props => <>
                <CopyText
                    value={props.row.original.cost}
                    number
                />
            </>,
            header: () => 'Cost',
            size: 5,
        }),
        columnHelper.display({
            id: 'action',
            header: () => <CopyTable
                    value={stock.map(x => `${x.item.name}\t${x.quantity}\t${x.cost ? x.cost : '-/-'}`).join('\n')}
                />,
            meta: {
                align: 'right',
            },
            size: 1,
            maxSize: 1,
        }),
    ];

    const table = useReactTable<ProjectStock>({
        columns: columns,
        data: stock,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    return <>
        <TableWrapper
            table={table}
        />
    </>
}

export type ProjectStockListProp = {
    stock: ProjectStock[];
}
