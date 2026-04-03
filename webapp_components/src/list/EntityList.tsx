import { EntityCard, type EntityAdditionalProps } from '@internal/cards/EntityCard';
import { Center, SimpleGrid, Stack, Title } from '@mantine/core';

export function EntityList({
    entities,

    entityCardProps,
}: EntityListProps) {
    if (entities.length === 0) {
        return <Center mt={50} data-cy="noData">
            <Stack>
                <Title order={4}>No members</Title>
            </Stack>
        </Center>
    }

    return <>
        <SimpleGrid
            cols={{
                base: 1,
                sm: 3,
            }}
        >
            {
                entities
                    .map(x => <EntityCard
                            entity={x}
                            {...entityCardProps}
                        />
                    )
            }
        </SimpleGrid>
    </>;
}

export type Entity = {
    id:       number,
    category: 'alliance' | 'corporation' | 'character',
    name:     string,
}

export type EntityListProps = {
    entities: Entity[];

    entityCardProps?: EntityAdditionalProps;
}


