import { Card, Flex, Group, Pill, SimpleGrid, Stack, Text, UnstyledButton } from '@mantine/core';
import { CopyText } from '@/components/CopyText';
import { EveIcon } from './EveIcon';

export function CharacterCorporationAllianceList({
    characterCorporationAlliances,

    characterCorporationAllianceCardProps,
}: CharacterCorporationAllianceListProps) {
    return <>
        <SimpleGrid
            cols={{
                base: 1,
                sm: 3,
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

    editable = false,
    onChange = () => {},
}: CharacterCorporationAllianceCardProps) {
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
                    justify='space-between'
                >
                    <Group>
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
                    </Group>
                </Group>

                <Group
                    gap={'xs'}
                >
                    <Text
                        size='sm'
                        fw={700}
                    >
                        Type:
                    </Text>
                    <Text
                        size='sm'
                    >
                        <Pill>
                            {characterCorporationAlliance.category}
                        </Pill>
                    </Text>
                </Group>
            </Stack>
        </>
    }

    const actionBar = () => {
        const edit = editable
            ?   <UnstyledButton
                    onClick={() => onChange('remove', characterCorporationAlliance)}
                    style={{
                        color: 'var(--mantine-color-red-4)',
                        fontSize: 'var(--mantine-font-size-sm)',
                    }}
                >
                    Remove
                </UnstyledButton>
            :   <></>

        if (editable) {
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
                    </Group>
                </Flex>
            </>
        } else {
            return <></>
        }
    }

    return <>
        <Card
            key={characterCorporationAlliance.id}
            style={{
                padding: 0,
            }}
        >
            <Card.Section
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
    editable?: boolean;

    onChange?: (event: 'remove', entity: CharacterCorporationAlliance) => void;
}

export type CharacterCorporationAllianceCardProps = CharacterCorporationAllianceCardMandatoryProps & CharacterCorporationAllianceCardAdditionalProps;
