import { Alert, Button, Group, Select, Stack } from "@mantine/core";
import { createFitting, type EveFit, type EveFitItem } from "@starfoundry/components/services/fittings/create";
import { LoadingAnimation } from "@starfoundry/components/misc/LoadingAnimation";
import { LoadingError } from "@starfoundry/components/misc/LoadingError";
import { ModalWrapper } from "@starfoundry/components/wrapper/Modal";
import { useEffect, useState, type ReactElement } from "react";
import { useListCharacters } from "@starfoundry/components/services/character/list";
import { useMutation } from "@tanstack/react-query";
import type { ProjectMarketEntry } from "@starfoundry/components/services/projects/listMarket";
import type { ProjectStock } from "@starfoundry/components/services/projects/fetch";

const SHIPS = [{
    label: 'Charon',
    value: '20185',
}, {
    label: 'Providence',
    value: '20183',
}, {
    label: 'Fenrir',
    value: '20189',
}, {
    label: 'Obelisk',
    value: '20187',
}];

export function FittingModal({
    entries,
    fitName,

    opened,
    close,
}: FittingModalProps): ReactElement {
    const [selectedCharacter, setSelectedCharacter] = useState<string | null>(null);
    const [selectedShip, setSelectedShip] = useState<string | null>('20185');

    const [successfulCreate, setSuccessfulCreate] = useState<boolean>();
    const [errorCreate, setErrorCreate] = useState<string | undefined>();

    const {
        isPending,
        isError,
        data: characters,
    } = useListCharacters();

    useEffect(() => {
        if (characters) {
            setSelectedCharacter(characters[0].character_id.toString());
        }
    }, [characters]);

    const createFitMutation = useMutation({
        mutationFn: (data: EveFit) => createFitting(Number.parseInt(selectedCharacter as string), data),
        onSuccess: () => {
            setErrorCreate(undefined);
            setSuccessfulCreate(true);
        },
        onError: (error) => {
            setErrorCreate(error.message);
            setSuccessfulCreate(false);
        },
    });

    const content = () => {
        if (isPending || isError) {
            return;
        }

        const selectableCharacters = characters
            .filter(x => x.scopes.indexOf('esi-fittings.write_fittings.v1') > -1)
            .map(x => {
                return {
                    label: x.character_name,
                    value: x.character_id.toString(),
                }
            });

        const fit = {
            name: fitName,
            description: 'Automatically created',
            items: [
                ...(entries || [])
                    .map(x => {
                        return {
                            quantity: x.quantity,
                            type_id: x.item.type_id,
                            flag: 'Cargo',
                        } as EveFitItem
                    }),
                {
                    quantity: 1,
                    type_id: 1319,
                    flag: 'LoSlot0',
                } as EveFitItem,
                {
                    quantity: 1,
                    type_id: 1319,
                    flag: 'LoSlot1',
                } as EveFitItem,
                {
                    quantity: 1,
                    type_id: 1319,
                    flag: 'LoSlot2',
                } as EveFitItem,
            ],
            ship_type_id: Number.parseInt(selectedShip as string),
        };

        return <>
            <Stack>
                <Select
                    label="Character"
                    description="Select the character that should receive the fit"
                    data={selectableCharacters}
                    value={selectedCharacter ? selectedCharacter : null}
                    onChange={(value) => setSelectedCharacter(value)}
                    searchable
                />

                <Select
                    label="Ship"
                    description="Select the ship that should be used for creating the fit"
                    data={SHIPS}
                    value={selectedShip ? selectedShip : null}
                    onChange={(value) => setSelectedShip(value)}
                    searchable
                />

                <Group
                    justify="flex-end"
                >
                    <Button
                        onClick={() => createFitMutation.mutate(fit)}
                    >
                        Create
                    </Button>
                </Group>
            </Stack>
        </>
    }
    
    const notification = () => {
        if (successfulCreate) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Success'
                data-cy="successfulUpdate"
                onClose={ () => setSuccessfulCreate(false) }
                withCloseButton
            >
                The fit was created
            </Alert>;
        } else if (errorCreate) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Update error'
                data-cy="errorUpdate"
                onClose={ () => setErrorCreate(undefined) }
                withCloseButton
            >
                There was an error while creating the fit
            </Alert>;
        }
    };

    return <>
        <ModalWrapper
            opened={opened}
            close={close}
            title="Create fit"
            size="50%"
        >
            {
                isPending
                ?   LoadingAnimation()
                :   <></>
            }
            {
                isError
                ?   LoadingError()
                :   <></>
            }

            {notification()}

            {content()}
        </ModalWrapper>
    </>
}

export type FittingModalProps = {
    entries: ProjectStock[] | ProjectMarketEntry[];
    fitName: string;

    opened: boolean;
    close: () => void;
}
