import { Alert, Button, Flex, TextInput } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { type CreateProjectGroup as AddStructure } from '@/services/project-group/create';
import { LIST_STRUCTURE } from '@/services/structure/list';
import { Route as ListProjectGroupRoute } from '@/routes/project-groups/index';
import { useForm } from '@tanstack/react-form'
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';
import { resolveStructure } from '@/services/structure/resolveStructure';

export const Route = createFileRoute('/structures/create')({
    component: AddStructure,
})

export function AddStructure() {
    const queryClient = useQueryClient();
    const navigation = useNavigate({ from: Route.fullPath });

    const [errorCreate, setErrorCreate] = useState<string | undefined>();

    const create = useMutation({
        mutationFn: async (data: number) => {
            return await resolveStructure(data);
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [LIST_STRUCTURE] })
        },
    });

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

        <form
            onSubmit={(e) => {
                e.preventDefault();
                e.stopPropagation();
                resolveStructureForm.handleSubmit();
            }}
        >
            <form.Field
                name="structure_id"
                validators={{
                    onBlur: ({ value }) => {
                        return (value.trimStart().length === 0 ? 'The field is required' : undefined) ||
                            (value.length > 100 ? 'Maximum allowed chars is 100' : undefined)
                    }
                }}
                children={(field) => {
                    return <>
                        <TextInput
                            data-1p-ignore
                            data-cy="name"
                            label="Eve-Structure ID"
                            description="Either an EVE-Structure ID or a chat link to a structure"
                            placeholder="Structure ID"
                            id={field.eveStructureId}
                            name={field.eveStructureId}
                            value={field.state.value}
                            error={
                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                            }
                            onBlur={field.handleBlur}
                            onChange={(e) => field.handleChange(e.target.value)}
                        />
                    </>
                }}
            />

            <form.Subscribe
                selector={(state) => [state.canSubmit, state.isSubmitting]}
                children={([canSubmit, isSubmitting]) => (
                    <Flex
                        justify="flex-end"
                        gap="sm"
                    >
                        <Button
                            mt="sm"
                            variant="subtle"
                            color="gray"
                            onClick={() => navigation({ to: ListProjectGroupRoute.to })}
                        >
                            Back
                        </Button>
                        <Button
                            data-cy="create"
                            mt="sm"
                            type="submit"
                            disabled={!canSubmit || isSubmitting}
                            loading={isSubmitting}
                        >
                            Create
                        </Button>
                    </Flex>
                )}
            />
        </form>
    </>
}
