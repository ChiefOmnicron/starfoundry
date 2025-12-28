import { Button, Flex, Table, Text } from "@mantine/core";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { EveIcon } from "@/components/EveIcon";
import { ItemSelector, type ItemSelectorRef } from "@/components/selectors/ItemSelector";
import { useRef, useState, type ReactElement } from "react";
import type { TypeId } from "@/services/utils";
import type { Item } from "@/services/item/model";

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
// const removeSelectedItem = (typeId: TypeId) => {
//     const removedItem = selectedItems
//         .filter(x => x.type_id !== typeId);
//     setSelectedItems(removedItem)
// }
// const addSelectedItem = (item: Item) => {
//     setSelectedItem([
//         item,
//         ...selectedItem,
//     ]);
// }
//
//  <ItemList
//      items={defaultBlacklist}
//      onDelete={removeSelectedItem}
//      onSelect={addSelectedItem}
//  />
// ```
export function ItemList({
    selected: entries,

    blueprint = false,
    buildable = false,

    // editable
    editable = false,
    onDelete = (_) => {},
    onSelect = (_) => {},
}: ItemListProp): ReactElement {
    const [addItemSelect, setAddItemSelect] = useState<Item | undefined>();
    const ItemSelectorRef = useRef<ItemSelectorRef>({} as any);

    const columnHelper = createColumnHelper<Item>();
    const columns = [
        columnHelper.display({
            id: 'icon',
            cell: props => <EveIcon
                id={props.row.original.type_id}
            />,
            size: 1,
            maxSize: 1,
        }),
        columnHelper.accessor('name', {
            id: 'name',
            cell: props => props.row.original.name,
            header: () => 'Name',
            size: 78,
        }),
        columnHelper.accessor('type_id', {
            id: 'delete',
            cell: props => <Button
                    variant="outline"
                    color="#c92a2a"
                    style={{
                        width: '100%'
                    }}
                    onClick={() => {
                        onDelete(props.row.original.type_id);
                    }}
                >
                    Remove
                </Button>,
            header: () => '',
            size: 6,
            maxSize: 6,
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

    const footer = () => {
        if (!editable) {
            return <></>;
        }

        return <Table.Tfoot>
            <tr>
                <Table.Td
                    colSpan={2}
                >
                    <ItemSelector
                        onSelect={setAddItemSelect}
                        selected={(entries || []).map(x => x.type_id)}
                        ref={ItemSelectorRef as any}
                        blueprint={blueprint}
                        buildable={buildable}
                    />
                </Table.Td>
                <Table.Td>
                    <Button
                        disabled={!addItemSelect}
                        variant="outline"
                        onClick={() => {
                            if (addItemSelect) {
                                onSelect(addItemSelect);
                                setAddItemSelect(undefined);
                                ItemSelectorRef.current.reset();
                            }
                        }}
                        style={{
                            width: '100%'
                        }}
                    >
                        Add
                    </Button>
                </Table.Td>
            </tr>
        </Table.Tfoot>
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
            { footer() }
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
    onDelete?: (id: TypeId) => void;
    onSelect?: (item: Item) => void;
}
