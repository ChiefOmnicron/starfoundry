import { TableWrapper } from "@starfoundry/components/wrapper/Table";
import type { Appraisal, AppraisalItem } from "../create";
import { createColumnHelper, getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { EveIcon } from "@starfoundry/components/misc/EveIcon";
import { CopyText } from "@starfoundry/components/misc/CopyText";
import { Text } from "@mantine/core";

export function AppraisalItemTable({
    appraisal,
}: AppraisalItemTableProps) {
    const columnHelper = createColumnHelper<AppraisalItem>();
    const columns = [
        columnHelper.display({
            id: 'icon',
            cell: ({ row }) => <EveIcon
                id={row.original.item.type_id}
            />,
            size: 1,
            maxSize: 1,
        }),
        columnHelper.display({
            id: 'name',
            cell: ({ row }) => <CopyText
                value={row.original.item.name}
            />,
            header: () => 'Name',
            size: 20,
        }),
        columnHelper.display({
            id: 'quantity',
            cell: ({ row }) => <CopyText
                value={row.original.quantity}
            />,
            header: () => 'Quantity',
            size: 20,
        }),
        columnHelper.display({
            id: 'volume',
            cell: ({ row }) => <>
                <CopyText
                    value={row.original.quantity * row.original.item.volume}
                    number
                />
                <CopyText
                    value={row.original.item.volume}
                    size="xs"
                    number
                    withComma
                    disabled
                />
            </>,
            header: () => 'Volume (m3)',
            size: 20,
        }),
        columnHelper.display({
            id: 'buy',
            cell: ({ row }) => <>
                <CopyText
                    value={row.original.quantity * row.original.buy_price.max}
                    number
                    withComma
                />
                <CopyText
                    value={row.original.buy_price.max}
                    size="xs"
                    number
                    withComma
                    disabled
                />
            </>,
            header: () => 'Buy (ISK)',
            size: 20,
        }),
        columnHelper.display({
            id: 'split',
            cell: ({ row }) => <>
                <CopyText
                    value={
                        (
                            (row.original.quantity * row.original.buy_price.max) +
                            (row.original.quantity * row.original.sell_price.min)
                        ) / 2
                    }
                    number
                    withComma
                />
                <CopyText
                    value={
                        (
                            (row.original.buy_price.max) +
                            (row.original.sell_price.min)
                        ) / 2
                    }
                    size="xs"
                    number
                    withComma
                    disabled
                />
            </>,
            header: (x) => {
                x.header.colSpan = 2;
                return 'Split (ISK)'
            },
            size: 20,
        }),
        columnHelper.display({
            id: 'sell',
            cell: ({ row }) => <>
                <CopyText
                    value={row.original.quantity * row.original.sell_price.min}
                    number
                    withComma
                />
                <CopyText
                    value={row.original.sell_price.min}
                    size="xs"
                    number
                    withComma
                    disabled
                />
            </>,
            header: () => 'Sell (ISK)',
            size: 20,
        }),
    ];

    const table = useReactTable<AppraisalItem>({
        columns: columns,
        data: appraisal.items,
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    return <>
        <TableWrapper
            table={table}
        />
    </>
}

export type AppraisalItemTableProps = {
    appraisal: Appraisal,
}
