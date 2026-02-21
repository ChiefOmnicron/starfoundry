import { CloseButton, Flex, Table, Text, TextInput } from "@mantine/core";
import { CopyText } from "../misc/CopyText";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { EveIcon } from "@internal/misc/EveIcon";
import { LoadingAnimation } from "../misc/LoadingAnimation";
import { LoadingError } from "../misc/LoadingError";
import { systemRigBonusModifier } from "@internal/services/structure/utils";
import { useListRigBlueprintBonus, type RigBlueprintBonus } from "@internal/services/structure/listRigBlueprintBonus";
import {useState, type ReactElement } from "react";

export function BlueprintBonusList({
    rigs,
    services,

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
    } = useListRigBlueprintBonus({
        rigs,
        services,
    });

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
            value={search}
            onChange={ (e) => setSearch(e.currentTarget.value) }
            rightSection={
                <CloseButton
                    aria-label="Clear input"
                    onClick={() => setSearch('')}
                    style={{ display: search ? undefined : 'none' }}
                />
            }
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
    rigs:     number[];
    services: number[];

    systemSecurityStr: string;
}
