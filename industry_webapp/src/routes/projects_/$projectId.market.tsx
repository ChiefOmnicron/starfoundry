import { Alert, Button, Group, Table, Tabs } from '@mantine/core';
import { CopyText } from '@starfoundry/components/misc/CopyText';
import { createFileRoute } from '@tanstack/react-router'
import { EveIcon } from '@starfoundry/components/misc/EveIcon';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { MultiBuyTab } from './-components/MultiBuyTab';
import { SmartBuyTab } from './-components/SmartBuyTab';
import { LIST_PROJECT_MARKET, useListProjectMarket, type ProjectMarketEntry } from '@starfoundry/components/services/projects/listMarket';
import { useListProjectMarketStructures } from '@starfoundry/components/services/projects/listMarketStructure';
import { useListStructure } from '@starfoundry/components/services/structure/list';
import { useState } from 'react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import type { Uuid } from '@starfoundry/components/services/utils';
import { deleteMarketEntry } from '@starfoundry/components/services/projects/deleteMarket';
import { useDisclosure } from '@mantine/hooks';
import { EditMarketEntryModal } from './-components/EditMarketEntryModal';
import { updateMarketEntry } from '@starfoundry/components/services/projects/updateMarketEntry';

export const Route = createFileRoute('/projects_/$projectId/market')({
    component: RouteComponent,
});

function RouteComponent() {
    const queryClient = useQueryClient();
    const { projectId } = Route.useParams();

    const [updateEntry, setUpdateEntry] = useState<ProjectMarketEntry | undefined>(undefined);

    const [updateEntryModalOpened, {
        open: openUpdateEntryModal,
        close: closeUpdateEntryModal,
    }] = useDisclosure(false);

    const [updateError, setUpdateError] = useState<boolean>(false);
    const [updateSuccess, setUpdateSuccess] = useState<boolean>(false);
    const [deleteError, setDeleteError] = useState<boolean>(false);
    const [deleteSuccess, setDeleteSuccess] = useState<boolean>(false);

    const {
        isPending: isPendingDefaultMarkets,
        isError: isErrorDefaultMarkets,
        data: defaultMarkets
    } = useListProjectMarketStructures(projectId);

    const {
        isPending: isPendingMarkets,
        isError: isErrorMarkets,
        data: markets,
    } = useListStructure({
        service_id: 35892,
        include_npc: true,
    });

    const {
        isError,
        isPending,
        data: projectMarket,
    } = useListProjectMarket(projectId);

    const deleteEntry = useMutation({
        mutationFn: async (data: Uuid) => {
            return await deleteMarketEntry(
                projectId,
                data,
            );
        },
        onSuccess: () => {
            setDeleteError(false);
            setDeleteSuccess(true);
            queryClient.invalidateQueries({
                queryKey: [LIST_PROJECT_MARKET, projectId]
            });
        },
        onError: () => {
            setDeleteError(true);
            setDeleteSuccess(false);
        }
    });

    const updateEntryMutation = useMutation({
        mutationFn: async (update: ProjectMarketEntry) => {
            return await updateMarketEntry(
                projectId,
                update.id,
                {
                    quantity: update.quantity,
                    cost: update.cost,
                    source: update.source,
                }
            );
        },
        onMutate: async (updatedEntry, context) => {
            await context
                .client
                .cancelQueries({ queryKey: [LIST_PROJECT_MARKET, projectId]});
            const previous = context
                .client
                .getQueryData([LIST_PROJECT_MARKET, projectId]);

            context
                .client
                .setQueryData(
                    [LIST_PROJECT_MARKET, projectId],
                    (old: ProjectMarketEntry[]) => {
                        const updated = old
                            .map(x => {
                                if (x.id === updatedEntry.id) {
                                    return updatedEntry;
                                } else {
                                    return x;
                                }
                            });
                        return updated;
                    }
                );
            return { previous }
        },
        onError: (_err, _updateEntry, onMutationResult, context) => {
            context.client.setQueryData([LIST_PROJECT_MARKET, projectId], onMutationResult?.previous);
            setUpdateSuccess(false);
            setUpdateError(true);
        },
        onSuccess: () => {
            setUpdateSuccess(true);
            setUpdateError(false);
        },
        onSettled: () => {
            queryClient.invalidateQueries({ queryKey: [LIST_PROJECT_MARKET, projectId] });
        }
    })

    if (isPending || isPendingMarkets || isPendingDefaultMarkets) {
        return LoadingAnimation();
    }

    if (isError || isErrorMarkets || isErrorDefaultMarkets) {
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
        } else if (deleteError) {
            return <Alert
                variant='light'
                color='red'
                data-cy="updateError"
                onClose={ () => setUpdateError(false) }
                withCloseButton
            >
                Error while deleting market entry
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
        } else if (deleteSuccess) {
            return <Alert
                variant='light'
                color='green'
                data-cy="updateSuccessful"
                onClose={ () => setDeleteSuccess(false) }
                withCloseButton
            >
                Entry was successful removed
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
                <Table.Td w={400}>
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
                <Table.Td w={200}>
                    <Group>
                        <Button
                            color="blue.9"
                            variant="subtle"
                            onClick={() => {
                                openUpdateEntryModal();
                                setUpdateEntry(x);
                            }}
                        >
                            Edit
                        </Button>

                        <Button
                            color="red.9"
                            variant="subtle"
                            onClick={() => deleteEntry.mutate(x.id)}
                        >
                            Delete
                        </Button>
                    </Group>
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
                        <Table.Th></Table.Th>
                    </Table.Tr>
                </Table.Thead>

                <Table.Tbody>
                    {rows}
                </Table.Tbody>
            </Table>
        </>;
    }

    return <>
        {
            updateEntry
            ?   <EditMarketEntryModal
                    entry={updateEntry || {} as ProjectMarketEntry}

                    onSave={(entry: ProjectMarketEntry) => {
                        updateEntryMutation.mutate(entry);
                        setUpdateEntry(undefined);
                    }}

                    opened={updateEntryModalOpened}
                    close={closeUpdateEntryModal}
                />
            :   <></>
        }

        {showError()}
        {showUpdateSuccess()}

        <Tabs
            defaultValue="overview"
            keepMounted={false}
        >
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
                <MultiBuyTab
                    projectId={projectId}

                    markets={markets}
                    defaultMarkets={defaultMarkets}
                />
            </Tabs.Panel>
            <Tabs.Panel value="smartBuy">
                <SmartBuyTab
                    projectId={projectId}

                    markets={markets}
                    defaultMarkets={defaultMarkets}
                />
            </Tabs.Panel>
        </Tabs>
    </>
}
