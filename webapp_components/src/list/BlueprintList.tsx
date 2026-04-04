import { ActionIcon, Button, Flex, Table, Text, Tooltip } from "@mantine/core";
import { CopyText } from "../misc/CopyText";
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { EveIcon } from "@internal/misc/EveIcon";
import { faCopy } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useClipboard } from "@mantine/hooks";
import {useEffect, useState, type ReactElement } from "react";
import type { Item } from "@internal/services/item/model";

export function BlueprintList({
    blueprints,
}: BlueprintListProp): ReactElement {
    const clipboard = useClipboard();
    const [tooltipOpened, setTooltipOpened] = useState<boolean>(false);

    const [storedBlueprints, setStoredBlueprints] = useState<number[]>([]);

    useEffect(() => {
        if (tooltipOpened) {
            setTimeout(() => setTooltipOpened(false), 1000);
        }
    }, [tooltipOpened]);

    const columnHelper = createColumnHelper<BlueprintListItem>();
    const columns = [
        columnHelper.display({
            id: 'icon',
            cell: props => <EveIcon
                id={props.row.original.item.type_id}
                category="types"
            />,
            size: 1,
            maxSize: 1,
        }),
        columnHelper.display({
            id: 'name',
            cell: props => <CopyText
                value={props.row.original.item.name}
                disabled={storedBlueprints.indexOf(props.row.original.item.type_id) > -1}
            />,
            header: () => 'Name',
            size: 50,
        }),
        columnHelper.display({
            id: 'quantity',
            cell: props => <>
                <CopyText
                    value={props.row.original.runs.length}
                    disabled={storedBlueprints.indexOf(props.row.original.item.type_id) > -1}
                />
            </>,
            header: () => 'Quantity',
            size: 5,
        }),
        columnHelper.display({
            id: 'runs',
            cell: props => <>
                <CopyText
                    value={props.row.original.runs.map(x => `${x}`).join(', ')}
                    disabled={storedBlueprints.indexOf(props.row.original.item.type_id) > -1}
                />
            </>,
            header: () => 'Runs',
            size: 5,
        }),
        columnHelper.display({
            id: 'action',
            header: () => <Tooltip
                    label="Copied!"
                    position="top"
                    opened={tooltipOpened}
                >
                    <ActionIcon
                        color="gray"
                        variant="transparent"
                        onClick={copy}
                    >
                        <FontAwesomeIcon icon={faCopy} />
                    </ActionIcon>
                </Tooltip>,
            cell: props => <Button
                    onClick={() => {
                        setStoredBlueprints(Array.from(new Set([...storedBlueprints, props.row.original.item.type_id])))
                    }}
                    disabled={storedBlueprints.indexOf(props.row.original.item.type_id) > -1}
                >
                    Stored
                </Button>,
            meta: {
                align: 'right',
            },
            size: 5,
        }),
    ];

    const copy = () => {
        setTooltipOpened(true);
        let content = blueprints
            .map(x => `${x.item.name}\t${x.runs.length}\t${x.runs}`)
            .join('\n');
        clipboard.copy(content);
    }

    const emptyTable = () => {
        if (blueprints.length === 0) {
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

    const table = useReactTable<BlueprintListItem>({
        columns: columns,
        data: blueprints,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    return <>
        <Table.ScrollContainer
            minWidth={300}
            maxHeight={300}
        >
            <Table
                striped
                stickyHeader
                data-cy="data"
            >
                <Table.Thead>
                {table.getHeaderGroups().map(headerGroup => (
                    <Table.Tr key={headerGroup.id}>
                        {headerGroup.headers.map(header => (
                            <Table.Th
                                key={header.id}
                                ta={(header.column.columnDef.meta as any)?.align}
                                style={{
                                    width: `${header.getSize()}%`
                                }}
                            >
                                {
                                    flexRender(
                                        header.column.columnDef.header,
                                        header.getContext()
                                    )
                                }
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
                                    <Table.Td
                                        key={cell.id}
                                    >
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
        </Table.ScrollContainer>
    </>
}

export type BlueprintListProp = {
    blueprints: BlueprintListItem[];
}

export type BlueprintListItem = {
    runs:       number[];
    item:       Item;
}
