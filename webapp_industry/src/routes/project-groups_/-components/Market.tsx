import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { useListProjectGroupDefaultMarkets } from '@/services/project-group/listDefaultMarket';
import { Title } from '@mantine/core';
import { useListStructure, type Structure } from '@/services/structure/list';
import { StructureList } from '@/components/StructureList';
import { useEffect, useState } from 'react';
import type { Uuid } from '@/services/utils';

export function ProjectGroupDefaultsMarket({
    projectGroupId,
}: DefaultMarketProps) {
    const [selectedStructures, setSelectedStructures] = useState<Structure[]>([]);

    const {
        isError: isErrorMarket,
        isPending: isPendingMarket,
        data: defaultMarket,
    } = useListProjectGroupDefaultMarkets(projectGroupId);

    const {
        isPending: isPendingStructures,
        isError: isErrorStructures,
        data: structures,
    } = useListStructure({
        service_id: 35892,
        include_npc: true,
    });

    useEffect(() => {
        if (defaultMarket) {
            setSelectedStructures(defaultMarket);
        }
    }, [defaultMarket]);

    if (isPendingMarket || isPendingStructures) {
        return LoadingAnimation();
    }

    if (isErrorMarket || isErrorStructures) {
        return LoadingError();
    }

    const onDeleteStructure = (structureId: string) => {
        const removedStructure = selectedStructures
            .filter(x => x.id !== structureId);
        setSelectedStructures(removedStructure)
    }

    const onSelectStructure = (structure: Structure) => {
        setSelectedStructures([
            structure,
            ...selectedStructures,
        ]);
    }

    return <>
        <Title order={2}>
            Market
        </Title>

        <StructureList
            structures={selectedStructures}
            selectableStructures={structures}
            onDelete={onDeleteStructure}
            onSelect={onSelectStructure}
        />
    </>
}

export type DefaultMarketProps = {
    projectGroupId: Uuid,
}
