import { CopyText } from "./CopyText";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { EveIcon } from "@/components/EveIcon";
import { Flex, Table, Text, TextInput } from "@mantine/core";
import {useState, type ReactElement } from "react";
import { useListRigBlueprintBonus, type RigBlueprintBonus } from "@/services/structure/listRigBlueprintBonus";
import { LoadingAnimation } from "./LoadingAnimation";
import { LoadingError } from "./LoadingError";
import { systemRigBonusModifier } from "@/services/structure/utils";

export function BlueprintBonusList({
    rigTypeIds,

    systemSecurityStr,
}: BlueprintBonusListProps): ReactElement {
    let systemModifier = systemRigBonusModifier(systemSecurityStr);

    const [search, setSearch] = useState('');

    const columnHelper = createColumnHelper<RigBlueprintBonus>();
    const columns = [
        columnHelper.display({
            id: 'icon',
            cell: props => <EveIcon
                id={props.row.original.blueprint.type_id}
            />,
            size: 1,
            maxSize: 1,
        }),
        columnHelper.display({
            id: 'name',
            cell: props => <CopyText
                value={props.row.original.blueprint.name}
            />,
            header: () => 'Name',
        }),
        columnHelper.display({
            id: 'me',
            cell: props => <CopyText
                value={(props.row.original.bonus_me * systemModifier).toFixed(2)}
                display={`-${(props.row.original.bonus_me * systemModifier).toFixed(2)}%`}
            />,
            header: () => 'ME',
            size: 15,
        }),
        columnHelper.display({
            id: 'te',
            cell: props => <CopyText
                value={(props.row.original.bonus_te * systemModifier).toFixed(2)}
                display={`-${(props.row.original.bonus_te * systemModifier).toFixed(2)}%`}
            />,
            header: () => 'TE',
            size: 15,
        }),
    ];

    const {
        isPending,
        isError,
        data: blueprintBonuses,
    } = useListRigBlueprintBonus(rigTypeIds);

    const table = useReactTable<RigBlueprintBonus>({
        columns: columns,
        data: (blueprintBonuses || []).filter(x => x.blueprint.name.toLocaleLowerCase().indexOf(search.toLocaleLowerCase()) > -1),
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const emptyTable = () => {
        if (blueprintBonuses.length === 0) {
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

    return <>
        <TextInput
            label="Search"
            placeholder="Search for anything buildable"
            onChange={ (e) => setSearch(e.currentTarget.value) }
        />

        <Table.ScrollContainer minWidth={500} maxHeight={300}>
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
                                        .map(header => (
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
                                        )
                                    )
                                }
                            </Table.Tr>
                ))}
                </Table.Thead>

                <Table.Tbody>
                    { emptyTable() }

                    {
                        table
                            .getRowModel()
                            .rows
                            .map(row => (
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
                            )
                        )
                    }
                </Table.Tbody>
            </Table>
        </Table.ScrollContainer>
    </>
}

export type BlueprintBonusListProps = {
    rigTypeIds: number[];

    systemSecurityStr: string;
}
