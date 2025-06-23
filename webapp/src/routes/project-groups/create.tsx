import { Alert, Button, Flex, Textarea, TextInput } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { Route as listProjectGroupRoute } from '@/routes/project-groups/index';
import { useForm } from '@tanstack/react-form'
import { createProjectGroup, type CreateProjectGroup } from '@/services/project-group/create';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { LIST_PROJECT_GROUPS } from '@/services/project-group/list';
import { useState } from 'react';

export const Route = createFileRoute('/project-groups/create')({
    component: CreateProjectGroup,
})

export function CreateProjectGroup() {
    const queryClient = useQueryClient();
    const navigation = useNavigate({ from: Route.fullPath });

    const [errorUpdate, setErrorUpdated] = useState<string | undefined>();

    const create = useMutation({
        mutationFn: async (data: CreateProjectGroup) => {
            return await createProjectGroup(data);
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [LIST_PROJECT_GROUPS] })
        },
    });

    const form = useForm({
        defaultValues: {
            name: '',
            description: '',
        },
        onSubmit: async ({ value }) => await create
            .mutateAsync(value)
            .catch(error => {
                setErrorUpdated(error);
            }),
            // TODO: redirect
    });

    const notification = () => {
        if (errorUpdate) {
            return <Alert
                variant='light'
                color='red'
                title='Update error'
                data-cy="errorUpdate"
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
                            description="Name of the new project group"
                            placeholder="My cool project group"
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
            <form.Field
                name="description"
                validators={{
                    onBlur: ({ value }) => {
                        return (value.length > 10000 ? 'Maximum allowed chars is 10000' : undefined)
                    }
                }}
                children={(field) => {
                    return <>
                        <Textarea
                            data-1p-ignore
                            data-cy="description"
                            mt="sm"
                            label="Description (optional)"
                            description="Description of the project group"
                            placeholder="Only cool projects in here"
                            id={field.name}
                            name={field.name}
                            value={field.state.value}
                            error={
                                !field.state.meta.isValid && field.state.meta.errors.join(', ')
                            }
                            onBlur={field.handleBlur}
                            onChange={(e) => field.handleChange(e.target.value)}
                            autosize
                            minRows={3}
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
                            onClick={() => navigation({ to: listProjectGroupRoute.to })}
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
