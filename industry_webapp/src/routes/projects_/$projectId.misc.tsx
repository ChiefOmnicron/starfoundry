import { Alert } from '@mantine/core';
import { createFileRoute } from '@tanstack/react-router'
import { FETCH_PROJECT_COST } from '@starfoundry/components/services/projects/fetchCost';
import { LIST_PROJECT_MISC, useListProjectMisc, type ProjectMisc } from '@starfoundry/components/services/projects/listMisc';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { ProjectMiscList } from './-components/MiscList'
import { SaveDialog } from '@starfoundry/components/misc/SaveDialog';
import { updateMisc } from '@starfoundry/components/services/projects/updateMisc';
import { useEffect, useState } from 'react';
import { useMutation, useQueryClient } from '@tanstack/react-query';

export const Route = createFileRoute('/projects_/$projectId/misc')({
    component: RouteComponent,
})

function RouteComponent() {
    const queryClient = useQueryClient();
    const { projectId } = Route.useParams();

    const [selectedProjectMiscOld, setSelectedProjectMiscOld] = useState<ProjectMisc[]>([]);
    const [selectedProjectMisc, setSelectedProjectMisc] = useState<ProjectMisc[]>([]);

    const [successfulUpdate, setSuccessfulUpdate] = useState<boolean>();
    const [errorUpdate, setErrorUpdate] = useState<string | undefined>();

    const {
        isError,
        isPending,
        data: projectMisc,
    } = useListProjectMisc(projectId);

    useEffect(() => {
        if (projectMisc) {
            setSelectedProjectMiscOld(projectMisc);
            setSelectedProjectMisc(projectMisc);
        }
    }, [projectMisc]);

    const miscMutation = useMutation({
        mutationFn: () => {
            const entries = selectedProjectMisc
                .map(x => {
                    return {
                        item: x.item,
                        cost: x.cost,
                        description: x.description ? x.description : undefined,
                        quantity: x.quantity ? x.quantity : undefined,
                    }
                });

            return updateMisc(
                projectId,
                entries,
            )
        },
        onSuccess: () => {
            setErrorUpdate(undefined);
            setSuccessfulUpdate(true);
            queryClient.invalidateQueries({ queryKey: [LIST_PROJECT_MISC, projectId] });
            queryClient.invalidateQueries({ queryKey: [FETCH_PROJECT_COST, projectId] });
        },
        onError: (error) => {
            setErrorUpdate(error as any);
            setSuccessfulUpdate(false);
        }
    });

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const onSelect = (misc: ProjectMisc) => {
        console.log(misc, selectedProjectMisc, [
            misc,
            ...selectedProjectMisc,
        ])
        setSelectedProjectMisc([
            misc,
            ...selectedProjectMisc,
        ]);
    }

    const onDelete = (misc: ProjectMisc) => {
        const filtered = selectedProjectMisc
            .filter(x => x !== misc);
        setSelectedProjectMisc(filtered);
    }

    const resetChanges = () => {
        setSelectedProjectMisc(selectedProjectMiscOld);
    }

    const saveChanges = () => {
        miscMutation.mutate();
    }

    const notification = () => {
        if (successfulUpdate) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Update successful'
                data-cy="updateSuccessful"
            >
                Updating the industry hubs was successful
            </Alert>;
        } else if (errorUpdate) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Update error'
                data-cy="errorUpdate"
                onClose={ () => setErrorUpdate(undefined) }
                withCloseButton
            >
                There was an error while updating. Please try again later.
            </Alert>;
        } else {
            return <></>
        }
    }

    return <>
        { notification() }

        <ProjectMiscList
            selected={selectedProjectMisc}
            onSelect={onSelect}
            onDelete={onDelete}
            editable
        />

        <SaveDialog
            onReset={resetChanges}
            onSave={saveChanges}
            show={ selectedProjectMisc !== selectedProjectMiscOld }
        />
    </>
}
