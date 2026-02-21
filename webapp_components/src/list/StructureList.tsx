import { SimpleGrid, Title } from '@mantine/core';
import { StructureCard, type StructureCardAdditionalProps } from '@internal/cards/StructureCard';
import type { Structure, System } from '@internal/services/structure/list';

export function StructureList({
    structures,

    viewTarget = '_self',
    groupBySystem = true,

    structureCardProps,
}: StructureListProps) {
    const systems: System[] = [];
    structures
        .map(x => {
            if (!systems.find(y => y.system_id === x.system.system_id)) {
                systems.push(x.system);
            }
        });

    const structureCardBySystem = (systemId: number) => {
        return structures
            .filter(x => x.system.system_id == systemId)
            .map(x => <StructureCard
                    key={x.id}
                    structure={x}
                    viewTarget={viewTarget}
                    {...structureCardProps}
                />
            );
    }

    if (groupBySystem) {
        return systems
            .map(x => {
                return <>
                    <Title
                        order={2}
                        mt='xs'
                    >
                        { x.system_name }
                    </Title>

                    <SimpleGrid
                        cols={{
                            base: 1,
                            sm: 2,
                        }}
                    >
                        { structureCardBySystem(x.system_id) }
                    </SimpleGrid>
                </>
            });
    } else {
        return <>
            <SimpleGrid
                cols={{
                    base: 1,
                    sm: 2,
                }}
            >
                {
                    structures
                        .map(x => <StructureCard
                                structure={x}
                                viewTarget={viewTarget}
                                {...structureCardProps}
                            />
                        )
                }
            </SimpleGrid>
        </>;
    }
}

export type StructureListProps = {
    structures: Structure[];

    viewTarget?:    '_blank' | '_self';
    groupBySystem?: boolean;

    structureCardProps?: StructureCardAdditionalProps;
}
