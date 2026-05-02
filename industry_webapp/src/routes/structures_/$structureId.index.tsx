import { Alert } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { deleteStructure } from '@starfoundry/components/services/structure/delete';
import { EditStructure } from '@starfoundry/components/structure/EditStructure';
import { FETCH_STRUCTURE, useFetchStructure } from '@starfoundry/components/services/structure/fetch';
import { LIST_STRUCTURE } from '@starfoundry/components/services/structure/list';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { Route as StructureListRoute } from '@/routes/structures';
import { updateStructure, type UpdateStructure } from '@starfoundry/components/services/structure/update';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

export interface QueryParams {
    created?: boolean;
}

export const Route = createFileRoute('/structures_/$structureId/')({
    component: RouteComponent,
    validateSearch: (params: {
        created: boolean,
    }): QueryParams => {
        return {
            created: (params.created) || undefined
        };
    }
})

function RouteComponent() {
    const { structureId } = Route.useParams();
    const { created: createdResource } = Route.useSearch();
    const navigation = useNavigate();
    const queryClient = useQueryClient();

    const [successfulUpdate, setSuccessfulUpdated] = useState<boolean>();
    const [errorDelete, setErrorDelete] = useState<string | undefined>();
    const [errorUpdate, setErrorUpdated] = useState<string | undefined>();

    const {
        isPending,
        isError,
        data: structure,
    } = useFetchStructure(structureId, {
        include_installable: true,
    });

    const mutationUpdate = useMutation({
        mutationFn: (data: UpdateStructure) => updateStructure(structureId, data),
        onSuccess: () => {
            setSuccessfulUpdated(true);
            queryClient.invalidateQueries({ queryKey: [FETCH_STRUCTURE, structureId] });
            queryClient.invalidateQueries({ queryKey: [LIST_STRUCTURE] });
        },
        onError: (error) => {
            setErrorUpdated(error.message);
            setSuccessfulUpdated(false);
        }
    });

    const mutationDelete = useMutation({
        mutationFn: () => deleteStructure(structureId),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [LIST_STRUCTURE] });

            navigation({
                to: StructureListRoute.to,
                search: {
                    deleted: true,
                }
            });
        },
        onError: (error) => {
            setErrorDelete(error.message);
            setSuccessfulUpdated(false);
        }
    });

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const notification = () => {
        if (createdResource) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Create successful'
                data-cy="createdSuccessfully"
            >
                The structure was successfully created
            </Alert>;
        } else if (successfulUpdate) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Update successful'
                data-cy="successfulUpdate"
                onClose={ () => setSuccessfulUpdated(false) }
                withCloseButton
            >
                The structure was updated
            </Alert>;
        } else if (errorUpdate) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Update error'
                data-cy="errorUpdate"
                onClose={ () => setErrorUpdated(undefined) }
                withCloseButton
            >
                There was an error while updating. Please try again later.
            </Alert>;
        } else if (errorDelete) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Delete error'
                data-cy="errorDelete"
                onClose={ () => setErrorDelete(undefined) }
                withCloseButton
            >
                There was an error while deleting. Please try again later.
            </Alert>;
        }
    };

    return <>
        { notification() }

        <EditStructure
            structure={structure}

            onDelete={mutationDelete.mutate}
            onUpdate={mutationUpdate.mutate}
        />
    </>
}
