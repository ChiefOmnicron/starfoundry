import { Table} from "@mantine/core";
import { flexRender, type Table as ReactTable, type RowData, type TableFeatures } from "@tanstack/react-table";

export function TableWrapper<TFeatures extends TableFeatures, TData extends RowData>({
    scrollable = false,

    table,
}: TableWrapperProps<TFeatures, TData>) {
    if (table._features.columnSizingFeature) {

    }

    const headerWidth = (header: any): string | undefined => {
        return table._features.columnSizingFeature
        ? `${(header as any).getSize()}%`
        : undefined
    }

    const columns = (row: any) => {
        if (table._features.columnVisibilityFeature) {
            (row as any)
                .getVisibleCells()
                .map((cell: any) => (
                    <Table.Td key={cell.id}>
                        {
                            flexRender(
                                cell.column.columnDef.cell,
                                cell.getContext()
                            )
                        }
                    </Table.Td>
                ))
        } else {
            return row
                .getAllCells()
                .map((cell: any) => (
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
    }

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
                                    .map(header => {
                                        return <Table.Th
                                            key={header.id}
                                            style={{
                                                width: headerWidth(header)
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
                            { columns(row) }
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

interface TableWrapperProps<TFeatures extends TableFeatures, TData extends RowData> {
    scrollable?:    boolean;

    table:          ReactTable<TFeatures, TData>,
}
