import { BadgeWrapper } from "../wrapper/Badge";
import { CopyText } from "../misc/CopyText";
import { createColumnHelper, flexRender, tableFeatures, useTable, type ColumnDef, columnSizingFeature, columnVisibilityFeature } from "@tanstack/react-table";
import { EveIcon } from "../misc/EveIcon";
import { Flex, Table, Text } from "@mantine/core";
import {useMemo, type ReactElement } from "react";
import type { AuthedCharacterInfo } from "../services/character/list";

export function CharacterTable({
    characters,
}: CharacterListProps): ReactElement {
    const features = tableFeatures({
        columnSizingFeature,
        columnVisibilityFeature,
    });
    const columnHelper = createColumnHelper<typeof features, AuthedCharacterInfo>();
    const columns = useMemo<ColumnDef<typeof features, AuthedCharacterInfo>[]>(
        () => [
            columnHelper.display({
                id: 'character_icon',
                cell: props => <EveIcon
                    id={props.row.original.character_id}
                    category="characters"
                    type="portrait"
                />,
                size: 1,
                maxSize: 1,
            }),
            columnHelper.display({
                id: 'character_name',
                cell: props => <CopyText
                    value={props.row.original.character_name}
                />,
                header: () => 'Character',
                size: 30,
            }),
            columnHelper.display({
                id: 'corporation_icon',
                cell: props => <EveIcon
                    id={props.row.original.corporation_id}
                    category="corporations"
                    type="logo"
                />,
                header: () => '',
                size: 1,
                maxSize: 1,
            }),
            columnHelper.display({
                id: 'corporation_name',
                cell: props => <CopyText
                    value={props.row.original.corporation_name}
                />,
                header: () => 'Corporation',
                size: 30,
            }),
            columnHelper.display({
                id: 'corporation',
                cell: props => props.row.original.scopes.indexOf('esi-industry.read_corporation_jobs.v1') > -1
                    ? <BadgeWrapper color="green.9">Corporation authed</BadgeWrapper>
                    : <BadgeWrapper color="gray.9">Corporation not authed</BadgeWrapper>,
                header: () => '',
                size: 1,
                maxSize: 10,
            }),
        ],
        [],
    );

    const table = useTable<typeof features, AuthedCharacterInfo>({
        features: features,
        columns: columns,
        data: characters,
    });

    const emptyTable = () => {
        if (characters.length === 0) {
            return <Table.Tr>
                <Table.Td colSpan={10}>
                    <Flex
                        justify="center"
                        align="center"
                    >
                        <Text>No characters authed</Text>
                    </Flex>
                </Table.Td>
            </Table.Tr>
        }
    }

    return <>
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

export type CharacterListProps = {
    characters: AuthedCharacterInfo[];
}
