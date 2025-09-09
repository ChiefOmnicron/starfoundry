import { Button, Table } from "@mantine/core";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import type { ReactElement } from "react";
import { StructureSelector } from "@/components/StructureSelector";
import { EveIcon } from "@/components/EveIcon";
import type { Structure } from "@/services/structure/list";

const columnHelper = createColumnHelper<Structure>();
const columns = [
    columnHelper.display({
        id: 'icon',
        cell: props => <EveIcon
            id={props.row.original.structure.type_id}
        />,
        size: 16,
    }),
    columnHelper.accessor('name', {
        id: 'name',
        cell: props => props.getValue(),
        header: () => 'Name',
        size: 750,
    }),
    columnHelper.accessor('id', {
        id: 'delete',
        cell: (_: any) => <Button>
            Delete
        </Button>,
        header: () => 'Projects',
        size: 50,
    }),
];

export function ProjectGroupMarket ({
    entries,
}: ProjectGroupMarketProp): ReactElement {
    const table = useReactTable<Structure>({
        columns: columns,
        data: entries,
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
                {table.getRowModel().rows.map(row => (
                    <Table.Tr key={row.id}>
                        {
                            row.getVisibleCells().map(cell => (
                                <Table.Td
                                    key={cell.id}
                                    style={{
                                        width: `16px`
                                    }}
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
            <Table.Tfoot>
                <tr>
                    <td colSpan={3}>
                        <StructureSelector
                            onSelect={(uuid) => console.log('x', uuid)}
                            filters={{
                                // Standup Market Hub I
                                service_id: 35892
                            }}
                            selected={(entries || []).map(x => x.id)}
                        />
                    </td>
                </tr>
            </Table.Tfoot>
        </Table>
    </>
}

export type ProjectGroupMarketProp = {
    entries: Structure[];
}
