import { Button, Flex, NumberInput, Table, Text } from "@mantine/core";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { EveIcon } from "@/components/EveIcon";
import { ItemSelector, type ItemSelectorRef } from "@/components/selectors/ItemSelectorInline";
import { useRef, useState, type ReactElement } from "react";
import type { TypeId } from "@/services/utils";
import type { Item } from "@/services/item/model";
import type { BlueprintOverwrite } from "@/services/project-group/listDefaultBlueprintOverwrites";
import { CopyText } from "@/components/CopyText";

// Implementation for an editable list
//
// ```
// const [selectedItems, setSelectedItems] = useState<Item[]>([]);
//
// const {
//     isError: isError,
//     isPending: isPendingMarket,
//     data: defaultBlueprintOverwrites,
// } = useListProjectGroupDefaultBlueprintOverwrites(projectGroupId);
//
// useEffect(() => {
//     setSelectedItems(defaultBlueprintOverwrites);
// }, [defaultBlueprintOverwrites]);
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
//  <BlueprintOverwriteList
//      items={defaultBlueprintOverwrites}
//      onDelete={removeSelectedItem}
//      onSelect={addSelectedItem}
//  />
// ```
export function BlueprintOverwriteList({
    selected: entries,

    // editable
    editable = false,
    onDelete = (_) => {},
    onSelect = (_) => {},
}: BlueprintOverwriteListProp): ReactElement {
    const [addItemSelect, setAddItemSelect] = useState<Item | undefined>();
    const [materialEfficiency, setMaterialEfficiency] = useState<string | undefined>();
    const ItemSelectorRef = useRef<ItemSelectorRef>({} as any);

    const columnHelper = createColumnHelper<BlueprintOverwrite>();
    const columns = [
        columnHelper.display({
            id: 'icon',
            cell: props => <EveIcon
                id={props.row.original.item.type_id}
                type="bp"
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
            size: 68,
        }),
        columnHelper.display({
            id: 'me',
            cell: props => <CopyText
                value={props.row.original.material_efficiency}
            />,
            header: () => 'Material Efficiency',
            size: 10,
        }),
        columnHelper.display({
            id: 'delete',
            cell: props => <Button
                    variant="outline"
                    color="#c92a2a"
                    style={{
                        width: '100%'
                    }}
                    onClick={() => {
                        onDelete(props.row.original.item.type_id);
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
                        selected={(entries || []).map(x => x.item.type_id)}
                        ref={ItemSelectorRef as any}
                        blueprint
                    />
                </Table.Td>
                <Table.Td
                    style={{
                        padding: 0
                    }}
                >
                    <NumberInput
                        placeholder="Set ME"
                        onChange={(v) => setMaterialEfficiency(v as string)}
                        value={materialEfficiency}
                        min={0}
                        max={10}
                    />
                </Table.Td>
                <Table.Td>
                    <Button
                        disabled={!addItemSelect || !materialEfficiency}
                        variant="outline"
                        onClick={() => {
                            if (addItemSelect && materialEfficiency) {
                                onSelect({
                                    item:                addItemSelect,
                                    material_efficiency: Number.parseInt(materialEfficiency),
                                });

                                setAddItemSelect(undefined);
                                setMaterialEfficiency('');
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

    const table = useReactTable<BlueprintOverwrite>({
        columns: columns,
        data: entries
            .sort((a, b) => a.item.name.localeCompare(b.item.name)),
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

export type BlueprintOverwriteListProp = {
    selected: BlueprintOverwrite[];

    // options for editing
    editable?: boolean;
    onDelete?: (id: TypeId) => void;
    onSelect?: (blueprintOverwrite: BlueprintOverwrite) => void;
}
