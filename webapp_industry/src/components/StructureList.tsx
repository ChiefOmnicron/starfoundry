import { Table, UnstyledButton } from "@mantine/core";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { Dotlan } from "./Dotlan";
import { EveIcon } from "@/components/EveIcon";
import { StructureSelector, type StructureSelectorRef } from "@/components/selectors/StructureSelector";
import { useRef, useState, type ReactElement } from "react";
import type { Structure } from "@/services/structure/list";
import { StructureLink } from "./StructureLink";
import type { Uuid } from "@/services/utils";

// Implementation for an editable list
//
// ```
// const [selectedStructures, setSelectedStructures] = useState<Structure[]>([]);
//
//  const {
//      isError: isErrorMarket,
//      isPending: isPendingMarket,
//      data: defaultMarket,
//  } = useListProjectGroupDefaultMarkets(projectGroupId);
//
//  const {
//      isPending: isPendingStructures,
//      isError: isErrorStructures,
//      data: structures,
//  } = useListStructure({});
//
// useEffect(() => {
//     setSelectedStructures(structureGroup.structures);
// }, [structureGroup]);
//
// const removeSelectedStructure = (structureId: string) => {
//     const removedStructure = selectedStructures
//         .filter(x => x.id !== structureId);
//     setSelectedStructures(removedStructure)
// }
// const addSelectedStructure = (structure: Structure) => {
//     setSelectedStructures([
//         structure,
//         ...selectedStructures,
//     ]);
// }
//
//  <StructureList
//      structures={selectedStructures}
//      selectableStructures={structures}
//      onDelete={removeSelectedStructure}
//      onSelect={addSelectedStructure}
//  />
// ```
export function StructureList({
    structures: entries,

    // editable
    onDelete = (_) => {},
    onSelect = (_) => {},
    selectableStructures = [],
}: StructureListProp): ReactElement {
    const [addStructureSelect, setAddStructureSelect] = useState<Structure | undefined>();
    const structureSelectorRef = useRef<StructureSelectorRef>({} as any);

    const columnHelper = createColumnHelper<Structure>();
    const columns = [
        columnHelper.display({
            id: 'icon',
            cell: props => <EveIcon
                id={props.row.original.item.type_id}
            />,
            size: 1,
            maxSize: 1,
        }),
        columnHelper.accessor('name', {
            id: 'name',
            cell: props => <StructureLink structure={props.row.original} />,
            header: () => 'Name',
            size: 50,
        }),
        columnHelper.display({
            id: 'location',
            cell: props => <Dotlan system={props.row.original.system} />,
            header: () => 'Location',
            size: 50,
        }),
        columnHelper.accessor('id', {
            id: 'delete',
            cell: props => <UnstyledButton
                    style={{
                        color: 'var(--mantine-color-red-filled)',
                    }}
                    onClick={() => {
                        onDelete(props.row.original.id);
                    }}
                >
                    Remove
                </UnstyledButton>,
            header: () => '',
            size: 1,
            maxSize: 1,
        }),
    ];

    const footer = () => {
        if (selectableStructures.length === 0) {
            return <></>;
        }

        return <Table.Tfoot>
            <tr>
                <Table.Td
                    colSpan={3}
                    style={{
                        padding: 0,
                    }}
                >
                    <StructureSelector
                        onSelect={setAddStructureSelect}
                        structures={selectableStructures}
                        selected={(entries || []).map(x => x.id)}
                        ref={structureSelectorRef as any}
                    />
                </Table.Td>
                <Table.Td>
                    <UnstyledButton
                        onClick={() => {
                            if (addStructureSelect) {
                                onSelect(addStructureSelect);
                                setAddStructureSelect(undefined);
                                structureSelectorRef.current.reset();
                            }
                        }}
                    >
                        Add
                    </UnstyledButton>
                </Table.Td>
            </tr>
        </Table.Tfoot>
    }

    const table = useReactTable<Structure>({
        columns: columns,
        data: entries,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
        initialState: {
            columnVisibility: {
                delete: selectableStructures.length > 0,
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

export type StructureListProp = {
    structures: Structure[];

    selectableStructures?: Structure[];
    onDelete?:             (id: Uuid) => void;
    onSelect?:             (structure: Structure) => void;
}
