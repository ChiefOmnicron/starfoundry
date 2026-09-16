import { Alert, Button, Center, Group, Select, Stack, Textarea } from '@mantine/core'
import { MARKETS } from '@/services/utils';
import { useMediaQuery } from '@mantine/hooks';
import { useEffect, useState, type ReactNode } from 'react';
import type { AppraisalMode } from '@starfoundry/components/services/appraisal/fetch';
import type { CreateAppraisal } from '@starfoundry/components/services/appraisal/create';

export function AppraisalInput({
    fullSize = false,
    showCreateError = false,

    content,

    onCreate,
}: AppraisalInputProps): ReactNode {
    const isMobile = useMediaQuery('(max-width: 50em)');

    const [market, setMarket] = useState<string>('60003760');
    const [mode, setMode] = useState<AppraisalMode>('APPRAISAL');
    const [appraisalStr, setAppraisalStr] = useState<string>('Tritanium 100');

    useEffect(() => {
        if (content) {
            setAppraisalStr(content);
        }
    }, [content]);

    const MODE = [
        { value: 'APPRAISAL', label: 'Appraisal' },
        { value: 'MULTIBUY', label: 'Multibuy' },
    ];

    const notification = () => {
        if (showCreateError) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Error while creating appraisal'
                data-cy="error"
            >
                There was an error while creating the appraisal. Please try again later.
            </Alert>
        }
    }

    return <>
        <Center>
            {notification()}

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
    showCreateError?: boolean;

    content?: string,

    onCreate: (info: CreateAppraisal) => void;
}
