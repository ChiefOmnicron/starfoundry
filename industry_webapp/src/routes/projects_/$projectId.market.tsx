import { createFileRoute } from '@tanstack/react-router'
import { Alert, Button, Group, Stack, Table, Tabs, Title } from '@mantine/core';
import { LIST_PROJECT_MARKET, useListProjectMarket, type ProjectMarketEntry } from '@starfoundry/components/services/projects/listMarket';
import { EveIcon } from '@starfoundry/components/misc/EveIcon';
import { CopyText } from '@starfoundry/components/misc/CopyText';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { useDisclosure } from '@mantine/hooks';
import { MultiBuyModal } from './-components/MultibuyModal';
import { useState } from 'react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { updateMarketBulk, type UpdateMarketRequest } from '@starfoundry/components/services/projects/updateMarket';
import { useListProjectMarketBuy, type ProjectMarketBuyEntry } from '@starfoundry/components/services/projects/listMarketBuy';
import { CompressionMinimal, DEFAULT_GAS_BONUS } from '@starfoundry/components/misc/CompressionMinimal';

export const Route = createFileRoute('/projects_/$projectId/market')({
    component: RouteComponent,
});

const source = (source: number): string => {
    switch (source) {
        case 1049588174021:
            return 'C-J'
        case 1046664001931:
            return 'UALX'
        case 60003760:
            return 'Jita'
        case 60008494:
            return 'Amarr'
        default:
            return 'Unknown ' + source
    }
}

function RouteComponent() {
    const queryClient = useQueryClient();
    const { projectId } = Route.useParams();

    const [marketItems, setMarketItems] = useState<ProjectMarketBuyEntry[]>([]);
    const [marketSource, setMarketSource] = useState<string>('Unknown');
    const [marketSourceId, setMarketSourceId] = useState<number>(0);

    const [gasBonus, setGasBonus] = useState<string>(DEFAULT_GAS_BONUS);
    const [mineralBonus, setMineralBonus] = useState<string>(DEFAULT_GAS_BONUS);

    const [updateError, setUpdateError] = useState<boolean>(false);
    const [updateSuccess, setUpdateSuccess] = useState<boolean>(false);

    const [opened, { open, close }] = useDisclosure(false);

    const {
        isError: isErrorMulti,
        isPending: isPendingMulti,
        data: projectMarketMulti,
    } = useListProjectMarketBuy(projectId, {
        strategy: 'MULTI_BUY'
    });
    const {
        isError: isErrorSmart,
        isPending: isPendingSmart,
        data: projectMarketSmart,
    } = useListProjectMarketBuy(projectId, {
        strategy: 'SMART_BUY'
    });

    const {
        isError,
        isPending,
        data: projectMarket,
    } = useListProjectMarket(projectId);

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
    })

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

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

    const marketTable = (
        marketData: ProjectMarketEntry[],
    ) => {
        const rows = marketData
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
                        value={x.quantity}
                        number
                    />
                </Table.Td>
                <Table.Td>
                    {
                        x.source
                        ?   <CopyText
                                value={x.source}
                            />
                        :   <> -/- </>
                    }
                </Table.Td>
                <Table.Td>
                    {
                        x.cost
                        ?   <CopyText
                                value={x.cost / x.quantity}
                                number
                            />
                        :   <> -/- </>
                    }
                </Table.Td>
                <Table.Td>
                    {
                        x.cost
                        ?   <CopyText
                                value={x.cost}
                                number
                            />
                        :   <> -/- </>
                    }
                </Table.Td>
            </Table.Tr>);

        return <>
            <Table>
                <Table.Thead>
                    <Table.Tr>
                        <Table.Th w={32}></Table.Th>
                        <Table.Th>Name</Table.Th>
                        <Table.Th>Quantity</Table.Th>
                        <Table.Th>Source</Table.Th>
                        <Table.Th>Cost Per</Table.Th>
                        <Table.Th>Cost Total</Table.Th>
                    </Table.Tr>
                </Table.Thead>

                <Table.Tbody>
                    {rows}
                </Table.Tbody>
            </Table>
        </>;
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
                        <Title order={2}>{source(data[0].entries[0].source)}</Title>

                        <Button
                            onClick={() => {
                                setMarketItems(data);
                                setMarketSource(source(data[0].entries[0].source))
                                setMarketSourceId(data[0].entries[0].source);
                                open();
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

    return <>
        {showError()}
        {showUpdateSuccess()}

        <MultiBuyModal
            items={marketItems}
            source={marketSource}
            onSave={(x) => {
                let entries = x
                    .split(`\n`)
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
                    gas_decompression:      gasBonus,
                    mineral_compression:    mineralBonus,
                });
                close();
            }}

            opened={opened}
            close={close}
        />

        <Tabs defaultValue="overview">
            <Tabs.List>
                <Tabs.Tab value="overview">
                    Overview
                </Tabs.Tab>
                <Tabs.Tab value="multiBuy">
                    MultiBuy
                </Tabs.Tab>
                <Tabs.Tab value="smartBuy">
                    SmartBuy
                </Tabs.Tab>
            </Tabs.List>

            <Tabs.Panel value="overview">
                {marketTable(projectMarket)}
            </Tabs.Panel>
            <Tabs.Panel value="multiBuy">
                {
                    isPendingMulti
                    ?   LoadingAnimation()
                    :   isErrorMulti
                        ?   LoadingError()
                        : marketBuyTable(projectMarketMulti)
                }
            </Tabs.Panel>
            <Tabs.Panel value="smartBuy">
                TODO: add gas compression, selector - needs to be send to the server when updating

                TODO: add mineral compression

                <Stack>
                    <CompressionMinimal
                        onGasUpdate={setGasBonus}
                        onMineralUpdate={setMineralBonus}
                    />

                    {
                        isPendingSmart
                        ?   LoadingAnimation()
                        :   isErrorSmart
                            ?   LoadingError()
                            : marketBuyTable(projectMarketSmart)
                    }
                </Stack>
            </Tabs.Panel>
        </Tabs>
    </>
}

/*const haulingCost = (
    structure_id: number,
    quantity: number,
    volume: number,
) => {
    switch (structure_id) {
        case 1046664001931:
            return (quantity * volume) * (19_816_099 / 370_000);
        // C-J
        case 1049588174021:
            return (quantity * volume) * (113_886_795 / 370_000);
        // Jita
        case 60003760:
            return (quantity * volume) * 475
        // Amarr
        case 60008494:
            return (quantity * volume) * (173_566_003 / 370_000)
        default:
            return 0;
    }
}
*/
