import { CloseButton, Flex, Table, Text, TextInput } from "@mantine/core";
import { CopyText } from "../misc/CopyText";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable, type ColumnDef } from "@tanstack/react-table";
import { EveIcon } from "@internal/misc/EveIcon";
import { LoadingAnimation } from "../misc/LoadingAnimation";
import { LoadingError } from "../misc/LoadingError";
import { systemRigBonusModifier } from "@internal/services/structure/utils";
import { useListRigBlueprintBonus, type RigBlueprintBonus } from "@internal/services/structure/listRigBlueprintBonus";
import {useMemo, useRef, useState, type ReactElement } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";

export function BlueprintBonusList({
    rigs,
    services,

    systemSecurityStr,
}: BlueprintBonusListProps): ReactElement {
    let systemModifier = systemRigBonusModifier(systemSecurityStr);

    const [search, setSearch] = useState('');

    const columnHelper = createColumnHelper<RigBlueprintBonus>();
    const columns = useMemo<ColumnDef<RigBlueprintBonus>[]>(
        () => [
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
                size: 60,
            }),
            columnHelper.display({
                id: 'me',
                cell: props => <CopyText
                    value={(props.row.original.bonus_me * systemModifier).toFixed(2)}
                    display={`-${(props.row.original.bonus_me * systemModifier).toFixed(2)}%`}
                />,
                header: () => 'ME',
                size: 10,
            }),
            columnHelper.display({
                id: 'te',
                cell: props => <CopyText
                    value={(props.row.original.bonus_te * systemModifier).toFixed(2)}
                    display={`-${(props.row.original.bonus_te * systemModifier).toFixed(2)}%`}
                />,
                header: () => 'TE',
                size: 10,
            }),
        ],
        [],
    );

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

    const parentRef = useRef<HTMLDivElement>(null);
    const { rows } = table.getRowModel();
    const virtualizer = useVirtualizer({
        count: rows.length,
        getScrollElement: () => parentRef.current,
        estimateSize: () => 34,
        overscan: 20,
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

    // during development this component has a huge performance problem
    // while deactivating it in development is not optimal, it is the best
    // solution until a better one is found
    if (process.env.NODE_ENV === 'development') {
        return <></>;
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

        <div ref={parentRef} className="container">
            <div style={{ height: `${virtualizer.getTotalSize()}px` }}>
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
                                virtualizer
                                    .getVirtualItems()
                                    .map(virtualRow => {
                                        const row = rows[virtualRow.index]
                                        return <Table.Tr key={row.id}>
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
                                    })
                            }
                        </Table.Tbody>
                    </Table>
                </Table.ScrollContainer>
            </div>
        </div>
    </>
}

export type BlueprintBonusListProps = {
    rigs:     number[];
    services: number[];

    systemSecurityStr: string;
}
