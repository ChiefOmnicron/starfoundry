import { Alert, Stack } from '@mantine/core';
import { compareArray } from '@/components/SaveDialog';
import { ItemList } from '@/components/ItemList';
import { LIST_PROJECT_GROUP_DEFAULT_BLACKLIST, useListProjectGroupDefaultBlacklist } from '@/services/project-group/listDefaultBlacklist';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { updateDefaultBlacklist } from '@/services/project-group/updateDefaultBlacklist';
import { useEffect, useState } from 'react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import type { Item } from '@/services/item/model';
import type { Uuid } from '@/services/utils';

export function ProjectGroupDefaultsBlacklist({
    projectGroupId,

    onTouchChange,
    triggerSave,
    triggerReset,
}: DefaultMarketProps) {
    const queryClient = useQueryClient();

    const [touched, setTouched] = useState<boolean>(false);
    const [updateSuccess, setUpdateSuccess] = useState<boolean>(false);

    const [selectedBlacklistOld, setSelectedBlacklistOld] = useState<Item[]>([]);
    const [selectedBlacklist, setSelectedBlacklist] = useState<Item[]>([]);

    const {
        isError,
        isPending,
        data: defaultBlacklist,
    } = useListProjectGroupDefaultBlacklist(projectGroupId);

    const update = useMutation({
        mutationFn: () => {
            if (!touched) {
                return Promise.resolve(null);
            }

            const typeIds = selectedBlacklist.map(x => x.type_id);
            return updateDefaultBlacklist(projectGroupId, typeIds);
        },
        onSuccess: () => {
            if (!touched) {
                return;
            }

            queryClient.invalidateQueries({ queryKey: [LIST_PROJECT_GROUP_DEFAULT_BLACKLIST] });
            setUpdateSuccess(true);
        },
    });

    useEffect(() => {
        if (defaultBlacklist) {
            setSelectedBlacklistOld(defaultBlacklist);
            setSelectedBlacklist(defaultBlacklist);
        }
    }, [defaultBlacklist]);

    useEffect(() => {
        const a = selectedBlacklistOld.map(x => x.type_id);
        const b = selectedBlacklist.map(x => x.type_id);
        onTouchChange(!compareArray(a, b));
        setTouched(!compareArray(a, b));
    }, [selectedBlacklist]);

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

        setSelectedBlacklist(selectedBlacklistOld);
    }, [triggerReset]);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const onSelectItems = (items: Item[]) => {
        setSelectedBlacklist(items);
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
                Updating the blacklist was successful
            </Alert>
        }
    }

    return <>
        { notification() }

        <Stack>
            <Alert variant='light' color='gray'>
                Determines which items should not be build.
            </Alert>

            <ItemList
                onSelect={onSelectItems}
                selected={selectedBlacklist}
                buildable
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
