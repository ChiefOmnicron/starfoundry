import { Button, Flex, InputBase, NumberInput, Stack } from '@mantine/core';
import { useForm } from '@tanstack/react-form';
import type { ProjectAssistantGeneralInformation } from '@/routes/projects/assistant';
import { useListProjectGroup } from '@starfoundry/components/services/project-group/list';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { ProjectGroupSelector } from '@starfoundry/components/selectors/ProjectGroupSelector';

export function GeneralInfo({
    nextStep,
    prevStep,
}: GeneralInfoProps) {
    const {
        isError: projectGroupError,
        isPending: projectGroupPending,
        data: projectGroups,
    } = useListProjectGroup({});

    const form = useForm({
        defaultValues: {
            name: '',
            orderer: '',
            sellPrice: 0,

            projectGroupId: '',
        },
        onSubmit: async ({ value }) => {
            nextStep(value)
        }
    });

    if (projectGroupPending) {
        return LoadingAnimation();
    }

    if (projectGroupError) {
        return LoadingError();
    }

    return <>
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
                    name="projectGroupId"
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
                                onClick={ prevStep }
                            >
                                Back
                            </Button>
                            <Button
                                data-cy="create"
                                mt="sm"
                                type="submit"
                                disabled={ !canSubmit }
                                loading={ isSubmitting }
                            >
                                Next Step
                            </Button>
                        </Flex>
                    )}
                />
            </Stack>
        </form>
    </>
}

export type GeneralInfoProps = {
    nextStep: (info: ProjectAssistantGeneralInformation) => void
    prevStep: () => void
}
