import { Button, Flex, NumberInput, Table, Text, TextInput } from "@mantine/core";
import { CopyText } from "@starfoundry/components/misc/CopyText";
import type { ProjectMisc } from "@starfoundry/components/services/projects/listMisc";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { useState, type ReactElement } from "react";

// Implementation for an editable list
//
// ```
// const [selectedItems, setSelectedItems] = useState<Item[]>([]);
//
// const {
//     isError,
//     isPending,
//     data: projectMisc,
// } = useListProjectMisc(projectId);
//
// useEffect(() => {
//     setSelectedItems(projectMisc);
// }, [projectMisc]);
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
//  <ProjectMiscList
//      entries={projectMisc}
//      onDelete={removeSelectedItem}
//      onSelect={addSelectedItem}
//  />
// ```
export function ProjectMiscList({
    selected: entries,

    // editable
    editable = false,
    onDelete = (_) => {},
    onSelect = (_) => {},
}: ProjectMiscListProp): ReactElement {
    const [cost, setCost] = useState<number | undefined>();
    const [item, setItem] = useState<string | undefined>();
    const [quantity, setQuantity] = useState<number | undefined>();
    const [description, setDescription] = useState<string | undefined>();

    const columnHelper = createColumnHelper<ProjectMisc>();
    const columns = [
        columnHelper.display({
            id: 'item',
            cell: props => <CopyText
                value={props.row.original.item}
            />,
            header: () => 'Item',
            size: 30,
            maxSize: 30,
        }),
        columnHelper.display({
            id: 'quantity',
            cell: props => <CopyText
                value={props.row.original.quantity}
                number
            />,
            header: () => 'Quantity',
            size: 20,
        }),
        columnHelper.display({
            id: 'description',
            cell: props => <CopyText
                value={props.row.original.description}
            />,
            header: () => 'Description',
            size: 30,
        }),
        columnHelper.display({
            id: 'cost',
            cell: props => <CopyText
                value={props.row.original.cost}
                number
            />,
            header: () => 'Cost',
            size: 20,
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
                        onDelete(props.row.original);
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
                <Table.Td>
                    <TextInput
                        placeholder="Misc item"
                        onChange={(v) => setItem(v.target.value)}
                        value={item}
                    />
                </Table.Td>
                <Table.Td
                    style={{
                        padding: 0
                    }}
                >
                    <NumberInput
                        placeholder="Quantity"
                        onChange={(v) => setQuantity(v as number)}
                        value={quantity}
                    />
                </Table.Td>
                <Table.Td>
                    <TextInput
                        placeholder="Description"
                        onChange={(v) => setDescription(v.target.value)}
                        value={description}
                    />
                </Table.Td>
                <Table.Td
                    style={{
                        padding: 0
                    }}
                >
                    <NumberInput
                        placeholder="Cost"
                        onChange={(v) => setCost(v as number)}
                        value={cost}
                    />
                </Table.Td>
                <Table.Td>
                    <Button
                        disabled={!item || !cost}
                        variant="outline"
                        onClick={() => {
                            if (item && cost) {
                                onSelect({
                                    cost: cost,
                                    item: item,
                                    description: description,
                                    quantity: quantity,
                                });
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

    const table = useReactTable<ProjectMisc>({
        columns: columns,
        data: entries
            .sort((a, b) => a.item.localeCompare(b.item)),
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

export type ProjectMiscListProp = {
    selected: ProjectMisc[];

    // options for editing
    editable?: boolean;
    onDelete?: (misc: ProjectMisc) => void;
    onSelect?: (misc: ProjectMisc) => void;
}
