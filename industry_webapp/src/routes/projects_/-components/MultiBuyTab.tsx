import { Button, Center, Group, Stack, Title } from '@mantine/core';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { MarketBuy } from './MarketBuy';
import { MultiBuySettingsModal } from './MultiBuySettingsModal';
import { useDisclosure } from '@mantine/hooks';
import { useEffect, useState } from 'react';
import { useListProjectMarketBuy } from '@starfoundry/components/services/projects/listMarketBuy';
import type { Structure } from '@starfoundry/components/services/structure/list';
import type { Uuid } from '@starfoundry/components/services/utils';

export function MultiBuyTab({
    projectId,

    markets,
    defaultMarkets,
}: MultiBuyTabProps) {
    const [selectedMarkets, setSelectedMarkets] = useState<Structure[]>([]);

    const [settingsModalOpened, {
        open: openSettingsModal,
        close: closeSettingsModal,
    }] = useDisclosure(false);

    const {
        isError,
        isPending,
        isFetching,
        data: marketData,
    } = useListProjectMarketBuy(projectId, {
        strategy: 'MULTI_BUY',
        structure_ids: selectedMarkets.map(x => x.structure_id),
    });

    useEffect(() => {
        if (defaultMarkets) {
            setSelectedMarkets(defaultMarkets);
        }
    }, [defaultMarkets]);

    const showTable = () => {
        if (isFetching || isPending) {
            return LoadingAnimation();
        }

        if (isError) {
            return LoadingError();
        }

        if (
            marketData.filter(x => x.quantity > 0).length === 0 &&
            !isFetching
        ) {
            return <Center mt={50} data-cy="noData">
                <Stack>
                    <Title order={4}>All materials bought</Title>
                </Stack>
            </Center>;
        }

        return <>
            <Group justify='flex-end'>
                <Button
                    onClick={openSettingsModal}
                >
                    Settings
                </Button>
            </Group>

            <MarketBuy
                marketData={marketData}
                projectId={projectId}

                structures={selectedMarkets}
            />
        </>
    }

    return <>
        <Stack>
            <MultiBuySettingsModal
                close={closeSettingsModal}
                opened={settingsModalOpened}

                markets={markets}
                selectedMarkets={selectedMarkets}
                onMarketUpdate={setSelectedMarkets}
            />

            {showTable()}
        </Stack>
    </>
}

export type MultiBuyTabProps = {
    projectId: Uuid,

    markets:        Structure[];
    defaultMarkets: Structure[];
}
