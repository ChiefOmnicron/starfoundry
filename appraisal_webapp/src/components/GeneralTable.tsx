import { Group, Table } from "@mantine/core";
import type { Appraisal } from "@starfoundry/components/src/services/appraisal/create";
import { CopyText } from "@starfoundry/components/misc/CopyText";
import { MARKETS } from "@/services/utils";

export function GeneralTable({
    appraisal,
}: GeneralTableProps) {
    const totalBuy = appraisal
        .items
        .map(x => x.buy_price.max * x.quantity)
        .reduce((prev: number, curr: number) => prev += curr, 0);
    const totalSell = appraisal
        .items
        .map(x => x.sell_price.min * x.quantity)
        .reduce((prev: number, curr: number) => prev += curr, 0);
    const totalSplit = (totalBuy + totalSell) / 2;

    const totalVolume = appraisal
        .items
        .map(x => x.item.volume * x.quantity)
        .reduce((prev: number, curr: number) => prev += curr, 0);

    const market = MARKETS.find(y => y.value === appraisal.market_id.toString())

    return <>
        <Table variant="vertical">
            <Table.Tbody>
                <Table.Tr>
                    <Table.Th w="15%">Created At</Table.Th>
                    <Table.Td w="35%">
                        <CopyText
                            value={appraisal.created_at_ts}
                            date
                        />
                    </Table.Td>

                    <Table.Th w="15%">Total Buy</Table.Th>
                    <Table.Td w="35%">
                        <Group justify="flex-end">
                            <CopyText
                                value={totalBuy}
                                suffix="ISK"
                                number
                                withComma
                            />
                        </Group>
                    </Table.Td>
                </Table.Tr>

                <Table.Tr>
                    <Table.Th>Market Data At</Table.Th>
                    <Table.Td>
                        <CopyText
                            value={appraisal.items[0].last_fetch}
                            date
                        />
                    </Table.Td>

                    <Table.Th>Total Split</Table.Th>
                    <Table.Td>
                        <Group justify="flex-end">
                            <CopyText
                                value={totalSplit}
                                suffix="ISK"
                                number
                                withComma
                            />
                        </Group>
                    </Table.Td>
                </Table.Tr>

                <Table.Tr>
                    <Table.Th>Market</Table.Th>
                    <Table.Td>
                        <CopyText
                            value={market ? market.label : 'Unknown'}
                        />
                    </Table.Td>

                    <Table.Th>Total Sell</Table.Th>
                    <Table.Td>
                        <Group justify="flex-end">
                            <CopyText
                                value={totalSell}
                                suffix="ISK"
                                number
                                withComma
                            />
                        </Group>
                    </Table.Td>
                </Table.Tr>

                <Table.Tr>
                    <Table.Th>Mode</Table.Th>
                    <Table.Td>
                        <CopyText
                            value={appraisal.mode === 'APPRAISAL' ? 'Appraisal' : 'Multibuy'}
                        />
                    </Table.Td>

                    <Table.Th>Total Volume</Table.Th>
                    <Table.Td>
                        <Group justify="flex-end">
                            <CopyText
                                value={totalVolume}
                                suffix="m3"
                                number
                                withComma
                            />
                        </Group>
                    </Table.Td>
                </Table.Tr>
            </Table.Tbody>
        </Table>
    </>
}

export type GeneralTableProps = {
    appraisal: Appraisal,
}
