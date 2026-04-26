import { CopyTable } from "@internal/misc/CopyTable";
import { CopyText } from "../misc/CopyText";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { EveIcon } from "@internal/misc/EveIcon";
import { Flex, Table, Text } from "@mantine/core";
import type { Item } from "@internal/services/item/model";
import type { ReactElement } from "react";

export function MaterialList({
    materials,
}: MaterialListProp): ReactElement {
    const columnHelper = createColumnHelper<MaterialListItem>();
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
            id: 'action',
            header: () => <CopyTable
                    value={materials.map(x => `${x.item.name}\t${x.quantity}`).join('\n')}
                />,
            meta: {
                align: 'right',
            },
            size: 1,
            maxSize: 1,
        }),
    ];

    const emptyTable = () => {
        if (materials.length === 0) {
            return <Table.Tr>
                <Table.Td colSpan={10}>
                    <Flex
                        justify="center"
                        align="center"
                    >
                        <Text>No data</Text>
                    </Flex>
                </Table.Td>
            </Table.Tr>
        }
    }

    const table = useReactTable<MaterialListItem>({
        columns: columns,
        data: materials,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    return <>
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
                            {
                                flexRender(
                                    header.column.columnDef.header,
                                    header.getContext()
                                )
                            }
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
                ))}
            </Table.Tbody>
        </Table>
    </>
}

export type MaterialListProp = {
    materials: MaterialListItem[];
}

export type MaterialListItem = {
    quantity:   number;
    item:       Item;
}
