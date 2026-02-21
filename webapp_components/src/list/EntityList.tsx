import { EntityCard, type EntityAdditionalProps } from '@internal/cards/EntityCard';
import { SimpleGrid } from '@mantine/core';

export function EntityList({
    entities,

    entityCardProps,
}: EntityListProps) {
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


