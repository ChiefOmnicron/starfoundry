import { Alert, Flex, Textarea, TextInput, Title } from '@mantine/core';
import { archiveProjectGroup } from '@/services/project-group/archive';
import { ArchiveResource } from '@/components/ArchiveResource';
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { deleteProjectGroup } from '@/services/project-group/delete';
import { DeleteResource } from '@/components/DeleteResource';
import { FETCH_PROJECT_GROUP, useFetchProjectGroup } from '@/services/project-group/fetch';
import { LIST_PROJECT_GROUPS } from '@/services/project-group/list';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { Route as ProjectGroupRoute } from '@/routes/project-groups/index';
import { SaveDialog } from '@/components/SaveDialog';
import { updateProjectGroup, type UpdateProjectGroup } from '@/services/project-group/updateGroup';
import { useFetchProjectGroupMemberSelf } from '@/services/project-group/fetchMemberSelf';
import { useForm } from '@tanstack/react-form';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

interface QueryParams {
    created?: boolean;
}

export const Route = createFileRoute(
    '/project-groups_/$projectGroupId/overview',
)({
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
    const navigation = useNavigate();
    const queryClient = useQueryClient();

    const { projectGroupId } = Route.useParams();
    const { created: createdResource } = Route.useSearch();

    const [successfulArchive, setSuccessfulArchive] = useState<boolean>();
    const [successfulUpdate, setSuccessfulUpdate] = useState<boolean>();
    const [errorArchive, setErrorArchive] = useState<string | undefined>();
    const [errorDelete, setErrorDelete] = useState<string | undefined>();
    const [errorUpdate, setErrorUpdated] = useState<string | undefined>();

    const [touched, setTouched] = useState<boolean>(false);

    const {
        isError,
        isPending,
        data: projectGroup,
    } = useFetchProjectGroup(projectGroupId);

    const {
        data: memberSelf,
    } = useFetchProjectGroupMemberSelf(projectGroupId);

    const mutationUpdate = useMutation({
        mutationFn: (data: UpdateProjectGroup) => updateProjectGroup(projectGroupId, data),
        onSuccess: () => {
            setErrorUpdated(undefined);
            setSuccessfulUpdate(true);
            setTouched(false);
            queryClient.invalidateQueries({ queryKey: [FETCH_PROJECT_GROUP] })
        },
    });

    const mutationArchive = useMutation({
        mutationFn: () => archiveProjectGroup(projectGroupId),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [FETCH_PROJECT_GROUP] });
            queryClient.invalidateQueries({ queryKey: [LIST_PROJECT_GROUPS] });

            setSuccessfulArchive(true);
            setErrorArchive(undefined);
        },
    });

    const mutationDelete = useMutation({
        mutationFn: () => deleteProjectGroup(projectGroupId),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [FETCH_PROJECT_GROUP] });
            queryClient.invalidateQueries({ queryKey: [LIST_PROJECT_GROUPS] });

            navigation({
                to: ProjectGroupRoute.to,
                search: {
                    deleted: true,
                }
            });
        },
    });

    const form = useForm({
        defaultValues: {
            name: projectGroup?.name || '',
            description: projectGroup?.description || '',
        },
        onSubmit: async ({ value }) => await mutationUpdate
            .mutateAsync(value)
            .catch(error => {
                setErrorUpdated(error);
                setSuccessfulUpdate(false);
            }),
    });

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError && !projectGroup) {
        return LoadingError();
    }

    const archiveGroup = async () => {
        await mutationArchive
            .mutateAsync()
            .catch(error => {
                setErrorArchive(error);
                setSuccessfulUpdate(false);
            });
    }

    const deleteGroup = async () => {
        await mutationDelete
            .mutateAsync()
            .catch(error => {
                setErrorDelete(error);
                setSuccessfulUpdate(false);
            });
    }

    const canWrite = (): boolean => {
        if (memberSelf) {
            let has_write_group = memberSelf
                .permissions
                .indexOf('WRITE_GROUP') > -1;
            let is_owner = memberSelf
                .permissions
                .indexOf('OWNER') > -1;
            return has_write_group || is_owner;
        }

        return false;
    };

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
        } else if (successfulArchive) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title={`${projectGroup.archived ? 'Archive' : 'Unarchive'} successful`}
                data-cy="successfulArchive"
                onClose={ () => setSuccessfulArchive(false) }
                withCloseButton
            >
                {
                    projectGroup.archived
                    ? 'The project group was archived'
                    : 'The project group was unarchived'
                }
            </Alert>;
        } else if (createdResource) {
            return <Alert
                mt="sm"
                variant='light'
                color='green'
                title='Create successful'
                data-cy="createSuccessful"
            >
                The project group was successfully created
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
        } else if (errorArchive) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title={`${projectGroup.archived ? 'Unarchive' : 'Archive'} error`}
                data-cy="errorArchive"
                onClose={ () => setErrorArchive(undefined) }
                withCloseButton
            >
                There was an error while archiving. Please try again later.
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
        if (!memberSelf?.is_owner) {
            return <></>
        }

        return <>
            <Title
                data-cy="danger-zone-header"
                order={2}
                mt="md"
            >
                Danger Zone
            </Title>

            <ArchiveResource
                isArchived={projectGroup.archived}
                resource={projectGroup.name}
                onConfirm={() => archiveGroup()}
            />

            <DeleteResource
                resource={projectGroup.name}
                onConfirm={() => deleteGroup()}
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
                            disabled={!canWrite()}
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
                            disabled={!canWrite()}
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
                        {
                            canWrite()
                                ? <SaveDialog
                                    onReset={ () => {
                                        form.reset();
                                        setTouched(!form.state.isDefaultValue);
                                    }}
                                    onSave={ form.handleSubmit }
                                    show={ touched }
                                />
                                : <></>
                        }
                    </Flex>
                )}
            />
        </form>

        { dangerZone() }
    </>
}
