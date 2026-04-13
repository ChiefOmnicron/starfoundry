import { CompressionMinimal, type GasDecompression, type MineralCompression } from "@starfoundry/components/misc/CompressionMinimal";
import { ModalWrapper } from "@starfoundry/components/wrapper/Modal";
import { Stack, Title } from "@mantine/core";
import { StructureList } from "@starfoundry/components/list/StructureList";
import type { ReactElement } from "react";
import type { Structure } from "@starfoundry/components/services/structure/list";

export function SmartBuySettingsModal({
    opened,
    close,

    gasDecompression,
    mineralCompression,

    markets,
    selectedMarkets,
    onMarketUpdate,

    onGasUpdate,
    onMineralUpdate,
}: SmartBuySettingsModalProp): ReactElement {
    return <>
        <ModalWrapper
            opened={opened}
            close={close}

            title="SmartBuy Settings"
        >
            <Stack>
                <CompressionMinimal
                    gasDecompression={gasDecompression}
                    mineralCompression={mineralCompression}

                    onGasUpdate={onGasUpdate}
                    onMineralUpdate={onMineralUpdate}
                />

                <Title order={2}>Markets</Title>

                <StructureList
                    structures={markets}
                    groupBySystem={false}

                    structureCardProps={{
                        checkable: true,
                        checked: selectedMarkets,
                        onChange: (event: 'checked' | 'unchecked', structure: Structure) => {
                            onMarketUpdate(
                                event === 'checked'
                                    ? [...selectedMarkets, structure]
                                    : selectedMarkets.filter((y) => y.id !== structure.id)
                            );
                        }
                    }}
                />
            </Stack>
        </ModalWrapper>
    </>
}

export type SmartBuySettingsModalProp = {
    opened: boolean;
    close: () => void;

    gasDecompression: GasDecompression;
    mineralCompression: MineralCompression;

    markets: Structure[];
    selectedMarkets: Structure[];
    onMarketUpdate(structures: Structure[]): void;

    onGasUpdate(value: GasDecompression): void;
    onMineralUpdate(value: MineralCompression): void;
}
