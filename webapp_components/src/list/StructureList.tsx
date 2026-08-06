import { SimpleGrid, Title } from '@mantine/core';
import { StructureCard } from '@internal/cards/StructureCard';
import type { Structure, System } from '@internal/services/structure/list';
import { useEffect, useState } from 'react';
import type { Uuid } from '@internal/services/utils';

export function StructureList({
    structures,

    filter,
    multiple = false,

    groupBySystem = true,

    onSelect = undefined,
    selectedStructures,

    onEditClick = () => {},
}: StructureListProps) {
    const [selectedStructuresInternal, setSelectedStructuresInternal] = useState<Structure[]>([]);

    useEffect(() => {
        setSelectedStructuresInternal(selectedStructures || []);
    }, [selectedStructures]);

    const systems: System[] = [];
    structures
        .map(x => {
            if (!systems.find(y => y.system_id === x.system.system_id)) {
                systems.push(x.system);
            }
        });

    const structureCards = (
        filter: StructureListFilter = {},
    ) => {
        return structures
            .filter(x => {
                if (filter && filter.search) {
                    return x.name
                            .toLowerCase()
                            .includes(filter.search.toLowerCase().trim()) ||
                        x.system
                            .system_name
                            .toLowerCase()
                            .includes(filter.search.toLowerCase().trim());
                }

                return true;
            })
            .filter(x => {
                if (filter && filter.systemId) {
                    return x.system.system_id === filter.systemId;
                }

                return true;
            })
            .map(x => <StructureCard
                    key={x.id}
                    structure={x}
                    checked={!!selectedStructuresInternal.find(y => y.id === x.id)}
                    checkable={!!onSelect}
                    onEditClick={() => onEditClick(x.id)}
                    onChange={(event: 'checked' | 'unchecked', structure: Structure) => {
                        if (!onSelect) {
                            return;
                        }

                        const update = event === 'checked'
                            ? multiple
                                ?   [...selectedStructuresInternal, structure]
                                :   [structure]
                            : selectedStructuresInternal.filter((y) => y.id !== structure.id);

                        setSelectedStructuresInternal(update);
                        onSelect(update);
                    }}
                />
            );
    }

    const structureCardBySystem = (systemId: number) => {
        return structureCards({
            systemId: systemId,
            ...filter,
        });
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
                { structureCards(filter) }
            </SimpleGrid>
        </>;
    }
}

export type StructureListProps = {
    structures: Structure[];

    filter?: StructureListFilter;
    multiple?: boolean;

    groupBySystem?: boolean;

    selectedStructures?: Structure[];
    onSelect?: (structures: Structure[]) => void;

    onEditClick?: (structureId: Uuid) => void;
}

export type StructureListFilter = {
    search?: string;
    systemId?: number;
}
