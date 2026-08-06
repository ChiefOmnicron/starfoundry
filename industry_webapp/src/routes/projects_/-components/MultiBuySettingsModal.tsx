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
                    multiple

                    selectedStructures={selectedMarkets}
                    onSelect={onMarketUpdate}
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
