import { Button, Center, Group, Select, Stack, Textarea } from '@mantine/core'
import { useMediaQuery } from '@mantine/hooks';
import { useState, type ReactNode } from 'react';
import type { AppraisalMode, CreateAppraisal } from '@/services/appraisal/create';

export function AppraisalInput({
    fullSize = false,

    onCreate,
}: AppraisalInputProps): ReactNode {
    const isMobile = useMediaQuery('(max-width: 50em)');

    const [market, setMarket] = useState<string>('60003760');
    const [mode, setMode] = useState<AppraisalMode>('APPRAISAL');
    const [appraisalStr, setAppraisalStr] = useState<string>('Tritanium 100');

    const MARKETS = [
        { value: '60003760', label: 'Jita 4-4' },
        { value: '60008494', label: 'Amarr' },
        { value: '1046664001931', label: 'UALX-3' },
        { value: '1049588174021', label: 'C-J6MT' },
    ];
    const MODE = [
        { value: 'APPRAISAL', label: 'Appraisal' },
        { value: 'MULTIBUY', label: 'Multibuy' },
    ];

    return <>
        <Center>
            <Stack
                w={(isMobile || fullSize) ? '100%' : '50%'}
            >
                <Textarea
                    placeholder='Insert the items that should be appraised.

Copy and paste it from one of the following locations:
Inventory, Fits

Format:
Tritanium x1000
Isogen x1000'
                    value={appraisalStr}
                    onChange={(x) => setAppraisalStr(x.currentTarget.value)}
                    rows={15}
                />

                <Group
                    grow
                >
                    <Select
                        data={MARKETS}
                        value={market}
                        onChange={(x) => {
                            if (x) {
                                setMarket(x);
                            }
                        }}
                    />

                    <Select
                        data={MODE}
                        value={mode}
                        onChange={(x) => {
                            if (x) {
                                setMode(x as AppraisalMode);
                            }
                        }}
                    />

                    <Button
                        onClick={() => {
                            onCreate({
                                item_str:   appraisalStr,
                                market_id:  parseInt(market),
                                mode:       mode,
                            });
                        }}
                    >
                        Create
                    </Button>
                </Group>
            </Stack>
        </Center>
    </>
}

export type AppraisalInputProps = {
    fullSize?: boolean;

    onCreate: (info: CreateAppraisal) => void;
}
