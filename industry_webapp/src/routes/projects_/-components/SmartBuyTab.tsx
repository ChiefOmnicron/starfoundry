import { Button, Center, Group, Stack, Title } from '@mantine/core';
import { DEFAULT_GAS_BONUS, DEFAULT_MINERAL_BONUS, type GasDecompression, type MineralCompression } from '@starfoundry/components/misc/CompressionMinimal';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { MarketBuy } from './MarketBuy';
import { SmartBuySettingsModal } from '@/routes/projects_/-components/SmartBuySettingsModal';
import { useDisclosure } from '@mantine/hooks';
import { useEffect, useState } from 'react';
import { useListProjectMarketBuy } from '@starfoundry/components/services/projects/listMarketBuy';
import { type Structure } from '@starfoundry/components/services/structure/list';
import type { Uuid } from '@starfoundry/components/services/utils';

export function SmartBuyTab({
    projectId,

    markets,
    defaultMarkets,
}: SmartBuyTabProps) {
    const [selectedMarkets, setSelectedMarkets] = useState<Structure[]>([]);

    const [gasBonus, setGasBonus] = useState<GasDecompression>(DEFAULT_GAS_BONUS);
    const [mineralBonus, setMineralBonus] = useState<MineralCompression>(DEFAULT_MINERAL_BONUS);

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
        strategy: 'SMART_BUY',
        structure_ids: selectedMarkets.map(x => x.structure_id),
        gas_decompression: gasBonus,
        mineral_compression: mineralBonus,
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

                gasDecompression={gasBonus}
                mineralCompression={mineralBonus}

                structures={selectedMarkets}
            />
        </>
    }

    return <>
        <Stack>
            <SmartBuySettingsModal
                close={closeSettingsModal}
                opened={settingsModalOpened}

                gasDecompression={gasBonus}
                mineralCompression={mineralBonus}

                markets={markets}
                selectedMarkets={selectedMarkets}
                onMarketUpdate={setSelectedMarkets}

                onGasUpdate={setGasBonus}
                onMineralUpdate={setMineralBonus}
            />

            {showTable()}
        </Stack>
    </>
}

export type SmartBuyTabProps = {
    projectId: Uuid,

    markets:        Structure[];
    defaultMarkets: Structure[];
}
