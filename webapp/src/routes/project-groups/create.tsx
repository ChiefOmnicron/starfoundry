import { Alert, Button, Flex, Textarea, TextInput } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { createProjectGroup, type CreateProjectGroup } from '@/services/project-group/create';
import { LIST_PROJECT_GROUPS } from '@/services/project-group/list';
import { Route as ListProjectGroupRoute } from '@/routes/project-groups/index';
import { Route as ProjectGroupOverviewRoute } from '@/routes/project-groups_/$projectGroupId.index';
import { useForm } from '@tanstack/react-form'
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

export const Route = createFileRoute('/project-groups/create')({
    component: CreateProjectGroup,
})

export function CreateProjectGroup() {
    const queryClient = useQueryClient();
    const navigation = useNavigate({ from: Route.fullPath });

    const [errorCreate, setErrorCreate] = useState<string | undefined>();

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
            .then(x => {
                navigation({
                    to: ProjectGroupOverviewRoute.to,
                    params: {
                        projectGroupId: x.id,
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
