import { Alert, Button, Flex, TextInput } from '@mantine/core';
import { createStructureGroup, type CreateStructureGroup } from '@/services/structure-group/create';
import { Route as ProjectGroupRoute } from '@/routes/structure-groups_/$structureGroupId.index';
import { useForm } from '@tanstack/react-form';
import { useMutation } from '@tanstack/react-query';
import { useNavigate } from '@tanstack/react-router';
import { useState } from 'react';

export function AddStructureGroup({
    close,
}: Props) {
    const navigation = useNavigate();

    const [errorCreate, setErrorCreate] = useState<string | undefined>();

    const mutation = useMutation({
        mutationFn: async (value: CreateStructureGroup) => {
            return await createStructureGroup(value)
        },
    });

    const form = useForm({
        defaultValues: {
            name: '',
        },
        onSubmit: async ({ value }) => await mutation
            .mutateAsync(value)
            .then(x => {
                navigation({
                    to: ProjectGroupRoute.to,
                    params: {
                        structureGroupId: x.id,
                    },
                    search: {
                        created: true,
                    }
                });
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
                form.handleSubmit();
            }}
        >
            <form.Field
                name="name"
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
                            label="Name"
                            description="Name of the new structure group"
                            placeholder="My cool structure group"
                            id={field.name}
                            name={field.name}
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
                            onClick={ close }
                        >
                            Close
                        </Button>
                        <Button
                            data-cy="create"
                            mt="sm"
                            type="submit"
                            disabled={!canSubmit || isSubmitting}
                            loading={isSubmitting}
                        >
                            Add
                        </Button>
                    </Flex>
                )}
            />
        </form>
    </>
}

export type Props = {
    close: () => void,
}
