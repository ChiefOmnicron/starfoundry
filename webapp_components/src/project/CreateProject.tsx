import { Alert, Button, Group, InputBase, NumberInput, Stack } from '@mantine/core';
import { createProject, type CreateProject, type CreateProjectResponse } from '@internal/services/projects/create';
import { LoadingAnimation } from '@internal/misc/LoadingAnimation';
import { LoadingError } from '@internal/misc/LoadingError';
import { ModalWrapper } from '@internal/wrapper/Modal';
import { ProjectGroupSelector } from '@internal/selectors/ProjectGroupSelector';
import { useForm } from '@tanstack/react-form';
import { useListProjectGroup } from '@internal/services/project-group/list';
import { useMutation } from '@tanstack/react-query';
import { useState } from 'react';
import type { Uuid } from '@internal/services/utils';
import { MarkdownEditor } from '@internal/misc/MarkdownEditor';

export function CreateProject({
    onCreate,
}: CreateProjectProps) {

    const [errorCreate, setErrorCreate] = useState<string | undefined>();

    const {
        isError: projectGroupError,
        isPending: projectGroupPending,
        data: projectGroups,
    } = useListProjectGroup({
        archived: false,
    });

    const createProjectMutation = useMutation({
        mutationFn: async (value: CreateProject) => {
            return await createProject(value)
        },
        onSuccess: (data: CreateProjectResponse) => {
            onCreate(data.id);
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
            notes: '',

            project_group_id: '',
        },
        onSubmit: async ({ value }) => {
            await createProjectMutation.mutateAsync(value);
            form.reset();
        }
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
                    name="notes"
                    children={(field) => {
                        return <>
                            <MarkdownEditor
                                content={field.state.value}
                                title='Notes'
                                height='200px'
                                onChange={(value) => field.handleChange(value)}
                            />
                        </>
                    }}
                />

                <form.Subscribe
                    selector={(state) => [state.canSubmit, state.isSubmitting]}
                    children={([canSubmit, isSubmitting]) => (
                        <Group
                            justify="flex-end"
                            gap="sm"
                        >
                            <Button
                                mt="sm"
                                variant="subtle"
                                color="gray"
                                onClick={close}
                            >
                                Close
                            </Button>
                            <Button
                                data-cy="create"
                                mt="sm"
                                type="submit"
                                disabled={!canSubmit}
                                loading={isSubmitting}
                            >
                                Create
                            </Button>
                        </Group>
                    )}
                />
            </Stack>
        </form>
    </>
}

export function CreateProjectModal({
    opened,
    close,

    onCreate,
}: CreateProjectModalProps) {
    return <ModalWrapper
        opened={opened}
        close={close}
        title="Add project"
    >
        <CreateProject
            onCreate={onCreate}
        />
    </ModalWrapper>
}

export type CreateProjectProps = {
    onCreate: (projectId: Uuid) => void;
}

export type CreateProjectModalProps = {
    opened: boolean;
    close: () => void;

    onCreate: (projectId: Uuid) => void;
}
