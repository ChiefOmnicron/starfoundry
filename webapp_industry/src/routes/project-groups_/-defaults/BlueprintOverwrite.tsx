import { Alert, Stack } from '@mantine/core';
import { BlueprintOverwriteList } from '@/routes/project-groups_/-components/BlueprintOverwriteLIst';
import { LIST_PROJECT_GROUP_DEFAULT_BLUEPRINT_OVERWRITES, useListProjectGroupDefaultBlueprintOverwrites, type BlueprintOverwrite } from '@/services/project-group/listDefaultBlueprintOverwrites';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { updateDefaultBlueprintOverwrite } from '@/services/project-group/updateDefaultBlueprintOverwrite';
import { useEffect, useState } from 'react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import type { Uuid } from '@/services/utils';
import { compareArray } from '@/components/SaveDialog';

export function ProjectGroupDefaultsBlueprintOverwrite({
    projectGroupId,

    onTouchChange,
    triggerSave,
    triggerReset,
}: DefaultMarketProps) {
    const [touched, setTouched] = useState<boolean>(false);
    const [updateSuccess, setUpdateSuccess] = useState<boolean>(false);

    const queryClient = useQueryClient();
    const [selectedBlueprintOverwriteOld, setSelectedBlueprintOverwriteOld] = useState<BlueprintOverwrite[]>([]);
    const [selectedBlueprintOverwrite, setSelectedBlueprintOverwrite] = useState<BlueprintOverwrite[]>([]);

    const {
        isError,
        isPending,
        data: defaultBlueprintOverwrites,
    } = useListProjectGroupDefaultBlueprintOverwrites(projectGroupId);

    const update = useMutation({
        mutationFn: () => {
            if (!touched) {
                return Promise.resolve(null);
            }

            const entries = selectedBlueprintOverwrite
                .map(x => {
                    return {
                        type_id:             x.item.type_id,
                        material_efficiency: x.material_efficiency,
                    }
                })

            return updateDefaultBlueprintOverwrite(projectGroupId, entries);
        },
        onSuccess: () => {
            if (!touched) {
                return;
            }

            queryClient.invalidateQueries({ queryKey: [LIST_PROJECT_GROUP_DEFAULT_BLUEPRINT_OVERWRITES] });
            setUpdateSuccess(true);
        },
    });

    useEffect(() => {
        if (defaultBlueprintOverwrites) {
            setSelectedBlueprintOverwriteOld(defaultBlueprintOverwrites);
            setSelectedBlueprintOverwrite(defaultBlueprintOverwrites);
        }
    }, [defaultBlueprintOverwrites]);

    useEffect(() => {
        const a = selectedBlueprintOverwriteOld.map(x => x.item.type_id);
        const b = selectedBlueprintOverwrite.map(x => x.item.type_id);
        onTouchChange(!compareArray(a, b));
        setTouched(!compareArray(a, b));
    }, [selectedBlueprintOverwrite]);

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

        setSelectedBlueprintOverwrite(selectedBlueprintOverwriteOld);
    }, [triggerReset]);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const onDeleteItem = (typeId: number) => {
        const removedStructure = selectedBlueprintOverwrite
            .filter(x => x.item.type_id !== typeId);
        setSelectedBlueprintOverwrite(removedStructure)
    }

    const onSelectItem = (blueprintOverwrite: BlueprintOverwrite) => {
        setSelectedBlueprintOverwrite([
            blueprintOverwrite,
            ...selectedBlueprintOverwrite,
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
                Updating the blueprint overwrites was successful
            </Alert>
        }
    }

    return <>
        { notification() }

        <Stack>
            <Alert variant='light' color='gray'>
                Per default all sub blueprints will be calculated using a material efficiency if 10.<br />
                For some blueprints this is not necessary wanted, for example for Freighters when building a Jump Freighter.
                Having the Freighter with ME 8 is more realistic than ME 10.<br />
                <br />
                In the calculation the blueprints below will be configured to use the given ME instead of the default 10.
            </Alert>

            <BlueprintOverwriteList
                onSelect={onSelectItem}
                onDelete={onDeleteItem}
                selected={selectedBlueprintOverwrite}
                editable
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
