import { ModalWrapper } from "@starfoundry/components/wrapper/Modal";
import { Stack, Title } from "@mantine/core";
import { StructureList } from "@starfoundry/components/list/StructureList";
import type { ReactElement } from "react";
import type { Structure } from "@starfoundry/components/services/structure/list";

export function MultiBuySettingsModal({
    opened,
    close,

    markets,
    selectedMarkets,
    onMarketUpdate,
}: MultiBuySettingsModalProp): ReactElement {
    return <>
        <ModalWrapper
            opened={opened}
            close={close}

            title="MultiBuy Settings"
        >
            <Stack>
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

export type MultiBuySettingsModalProp = {
    opened: boolean;
    close: () => void;

    markets: Structure[];
    selectedMarkets: Structure[];
    onMarketUpdate(structures: Structure[]): void;
}
