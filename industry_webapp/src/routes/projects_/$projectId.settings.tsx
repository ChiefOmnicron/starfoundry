import { Alert, Flex, InputBase, NumberInput, Stack, TextInput, Title } from '@mantine/core';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { deleteProject } from '@starfoundry/components/services/projects/delete';
import { DeleteResource } from '@starfoundry/components/misc/DeleteResource';
import { FETCH_PROJECT, useFetchProject } from '@starfoundry/components/services/projects/fetch';
import { LIST_PROJECT } from '@starfoundry/components/services/projects/list';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { MarkdownEditor } from '@starfoundry/components/misc/MarkdownEditor';
import { ProjectGroupSelector } from '@starfoundry/components/selectors/ProjectGroupSelector';
import { ProjectStatusSelector } from '@starfoundry/components/project/ProjectStatus';
import { Route as ProjectRoute } from '@/routes/projects/index';
import { SaveDialog } from '@starfoundry/components/misc/SaveDialog';
import { updateProject, type UpdateProjectRequest } from '@starfoundry/components/services/projects/update';
import { useForm } from '@tanstack/react-form';
import { useListProjectGroup } from '@starfoundry/components/services/project-group/list';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

export interface QueryParams {
    created?: boolean;
}

export const Route = createFileRoute(
    '/projects_/$projectId/settings',
)({
    component: RouteComponent,
})

function RouteComponent() {
    const navigation = useNavigate();
    const queryClient = useQueryClient();

    const { projectId } = Route.useParams();

    const [successfulUpdate, setSuccessfulUpdate] = useState<boolean>();
    const [errorDelete, setErrorDelete] = useState<string | undefined>();
    const [errorUpdate, setErrorUpdate] = useState<string | undefined>();

    const [touched, setTouched] = useState<boolean>(false);

    const {
        isError,
        isPending,
        data: project,
    } = useFetchProject(projectId);

    const {
        isError: projectGroupError,
        isPending: projectGroupPending,
        data: projectGroups,
    } = useListProjectGroup({
        archived: false,
    });

    const updateMutation = useMutation({
        mutationFn: (data: UpdateProjectRequest) => updateProject(projectId, data),
        onSuccess: () => {
            setErrorUpdate(undefined);
            setSuccessfulUpdate(true);
            setTouched(false);
            queryClient.invalidateQueries({ queryKey: [FETCH_PROJECT] })
        },
        onError: (error) => {
            setErrorUpdate(error.message);
            setSuccessfulUpdate(false);
        },
    });

    const deleteMutation = useMutation({
        mutationFn: () => deleteProject(projectId),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [FETCH_PROJECT] });
            queryClient.invalidateQueries({ queryKey: [LIST_PROJECT] });

            navigation({
                to: ProjectRoute.to,
                search: {
                    deleted: true,
                }
            });
        },
        onError: (error) => {
            setErrorDelete(error.message);
        },
    });

    const form = useForm({
        defaultValues: {
            name: project?.name || '',
            project_group_id: project?.project_group.id || '',
            orderer: project?.orderer || '',
            sell_price: project?.sell_price || 0,
            note: project?.note || '',
            status: project?.status || 'READY_TO_START',
        },
        onSubmit: async ({ value }) => await updateMutation
            .mutateAsync(value)
            .catch(error => {
                setErrorUpdate(error);
                setSuccessfulUpdate(false);
            }),
    });

    if (isPending || projectGroupPending) {
        return LoadingAnimation();
    }

    if ((isError && !project) || projectGroupError) {
        return LoadingError();
    }

    const notification = () => {
        if (successfulUpdate) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Update successful'
                data-cy="successfulUpdate"
                onClose={ () => setSuccessfulUpdate(false) }
                withCloseButton
            >
                The project group was updated
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

    const dangerZone = () => {
        return <>
            <Title
                data-cy="danger-zone-header"
                order={2}
                mt="md"
            >
                Danger Zone
            </Title>

            <DeleteResource
                resource={project.name}
                onConfirm={() => {
                    deleteMutation.mutate();
                }}
            />
        </>
    }

    return <>
        { notification() }

        <form
            onChange={() => {
                setTouched(!form.state.isDefaultValue);
            }}
            onSubmit={(e) => {
                e.preventDefault();
                e.stopPropagation();
                form.handleSubmit();
            }}
        >
            <Stack>
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
                                withAsterisk
                                data-cy="name"
                                label="Name"
                                description="Name of the project"
                                placeholder="My cool project"
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
                    name="project_group_id"
                    children={(field) => {
                        return <>
                            <ProjectGroupSelector
                                selected={field.state.value}
                                projectGroups={projectGroups}
                                onSelect={(e) => {
                                    console.log(e)
                                    field.handleChange(e.id);
                                    setTouched(true);
                                }}
                            />
                        </>
                    }}
                />

                <form.Field
                    name="orderer"
                    validators={{
                        onBlur: ({ value }) => {
                            return (value.trimStart().length === 0 ? 'The field is required' : undefined) ||
                                (value.length > 100 ? 'Maximum allowed chars is 100' : undefined)
                        }
                    }}
                    children={(field) => {
                        return <>
                            <InputBase
                                data-1p-ignore
                                withAsterisk
                                data-cy="orderer"
                                label="Orderer"
                                description="Insert the name of orderer"
                                placeholder="Some character or corporation"
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
                    name="sell_price"
                    children={(field) => {
                        return <>
                            <NumberInput
                                data-1p-ignore
                                thousandSeparator
                                data-cy="sellPrice"
                                label="Sell price"
                                description="Price of the products"
                                placeholder="1,000,000,000"
                                id={field.name}
                                name={field.name}
                                value={field.state.value}
                                error={
                                    !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                }
                                onBlur={field.handleBlur}
                                onChange={(e) => {
                                    if (e) {
                                        field.handleChange(e as number);
                                    }
                                }}
                            />
                        </>
                    }}
                />

                <form.Field
                    name="status"
                    children={(field) => {
                        return <>
                            <ProjectStatusSelector
                                selected={field.state.value}
                                onChange={field.handleChange}
                            />
                        </>
                    }}
                />

                <form.Field
                    name="note"
                    validators={{
                        onBlur: ({ value }) => {
                            return (value.length > 10000 ? 'Maximum allowed chars is 10000' : undefined)
                        }
                    }}
                    children={(field) => {
                        return <>
                            <MarkdownEditor
                                title='Description'
                                content={field.state.value}
                                height='200px'
                                onChange={(value) => {
                                    field.handleChange(value);
                                    setTouched(true);
                                }}
                            />
                        </>
                    }}
                />

                <form.Subscribe
                    selector={(state) => [state.canSubmit, state.isSubmitting]}
                    children={() => (
                        <Flex
                            justify="flex-end"
                            gap="sm"
                        >
                            <SaveDialog
                                onReset={ () => {
                                    form.reset();
                                    setTouched(!form.state.isDefaultValue);
                                }}
                                onSave={ form.handleSubmit }
                                show={ touched }
                            />
                        </Flex>
                    )}
                />
            </Stack>
        </form>

        { dangerZone() }
    </>
}
