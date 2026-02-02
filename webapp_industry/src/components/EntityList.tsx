import { Card, Checkbox, Group, Pill, SimpleGrid, Stack, Text } from '@mantine/core';
import { CopyText } from '@/components/CopyText';
import { EveIcon } from './EveIcon';
import { useEffect, useState } from 'react';

export function CharacterCorporationAllianceList({
    characterCorporationAlliances,

    characterCorporationAllianceCardProps,
}: CharacterCorporationAllianceListProps) {
    return <>
        <SimpleGrid
            cols={{
                base: 1,
                sm: 6,
            }}
        >
            {
                characterCorporationAlliances
                    .map(x => <CharacterCorporationAllianceCard
                            characterCorporationAlliance={x}
                            {...characterCorporationAllianceCardProps}
                        />
                    )
            }
        </SimpleGrid>
    </>;
}

export function CharacterCorporationAllianceCard({
    characterCorporationAlliance,

    checkable = false,
    checked = [],
    onChange = () => {},
}: CharacterCorporationAllianceCardProps) {
    const [isSelected, setIsSelected] = useState<boolean>(false);

    useEffect(() => {
        setIsSelected(!!checked.find(x => x.id === characterCorporationAlliance.id as any));
    }, [checked]);

    const selectEntity = (
        state: boolean,
    ) => {
        if (!checkable) {
            return;
        }

        setIsSelected(state);

        if (state) {
            onChange('checked', characterCorporationAlliance);
        } else {
            onChange('unchecked', characterCorporationAlliance);
        }
    }

    const checkbox = () => {
        if (!checkable) {
            return <></>
        }

        return <>
            <Checkbox
                checked={isSelected}
                size='xs'
                onChange={(event) => {
                    selectEntity(event.currentTarget.checked);
                }}
            />
        </>
    }

    const card = () => {
        let category: 'alliances' | 'corporations' | 'characters' | 'types';
        switch (characterCorporationAlliance.category) {
            case 'alliance':
                category = 'alliances';
                break;
            case 'corporation':
                category = 'corporations';
                break;
            case 'character':
                category = 'characters';
                break;
            default:
                category = 'types';
        }

        let variation: 'portrait' | 'logo' = characterCorporationAlliance.category === 'character'
            ? 'portrait'
            : 'logo';

        return <>
            <Stack
                gap="xs"
                justify='space-between'
                style={{
                    width: '100%'
                }}
            >
                <Group
                    justify='center'
                >
                    <EveIcon
                        id={characterCorporationAlliance.id}
                        category={category}
                        type={variation}
                    />

                    <Text>
                        <CopyText
                            value={characterCorporationAlliance.name}
                        />
                    </Text>

                    <Pill>
                        {characterCorporationAlliance.category}
                    </Pill>

                    { checkbox() }
                </Group>
            </Stack>
        </>
    }

    const actionBar = () => {
        /*const edit = editLink
            ?   <InternalLink
                    to={StructureEditRoute.to}
                    params={{
                        structureId: structure.id,
                    } as any}
                    target={viewTarget}
                    content='Edit'
                />
            :   <></>
        const view = viewLink
            ?   <UnstyledButton
                    onClick={ openView }
                    style={{
                        color: 'var(--mantine-color-blue-4)',
                        fontSize: 'var(--mantine-font-size-sm)',
                    }}
                >
                    View
                </UnstyledButton>
            :   <></>

        if (editLink || viewLink) {
            return <>
                <Flex
                    align='flex-end'
                    justify='flex-end'
                    style={{
                        backgroundColor: 'rgba(93,93,104, 0.1)',
                        padding: '5px',
                    }}
                >
                    <Group>
                        { edit }

                        { view }
                    </Group>
                </Flex>
            </>
        } else {*/
            return <></>
        //}
    }

    return <>
        <Card
            key={characterCorporationAlliance.id}
            style={{
                padding: 0,
                border: isSelected ? '1px solid var(--mantine-color-blue-9)' : '',
            }}
        >
            <Card.Section
                onClick={ () => {
                    selectEntity(!isSelected);
                }}
                style={{
                    margin: '10px',
                    height: '100%'
                }}
            >
                { card() }
            </Card.Section>

            { actionBar() }
        </Card>
    </>
}

export type CharacterCorporationAlliance = {
    id:       number,
    category: 'alliance' | 'corporation' | 'character',
    name:     string,
}

export type CharacterCorporationAllianceListProps = {
    characterCorporationAlliances: CharacterCorporationAlliance[];

    characterCorporationAllianceCardProps?: CharacterCorporationAllianceCardAdditionalProps;
}

type CharacterCorporationAllianceCardMandatoryProps = {
    characterCorporationAlliance: CharacterCorporationAlliance;

    viewTarget?: '_blank' | '_self'
}

type CharacterCorporationAllianceCardAdditionalProps = {
    editLink?: boolean;
    viewLink?: boolean;

    // Determines if a checkbox is added or not
    checkable?: boolean,
    checked?: CharacterCorporationAlliance[];
    onChange?: (event: 'checked' | 'unchecked', entity: CharacterCorporationAlliance) => void;
}

export type CharacterCorporationAllianceCardProps = CharacterCorporationAllianceCardMandatoryProps & CharacterCorporationAllianceCardAdditionalProps;
