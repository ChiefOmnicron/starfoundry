import { Alert, Button, Group, Stack, Table, Title } from '@mantine/core';
import { CopyText } from '@starfoundry/components/misc/CopyText';
import { EveIcon } from '@starfoundry/components/misc/EveIcon';
import { LIST_PROJECT_MARKET } from '@starfoundry/components/services/projects/listMarket';
import { MultiBuyModal } from '@/routes/projects_/-components/MultibuyModal';
import { type ProjectMarketBuyEntry } from '@starfoundry/components/services/projects/listMarketBuy';
import { updateMarketBulk, type UpdateMarketRequest } from '@starfoundry/components/services/projects/updateMarket';
import { useDisclosure } from '@mantine/hooks';
import { useState } from 'react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import type { GasDecompression, MineralCompression } from '@starfoundry/components/misc/CompressionMinimal';
import type { Uuid } from '@starfoundry/components/services/utils';
import type { Structure } from '@starfoundry/components/services/structure/list';

export function MarketBuy({
    projectId,

    marketData,
    structures,

    gasDecompression,
    mineralCompression,
}: MarketBuyProps) {
    const queryClient = useQueryClient();

    const [marketItems, setMarketItems] = useState<ProjectMarketBuyEntry[]>([]);
    const [marketSource, setMarketSource] = useState<string>('Unknown');
    const [marketSourceId, setMarketSourceId] = useState<number>(0);

    const [updateError, setUpdateError] = useState<boolean>(false);
    const [updateSuccess, setUpdateSuccess] = useState<boolean>(false);

    const [buyMaterialModalOpened, {
        open: openBuyMaterialModal,
        close: closeBuyMaterialModal,
    }] = useDisclosure(false);

    const updateMarketMutation = useMutation({
        mutationFn: async (data: UpdateMarketRequest) => {
            return await updateMarketBulk(
                projectId,
                data,
            )
        },
        onSuccess: () => {
            setUpdateError(false);
            setUpdateSuccess(true);
            queryClient.invalidateQueries({
                queryKey: [LIST_PROJECT_MARKET, projectId]
            });
        },
        onError: () => {
            setUpdateError(true);
            setUpdateSuccess(false);
        }
    });

    const showError = () => {
        if (updateError) {
            return <Alert
                variant='light'
                color='red'
                data-cy="updateError"
                onClose={ () => setUpdateError(false) }
                withCloseButton
            >
                There was an error while updating
            </Alert>;
        }
    }

    const showUpdateSuccess = () => {
        if (updateSuccess) {
            return <Alert
                variant='light'
                color='green'
                data-cy="updateSuccessful"
                onClose={ () => setUpdateSuccess(false) }
                withCloseButton
            >
                Market was successfully updated
            </Alert>;
        } else {
            return <></>;
        }
    }

    const structureNameAsSource = (structureId: number): string => {
        let structure = structures.find(x => x.structure_id === structureId);
        if (structure) {
            return structure.name;
        } else {
            return 'Unknown';
        }
    }

    const marketBuyTable = (
        marketData: ProjectMarketBuyEntry[],
    ) => {
        const byMarket: { [key: number]: ProjectMarketBuyEntry[] } = {};

        marketData
            .filter(x => !x.cost)
            .forEach(x => {
                for (const entry of x.entries) {
                    if (x.cost) {
                        continue;
                    }

                    if (entry.insufficient_data) {
                        continue;
                    }

                    if (!byMarket[entry.source]) {
                        byMarket[entry.source] = [];
                    }

                    byMarket[entry.source].push({
                        entries: [entry],
                        item: x.item,
                        quantity: x.quantity,
                        cost: x.cost,
                        source: x.source,
                    });
                }
            });

        const tables = Object
            .keys(byMarket)
            .map((x: any) => {
                const data = byMarket[x];

                const rows = data
                    .map(x => <Table.Tr>
                        <Table.Td>
                            <EveIcon
                                id={x.item.type_id}
                            />
                        </Table.Td>
                        <Table.Td>
                            <CopyText
                                value={x.item.name}
                            />
                        </Table.Td>
                        <Table.Td>
                            <CopyText
                                value={x.entries[0].quantity}
                                number
                            />
                        </Table.Td>
                        <Table.Td>
                            <CopyText
                                value={x.entries[0].price}
                                number
                            /> ISK
                        </Table.Td>
                        <Table.Td>
                            <CopyText
                                value={x.entries[0].price * x.entries[0].quantity}
                                number
                            /> ISK
                        </Table.Td>
                        <Table.Td>
                            <CopyText
                                value={x.item.volume * x.entries[0].quantity}
                                number
                            /> m3
                        </Table.Td>
                    </Table.Tr>);

                const volume = data
                    .map(x => x.item.volume * x.entries[0].quantity)
                    .reduce((prev, curr) => prev += curr, 0);
                const cost = data
                    .map(x => x.entries[0].price * x.entries[0].quantity)
                    .reduce((prev, curr) => prev += curr, 0);

                return <>
                    <Group justify='space-between'>
                        <Title order={2}>{structureNameAsSource(data[0].entries[0].source)}</Title>

                        <Button
                            onClick={() => {
                                setMarketItems(data);
                                setMarketSource(structureNameAsSource(data[0].entries[0].source))
                                setMarketSourceId(data[0].entries[0].source);
                                openBuyMaterialModal();
                            }}
                        >
                            Buy
                        </Button>
                    </Group>

                    <Table
                        variant="vertical"
                        layout="fixed"
                    >
                        <Table.Tbody>
                            <Table.Tr>
                                <Table.Th w={200}>Volume</Table.Th>
                                <Table.Td>
                                    <CopyText
                                        value={volume}
                                        number
                                    /> m3
                                </Table.Td>
                            </Table.Tr>
                            <Table.Tr>
                                <Table.Th w={200}>Cost</Table.Th>
                                <Table.Td>
                                    <CopyText
                                        value={cost}
                                        number
                                    /> ISK
                                </Table.Td>
                            </Table.Tr>
                            <Table.Tr>
                                <Table.Th w={200}>Last updated</Table.Th>
                                <Table.Td>
                                    <CopyText
                                        value={data[0].entries[0].last_fetch}
                                        date
                                    />
                                </Table.Td>
                            </Table.Tr>
                        </Table.Tbody>
                    </Table>

                    <Table.ScrollContainer
                        minWidth={500}
                        maxHeight={200}
                    >
                        <Table>
                            <Table.Thead>
                                <Table.Tr>
                                    <Table.Th w={32}></Table.Th>
                                    <Table.Th>Name</Table.Th>
                                    <Table.Th>Quantity</Table.Th>
                                    <Table.Th>Cost Per</Table.Th>
                                    <Table.Th>Cost Total</Table.Th>
                                    <Table.Th>Volume Total</Table.Th>
                                </Table.Tr>
                            </Table.Thead>

                            <Table.Tbody>
                                {rows}
                            </Table.Tbody>
                        </Table>
                    </Table.ScrollContainer>
                </>
            });

        return <Stack>
            {tables}
        </Stack>
    }

    const insufficientData = (
        marketData: ProjectMarketBuyEntry[],
    ) => {
        const rows = marketData
            .map(marketData =>
                marketData.entries
                    .filter(y => y.insufficient_data)
                    .map(x =>
                        <Table.Tr>
                            <Table.Td>
                                <EveIcon
                                    id={marketData.item.type_id}
                                />
                            </Table.Td>
                            <Table.Td>
                                <CopyText
                                    value={marketData.item.name}
                                />
                            </Table.Td>
                            <Table.Td>
                                <CopyText
                                    value={x.quantity}
                                    number
                                />
                            </Table.Td>
                        </Table.Tr>
                    )
            )
            .filter(x => x.length > 0);

        if (rows.length === 0) {
            return <></>;
        }

        return <>
            <Stack>
                <Title order={2}>Insufficient materials</Title>

                <Table.ScrollContainer
                    minWidth={500}
                    maxHeight={200}
                >
                    <Table>
                        <Table.Thead>
                            <Table.Tr>
                                <Table.Th w={32}></Table.Th>
                                <Table.Th>Name</Table.Th>
                                <Table.Th>Quantity</Table.Th>
                            </Table.Tr>
                        </Table.Thead>

                        <Table.Tbody>
                            {rows}
                        </Table.Tbody>
                    </Table>
                </Table.ScrollContainer>
            </Stack>
        </>
    }

    return <>
        {showError()}
        {showUpdateSuccess()}

        <MultiBuyModal
            items={marketItems}
            source={marketSource}
            onSave={(x) => {
                let entries = x
                    .split(`\n`)
                    .filter(y => !y.startsWith('Total:'))
                    .map(x => {
                        let entries = x.split(`\t`);
                        return {
                            name:           entries[0],
                            quantity:       Number.parseInt(entries[1]),
                            cost:           Number.parseFloat(entries[3]),
                            structure_id:   marketSourceId > 0 ? marketSourceId : undefined,
                        }
                    });

                updateMarketMutation.mutate({
                    source:                 marketSource,
                    entries:                entries,
                    gas_decompression:      gasDecompression,
                    mineral_compression:    mineralCompression,
                });
                closeBuyMaterialModal();
            }}

            opened={buyMaterialModalOpened}
            close={closeBuyMaterialModal}
        />

        <Stack>
            {insufficientData(marketData)}
            {marketBuyTable(marketData)}
        </Stack>
    </>
}

export type MarketBuyProps = {
    projectId: Uuid,

    marketData: ProjectMarketBuyEntry[];
    structures: Structure[];

    gasDecompression?: GasDecompression;
    mineralCompression?: MineralCompression;
}
