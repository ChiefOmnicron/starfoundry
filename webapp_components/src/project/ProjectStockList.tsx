import { CopyText } from "../misc/CopyText";
import { createColumnHelper, tableFeatures, useTable, columnSizingFeature, columnVisibilityFeature, flexRender } from "@tanstack/react-table";
import { EveIcon } from "../misc/EveIcon";
import type { ReactElement } from "react";
import type { ProjectStock } from "../services/projects/fetch";
import { CopyTable } from "../misc/CopyTable";
import { Table } from "@mantine/core";

export function ProjectStockList({
    stock,
}: ProjectStockListProp): ReactElement {
    const features = tableFeatures({
        columnSizingFeature,
        columnVisibilityFeature,
    });
    const columnHelper = createColumnHelper<typeof features, ProjectStock>();
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

    const table = useTable<typeof features, ProjectStock>({
        features:   features,
        columns:    columns,
        data:       stock,
    });

    return <>
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
    </>
}

export type ProjectStockListProp = {
    stock: ProjectStock[];
}
