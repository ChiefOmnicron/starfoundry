import { Alert, Button, Flex, InputBase, Modal, NumberInput, Stack } from '@mantine/core';
import { createProject, type CreateProject } from '@/services/projects/create';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { ProjectGroupSelector } from '@/components/selectors/ProjectGroupSelector';
import { Route as ProjectOverviewRoute } from '@/routes/projects_/$projectId.overview';
import { Route as StructureRoute } from '@/routes/structures_/$structureId.index';
import { useForm } from '@tanstack/react-form';
import { useListProjectGroup } from '@/services/project-group/list';
import { useMutation } from '@tanstack/react-query';
import { useNavigate } from '@tanstack/react-router';
import { useState } from 'react';
import type { Uuid } from '@/services/utils';

export function AddProject({
    close
}: AddProjectProps) {
    const navigation = useNavigate();

    const [errorCreate, setErrorCreate] = useState<string | undefined>();

    const {
        isError: projectGroupError,
        isPending: projectGroupPending,
        data: projectGroups,
    } = useListProjectGroup({
        archived: false,
    });

    const addProjectMutation = useMutation({
        mutationFn: async (value: CreateProject) => {
            return await createProject(value)
        },
        onSuccess: (data: { id: Uuid }) => {
            return navigation({
                to: StructureRoute.to,
                params: {
                    structureId: data.id,
                },
                search: {
                    created: true,
                },
            });
        },
        onError: (error) => {
            setErrorCreate(error.message);
        }
    });

    const form = useForm({
        defaultValues: {
            name: '',
            orderer: '',
            sellPrice: 0,

            project_group_id: '',
        },
        onSubmit: async ({ value }) => await addProjectMutation
            .mutateAsync(value)
            .then(x => {
                navigation({
                    to: ProjectOverviewRoute.to,
                    params: {
                        projectId: x.id,
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

    if (projectGroupPending) {
        return LoadingAnimation();
    }

    if (projectGroupError) {
        return LoadingError();
    }

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
        {notification()}

        <form
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
                            <InputBase
                                data-1p-ignore
                                withAsterisk
                                data-cy="name"
                                label="Name"
                                description="Insert the name of the new project"
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
                                projectGroups={ projectGroups }
                                onSelect={(e) => field.handleChange(e.id)}
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
                    name="sellPrice"
                    children={(field) => {
                        return <>
                            <NumberInput
                                data-1p-ignore
                                thousandSeparator
                                data-cy="sellPrice"
                                label="Sell price"
                                description="Price of the "
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
                                onClick={close}
                            >
                                Back
                            </Button>
                            <Button
                                data-cy="create"
                                mt="sm"
                                type="submit"
                                disabled={!canSubmit}
                                loading={isSubmitting}
                            >
                                Add
                            </Button>
                        </Flex>
                    )}
                />
            </Stack>
        </form>
    </>
}

export function AddProjectModal({
    opened,
    close,
}: AddProjectModalProps) {
    return <Modal
        opened={ opened }
        onClose={ close }
        title="Add structure"
        overlayProps={{
            backgroundOpacity: 0.55,
            blur: 3,
        }}
        size="70%"
        centered
        closeOnEscape
        closeOnClickOutside
    >
        <AddProject
            close={close}
        />
    </Modal>
}

export type AddProjectProps = {
    close: () => void,
}

export type AddProjectModalProps = {
    opened: boolean;
    close: () => void,
}
