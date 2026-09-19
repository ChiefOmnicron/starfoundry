import { createColumnHelper, tableFeatures, useTable, columnSizingFeature, columnVisibilityFeature, metaHelper } from "@tanstack/react-table";
import { EveIcon } from "@starfoundry/components/misc/EveIcon";
import { CopyText } from "@starfoundry/components/misc/CopyText";
import { Group, Stack } from "@mantine/core";
import { TableWrapper } from '@starfoundry/components/wrapper';
import { CopyTable } from '@starfoundry/components/misc';
import type { Appraisal, AppraisalItem } from "@starfoundry/components/services/appraisal/fetch";

export function ItemTable({
    appraisal,
}: ItemTableProps) {
    interface TableMeta {
        rightAlign: boolean;
    }

    const features = tableFeatures({
        columnSizingFeature,
        columnVisibilityFeature,
        columnMeta: metaHelper<TableMeta>(),
    });

    const numberValue = (
        valueTop: number,
        valueBottom: number,
        unit: 'm3' | 'ISK',
    ) => {
        return <Group justify="flex-end">
            <Stack gap={1}>
                <CopyText
                    value={valueTop}
                    suffix={unit}
                    number
                    withComma
                />

                <Group justify="flex-end">
                    <CopyText
                        value={valueBottom}
                        suffix={unit}
                        size="sm"
                        number
                        withComma
                        muted
                    />
                </Group>
            </Stack>
        </Group>
    }

    const columnHelper = createColumnHelper<typeof features, AppraisalItem>();
    const columns = [
        columnHelper.display({
            id: 'icon',
            cell: ({ row }) => <EveIcon
                id={row.original.item.type_id}
            />,
            header: () => <CopyTable
                value={appraisal.items.map(x => {
                        const itemName = x.item.name;
                        const itemQuantity = x.quantity;
                        const totalVolume = x.quantity * x.item.volume;
                        const singleVolume = x.item.volume;

                        const totalBuy = x.quantity * x.buy_price.max;
                        const singleBuy = x.buy_price.max;
                        const totalSplit = (
                            (x.quantity * x.buy_price.max) +
                            (x.quantity * x.sell_price.min)
                        ) / 2;
                        const singleSplit = (
                            (x.buy_price.max) +
                            (x.sell_price.min)
                        ) / 2;
                        const totalSell = x.quantity * x.sell_price.min;
                        const singleSell = x.sell_price.min;

                        const totalString = `${totalVolume}\t${totalBuy}\t${totalSplit}\t${totalSell}`;
                        const singleString = `${singleVolume}\t${singleBuy}\t${singleSplit}\t${singleSell}`;

                        return `${itemName}\t${itemQuantity}\t${totalString}\t${singleString}`;
                    })
                    .join("\n")
                }
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
            size: 10,
        }),
        columnHelper.display({
            id: 'quantity',
            cell: ({ row }) => <Group justify="flex-end">
                    <CopyText
                        value={row.original.quantity}
                        number
                    />
                </Group>,
            header: () => 'Quantity',
            size: 5,
            maxSize: 10,
            meta: {
                rightAlign: true,
            }
        }),
        columnHelper.display({
            id: 'volume',
            cell: ({ row }) => numberValue(
                row.original.quantity * row.original.item.volume,
                row.original.item.volume,
                'm3',
            ),
            header: () => <>
                Total Volume<br />
                <div style={{
                    color: 'var(--mantine-color-disabled-color)'
                }}>
                    Single Volume
                </div>
            </>,
            size: 15,
            maxSize: 15,
            meta: {
                rightAlign: true,
            }
        }),
        columnHelper.display({
            id: 'buy',
            cell: ({ row }) => numberValue(
                row.original.quantity * row.original.buy_price.max,
                row.original.buy_price.max,
                'ISK',
            ),
            header: () => <>
                Total Buy<br />
                <div style={{
                    color: 'var(--mantine-color-disabled-color)'
                }}>
                    Single Buy
                </div>
            </>,
            size: 15,
            maxSize: 15,
            meta: {
                rightAlign: true,
            }
        }),
        columnHelper.display({
            id: 'split',
            cell: ({ row }) => numberValue(
                (
                    (row.original.quantity * row.original.buy_price.max) +
                    (row.original.quantity * row.original.sell_price.min)
                ) / 2,
                (
                    (row.original.buy_price.max) +
                    (row.original.sell_price.min)
                ) / 2,
                'ISK',
            ),
            header: () => <>
                Total Split<br />
                <div style={{
                    color: 'var(--mantine-color-disabled-color)'
                }}>
                    Single Split
                </div>
            </>,
            size: 15,
            maxSize: 15,
            meta: {
                rightAlign: true,
            }
        }),
        columnHelper.display({
            id: 'sell',
            cell: ({ row }) => numberValue(
                row.original.quantity * row.original.sell_price.min,
                row.original.sell_price.min,
                'ISK',
            ),
            header: () => <>
                Total Sell<br />
                <div style={{
                    color: 'var(--mantine-color-disabled-color)'
                }}>
                    Single Sell
                </div>
            </>,
            size: 15,
            maxSize: 15,
            meta: {
                rightAlign: true,
            }
        }),
    ];

    const table = useTable<typeof features, AppraisalItem>({
        features: features,
        columns: columns,
        data: appraisal.items,
    });

    return <>
        <TableWrapper
            table={table}
        />
    </>
}

export type ItemTableProps = {
    appraisal: Appraisal,
}
