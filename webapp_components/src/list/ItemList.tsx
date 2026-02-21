import { Button, Flex, Table, Text } from "@mantine/core";
import { CopyText } from "../misc/CopyText";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { EveIcon } from "@internal/misc/EveIcon";
import { ItemSelectorModal } from "../selectors/ItemSelectorModal";
import { useDisclosure } from "@mantine/hooks";
import type { Item } from "@internal/services/item/model";
import type {ReactElement } from "react";

// Implementation for an editable list
//
// ```
// const [selectedItems, setSelectedItems] = useState<Item[]>([]);
//
// const {
//     isError: isError,
//     isPending: isPendingMarket,
//     data: defaultBlacklist,
// } = useListProjectGroupDefaultBlacklist(projectGroupId);
//
// useEffect(() => {
//     setSelectedItems(defaultBlacklist);
// }, [defaultBlacklist]);
//
// const addSelectedItem = (items: Item[]) => {
//     setSelectedItem(items);
// }
//
//  <ItemList
//      items={defaultBlacklist}
//      onSelect={addSelectedItem}
//  />
// ```
export function ItemList({
    selected: entries,

    blueprint = false,
    buildable = false,

    // editable
    editable = false,
    onSelect = (_) => {},
}: ItemListProp): ReactElement {
    const [opened, { open, close }] = useDisclosure(false);

    const columnHelper = createColumnHelper<Item>();
    const columns = [
        columnHelper.display({
            id: 'icon',
            cell: props => <EveIcon
                id={props.row.original.type_id}
            />,
            size: 4,
            maxSize: 4,
        }),
        columnHelper.accessor('name', {
            id: 'name',
            cell: props => <CopyText
                value={props.row.original.name}
            />,
            header: () => 'Name',
            size: 50,
        }),
        columnHelper.display({
            id: 'volume',
            cell: props => <>
                <CopyText
                    value={props.row.original.volume}
                />
                m3
            </>,
            header: () => 'Volume',
            size: 5,
        }),
        columnHelper.display({
            id: 'group',
            cell: props => <CopyText
                value={props.row.original.group.name}
            />,
            header: () => 'Group',
            size: 10,
        }),
        columnHelper.display({
            id: 'category',
            cell: props => <CopyText
                value={props.row.original.category.name}
            />,
            header: () => 'Category',
            size: 10,
        }),
    ];

    const emptyTable = () => {
        if (entries.length === 0) {
            return <Table.Tr>
                <Table.Td colSpan={10}>
                    <Flex
                        justify="center"
                        align="center"
                    >
                        <Text>No data yet</Text>
                    </Flex>
                </Table.Td>
            </Table.Tr>
        }
    }

    const table = useReactTable<Item>({
        columns: columns,
        data: entries
            .sort((a, b) => a.name.localeCompare(b.name)),
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
        initialState: {
            columnVisibility: {
                delete: editable,
            }
        }
    });

    const addItems = () => {
        if (editable) {
            return <>
                <ItemSelectorModal
                    opened={ opened }
                    onClose={ close }
                    onSelect={(items: Item[]) => {
                        onSelect(items);
                        close();
                    }}

                    selected={(entries || [])}
                    blueprint={blueprint}
                    buildable={buildable}
                />

                <Flex
                    justify='end'
                >
                    <Button
                        onClick={ open }
                    >
                        Edit items
                    </Button>
                </Flex>
            </>;
        } else {
            return <></>;
        }
    }

    return <>
        { addItems() }

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

export type ItemListProp = {
    selected: Item[];

    // options for the item selector
    blueprint?: boolean;
    buildable?: boolean;

    // options for editing
    editable?: boolean;
    onSelect?: (items: Item[]) => void;
}
