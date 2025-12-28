import { Alert, Code, Stack } from '@mantine/core';
import { InternalLink } from '@/components/InternalLink';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { Route as StructureRoute } from '@/routes/structures/index';
import { StructureList } from '@/components/StructureList';
import { useEffect, useState } from 'react';
import { LIST_PROJECT_GROUP_DEFAULT_MARKETS, useListProjectGroupDefaultMarkets } from '@/services/project-group/listDefaultMarket';
import { useListStructure, type Structure } from '@/services/structure/list';
import type { Uuid } from '@/services/utils';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { updateDefaultMarket } from '@/services/project-group/updateDefaultMarket';
import { compareArray } from '@/components/SaveDialog';

export function ProjectGroupDefaultsMarket({
    projectGroupId,

    onTouchChange,
    triggerSave,
    triggerReset,
}: DefaultMarketProps) {
    const [touched, setTouched] = useState<boolean>(false);
    const [updateSuccess, setUpdateSuccess] = useState<boolean>(false);

    const queryClient = useQueryClient();
    const [selectedStructuresOld, setSelectedStructuresOld] = useState<Structure[]>([]);
    const [selectedStructures, setSelectedStructures] = useState<Structure[]>([]);

    const {
        isError,
        isPending,
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

    const update = useMutation({
        mutationFn: () => {
            if (!touched) {
                return Promise.resolve(null);
            }

            const structureIds = selectedStructures.map(x => x.id);
            return updateDefaultMarket(projectGroupId, structureIds);
        },
        onSuccess: () => {
            if (!touched) {
                return;
            }

            queryClient.invalidateQueries({ queryKey: [LIST_PROJECT_GROUP_DEFAULT_MARKETS] });
            setUpdateSuccess(true);
        },
    });

    useEffect(() => {
        if (defaultMarket) {
            setSelectedStructuresOld(defaultMarket);
            setSelectedStructures(defaultMarket);
        }
    }, [defaultMarket]);

    useEffect(() => {
        const a = selectedStructuresOld.map(x => x.structure_id);
        const b = selectedStructures.map(x => x.structure_id);
        onTouchChange(!compareArray(a, b));
        setTouched(!compareArray(a, b));
    }, [selectedStructures]);

    useEffect(() => {
        if (triggerSave === 0) {
            return;
        }

        update.mutate();
    }, [triggerSave]);

    useEffect(() => {
        if (triggerReset === 0) {
            return;
        }

        setSelectedStructures(selectedStructuresOld);
    }, [triggerReset]);

    if (isPending || isPendingStructures) {
        return LoadingAnimation();
    }

    if (isError || isErrorStructures) {
        return LoadingError();
    }

    const onDeleteStructure = (structureId: string) => {
        const removedStructure = selectedStructures
            .filter(x => x.id !== structureId);
        setSelectedStructures(removedStructure);
    }

    const onSelectStructure = (structure: Structure) => {
        setSelectedStructures([
            structure,
            ...selectedStructures,
        ]);
    }

    const notification = () => {
        if (updateSuccess) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Update successful'
                data-cy="updateSuccessful"
                onClose={ () => setUpdateSuccess(false) }
                withCloseButton
            >
                Updating the market structures was successful
            </Alert>
        }
    }

    return <>
        { notification() }

        <Stack>
            <Alert variant='light' color='gray'>
                Determines which markets should be used when comparing material prices.
                <br /><br />
                Amarr and Jita are per default available.
                <br />
                If you want to add your own market, add the structure under <InternalLink
                    to={StructureRoute.to}
                    target='_blank'
                    content='Structure Overview '
                /> and, add the <Code>Standup Market Hub I</Code> service to the structure.
                After that, you can refresh the selector, and your structure will show up.
            </Alert>

            <StructureList
                structures={selectedStructures}
                editable
                selectableStructures={structures}
                onDelete={onDeleteStructure}
                onSelect={onSelectStructure}
            />
        </Stack>
    </>
}

export type DefaultMarketProps = {
    projectGroupId: Uuid,

    onTouchChange(state: boolean): void,
    triggerSave:                   number;
    triggerReset:                  number;
}
