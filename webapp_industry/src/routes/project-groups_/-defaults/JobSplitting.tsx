import { Alert, Stack } from '@mantine/core';
import { JobSplittingRunList } from '../-components/JobSplittingRunList';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { useEffect, useState } from 'react';
import { LIST_PROJECT_GROUP_DEFAULT_JOB_SPLITTING, useListProjectGroupDefaultJobSplitting, type JobSplittingRun } from '@/services/project-group/listDefaultJobSplitting';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import type { Uuid } from '@/services/utils';
import { updateDefaultJobSplitting } from '@/services/project-group/updateDefaultJobSplitting';
import { compareArray } from '@/components/SaveDialog';

export function ProjectGroupDefaultsJobSplitting({
    projectGroupId,

    onTouchChange,
    triggerSave,
    triggerReset,
}: defaultJobSplittingsProps) {
    const [touched, setTouched] = useState<boolean>(false);
    const [updateSuccess, setUpdateSuccess] = useState<boolean>(false);

    const queryClient = useQueryClient();
    const [selectedJobSplittingRunOld, setSelectedJobSplittingRunOld] = useState<JobSplittingRun[]>([]);
    const [selectedJobSplittingRun, setSelectedJobSplittingRun] = useState<JobSplittingRun[]>([]);

    const {
        isError,
        isPending,
        data: defaultJobSplittings,
    } = useListProjectGroupDefaultJobSplitting(projectGroupId);

    const update = useMutation({
        mutationFn: () => {
            if (!touched) {
                return Promise.resolve(null);
            }

            const entries = selectedJobSplittingRun
                .map(x => {
                    return {
                        type_id:  x.item.type_id,
                        max_runs: x.max_runs,
                    }
                })

            return updateDefaultJobSplitting(projectGroupId, entries);
        },
        onSuccess: () => {
            if (!touched) {
                return;
            }

            queryClient.invalidateQueries({ queryKey: [LIST_PROJECT_GROUP_DEFAULT_JOB_SPLITTING] });
            setUpdateSuccess(true);
        },
    });

    useEffect(() => {
        if (defaultJobSplittings) {
            setSelectedJobSplittingRunOld(defaultJobSplittings.runs);
            setSelectedJobSplittingRun(defaultJobSplittings.runs);
        }
    }, [defaultJobSplittings]);

    useEffect(() => {
        const a = selectedJobSplittingRunOld.map(x => x.item.type_id);
        const b = selectedJobSplittingRun.map(x => x.item.type_id);
        onTouchChange(!compareArray(a, b));
        setTouched(!compareArray(a, b));
    }, [selectedJobSplittingRun]);

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

        setSelectedJobSplittingRun(selectedJobSplittingRunOld);
    }, [triggerReset]);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const onDeleteItem = (typeId: number) => {
        const removedJobSplitRun = selectedJobSplittingRun
            .filter(x => x.item.type_id !== typeId);
        setSelectedJobSplittingRun(removedJobSplitRun)
    }

    const onSelectItem = (blueprintOverwrite: JobSplittingRun) => {
        setSelectedJobSplittingRun([
            blueprintOverwrite,
            ...selectedJobSplittingRun,
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
                Updating the job splits was successful
            </Alert>
        }
    }

    return <>
        { notification() }

        <Stack>
            <Alert variant='light' color='gray'>
                Per default, jobs are splitted based on the time they take.<br />
                For some blueprints that is not necessary wanted, for example when 41 runs are required of a blueprint, but the copy is at max 40.< br/>
                Adding the blueprint here with the max number of runs, will deactivate the time based splitting and instead limit to the max runs.
            </Alert>

            <JobSplittingRunList
                onSelect={onSelectItem}
                onDelete={onDeleteItem}
                selected={selectedJobSplittingRun}
                editable
            />
        </Stack>
    </>
}

export type defaultJobSplittingsProps = {
    projectGroupId: Uuid,

    onTouchChange(state: boolean): void,
    triggerSave:                   number;
    triggerReset:                  number;
}
