import { Card, Flex, Group, Pill, Stack, Text, UnstyledButton } from "@mantine/core";
import { CopyText } from "@internal/misc/CopyText";
import { EveIcon } from "@internal/misc/EveIcon";
import type { Entity } from "@internal/list/EntityList";

export function EntityCard({
    entity,

    editable = false,
    onChange = () => {},
}: EntityCardProps) {
    const card = () => {
        let category: 'alliances' | 'corporations' | 'characters' | 'types';
        switch (entity.category) {
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

        let variation: 'portrait' | 'logo' = entity.category === 'character'
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
                            id={entity.id}
                            category={category}
                            type={variation}
                        />

                        <Text>
                            <CopyText
                                value={entity.name}
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
                            {entity.category}
                        </Pill>
                    </Text>
                </Group>
            </Stack>
        </>
    }

    const actionBar = () => {
        const edit = editable
            ?   <UnstyledButton
                    onClick={() => onChange('remove', entity)}
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
            key={entity.id}
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

type EntityRequiredProps = {
    entity: Entity;

    viewTarget?: '_blank' | '_self'
}

export type EntityAdditionalProps = {
    editable?: boolean;

    onChange?: (event: 'remove', entity: Entity) => void;
}

export type EntityCardProps = EntityRequiredProps & EntityAdditionalProps;
