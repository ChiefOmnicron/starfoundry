import { Button, Divider, Group, NumberFormatter, Stack, Textarea, Tooltip } from "@mantine/core";
import { ModalWrapper } from "@starfoundry/components/wrapper/Modal";
import { useClipboard } from "@mantine/hooks";
import { useEffect, useState, type ReactElement } from "react";
import type { ProjectMarketBuyEntry } from "@starfoundry/components/services/projects/listMarketBuy";

export function MultiBuyModal({
    source,
    items,

    onSave,

    opened,
    close,
}: MultiBuyModalProps): ReactElement {
    const clipboard = useClipboard();
    const [tooltipOpened, setTooltipOpened] = useState<boolean>(false);

    const [fromMultibuy, setFromMultibuy] = useState<string>('');

    useEffect(() => {
        if (opened) {
            setTimeout(() => setTooltipOpened(false), 1000);
        }
    }, [tooltipOpened]);

    useEffect(() => {
        const joined = items
            .map(x => {
                return x
                    .entries
                    .flatMap(y => `${x.item.name}\t${y.quantity}\t${y.price}\t${(y.quantity * y.price).toFixed(2)}`)
                    .join(`\n`);
            })
            .join(`\n`);
        setFromMultibuy(joined);
    }, [items]);

    const forMultibuy = items
        .map(x => {
            return x
                .entries
                .flatMap(y => `${x.item.name}\t${y.quantity}`)
                .join(`\n`);
        })
        .join(`\n`);

    let expectedCost = items
        .flatMap(x => x.entries.map(y => y.quantity * y.price))
        .reduce((prev, curr) => prev += curr, 0)
        .toFixed(0);

    return <>
        <ModalWrapper
            opened={opened}
            close={close}
            title={source}
            size="50%"
        >
            <Stack>
                <Group>
                    Expected cost: <NumberFormatter suffix=" ISK" value={expectedCost} thousandSeparator />
                </Group>

                <Textarea
                    label="For Multibuy"
                    description="Paste this into multibuy"
                    value={forMultibuy}
                    disabled
                />

                <Group
                    justify="flex-end"
                >
                    <Tooltip
                        opened={tooltipOpened}
                        label="Copied!"
                        position="top"
                    >
                        <Button
                            onClick={() => {
                                clipboard.copy(forMultibuy);
                                setTooltipOpened(true);
                            }}
                        >
                            Copy
                        </Button>
                    </Tooltip>
                </Group>

                <Divider />

                <Textarea
                    label="From Multibuy"
                    description="Copy the multibuy data into here"
                    value={fromMultibuy}
                    onChange={(x) => setFromMultibuy(x.target.value)}
                />

                <Group
                    justify="flex-end"
                >
                    <Button
                        onClick={() => onSave(fromMultibuy.replace(/,/g, ''))}
                    >
                        Mark as bought
                    </Button>
                </Group>
            </Stack>
        </ModalWrapper>
    </>
}

export type MultiBuyModalProps = {
    source: string;
    items:  ProjectMarketBuyEntry[];

    onSave: (value: string) => void;

    opened: boolean;
    close: () => void;
}
