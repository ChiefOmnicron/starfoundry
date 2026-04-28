import { Table} from "@mantine/core";
import { flexRender, type Table as ReactTable } from "@tanstack/react-table";

export function TableWrapper<T>({
    scrollable = false,

    table,
}: TableWrapperProps<T>) {
    const tableView = <Table stickyHeader striped data-cy="data">
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
    </Table>;

    if (scrollable) {
        return <>
            <Table.ScrollContainer minWidth={100} maxHeight={500}>
                {tableView}
            </Table.ScrollContainer>
        </>
    } else {
        return <>
            {tableView}
        </>;
    }
}

interface TableWrapperProps<T> {
    scrollable?:    boolean;

    table:          ReactTable<T>,
}
