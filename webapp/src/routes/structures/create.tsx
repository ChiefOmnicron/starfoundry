import { Alert, Button, Flex, Group, TextInput } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { type CreateProjectGroup as AddStructure } from '@/services/project-group/create';
import { LIST_STRUCTURE } from '@/services/structure/list';
import { Route as ListProjectGroupRoute } from '@/routes/project-groups/index';
import { useForm } from '@tanstack/react-form'
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';
import { resolveStructure, type ResolveStructureResponse } from '@/services/structure/resolveStructure';
import { ResolveStructure } from './_components/resolveStructure';

export const Route = createFileRoute('/structures/create')({
    component: AddStructure,
})

export function AddStructure() {
    const queryClient = useQueryClient();
    const navigation = useNavigate({ from: Route.fullPath });

    const [errorCreate, setErrorCreate] = useState<string | undefined>();

    const resolveStructureForm = useForm({
        defaultValues: {
            structure_id: 0,
        },
        onSubmit: async ({ value }) => await create
            .mutateAsync(value.structure_id)
            .then(x => {
                // TODO:
            })
            .catch(error => {
                setErrorCreate(error);
            }),
    });

    const notification = () => {
        if (errorCreate) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Create error'
                data-cy="errorCreate"
                onClose={ () => setErrorCreate(undefined) }
                withCloseButton
            >
                There was an error while creating. Please try again later.
            </Alert>;
        }
    };

    return <>
        { notification() }

        <ResolveStructure
            onError={(error) => {
                console.log(error)
            }}
            onSuccess={(structure) => {
                console.log(structure)
            }}
        />
    </>
}
