import LoadingAnimation from '@/components/LoadingAnimation';
import { FETCH_PROJECT_GROUP, useFetchProjectGroup } from '@/services/project-group/fetch';
import { updateProjectGroup, type UpdateProjectGroup } from '@/services/project-group/update_group';
import { Alert, Button, Flex, Textarea, TextInput } from '@mantine/core';
import { useForm } from '@tanstack/react-form';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { createFileRoute } from '@tanstack/react-router'
import { useState } from 'react';

export const Route = createFileRoute(
    '/project-groups_/$projectGroupId/overview',
)({
    component: ProjectGroupOverview,
})

export function ProjectGroupOverview() {
    const queryClient = useQueryClient();
    const { projectGroupId } = Route.useParams();

    const [successfulUpdate, setSuccessfulUpdated] = useState<boolean>();
    const [errorUpdate, setErrorUpdated] = useState<string | undefined>();

    const {
        isError,
        isPending,
        data: projectGroup,
    } = useFetchProjectGroup(projectGroupId);

    const update = useMutation({
        mutationFn: (data: UpdateProjectGroup) => updateProjectGroup(projectGroupId, data),
        onSuccess: () => {
            setErrorUpdated(undefined);
            setSuccessfulUpdated(true);
            queryClient.invalidateQueries({ queryKey: [FETCH_PROJECT_GROUP] })
        },
    });

    const form = useForm({
        defaultValues: {
            name: projectGroup?.name || '',
            description: projectGroup?.description || '',
        },
        onSubmit: async ({ value }) => await update
            .mutateAsync(value)
            .catch(error => {
                setErrorUpdated(error);
                setSuccessfulUpdated(false);
            }),
    });

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError && !projectGroup) {
        return <Alert
            variant='light'
            color='red'
            title='Unknown loading error'
            data-cy="error"
        >
            There was an unknown error while loading the data. Please try again later.
        </Alert>
    }

    const notification = () => {
        if (successfulUpdate) {
            return <Alert
                variant='light'
                color='green'
                title='Update successful'
                data-cy="successfulUpdate"
            >
                The project group was updated
            </Alert>;
        } else if (errorUpdate) {
            return <Alert
                variant='light'
                color='red'
                title='Update error'
                data-cy="errorUpdate"
            >
                There was an error while updating. Please try again later.
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
                            data-cy="save"
                            mt="sm"
                            type="submit"
                            disabled={!canSubmit || isSubmitting}
                            loading={isSubmitting}
                        >
                            Save
                        </Button>
                    </Flex>
                )}
            />
        </form>
    </>
}
