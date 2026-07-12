import { BadgeWrapper } from "@internal/wrapper/Badge";
import { Alert, Button, ColorInput, Grid, Group, InputBase, InputWrapper, SegmentedControl, Select, Stack, TextInput } from "@mantine/core";
import { createTag, type CreateTag } from "@internal/services/tags/create";
import { randomColor } from "@internal/utils";
import { useForm } from "@tanstack/react-form";
import { useMutation } from "@tanstack/react-query";
import { LIST_TAGS, type AutoTagCompare, type AutoTagSelect, type TagType } from "@internal/services/tags/list";
import { useState, type ReactElement } from "react";
import { tagOptions } from "@internal/services/tags/options";

export function CreateTag({
    onCreate,
}: CreateTagProps): ReactElement {
    const [hasError, setHasError] = useState<boolean>(false);

    const createTagMutation = useMutation({
        mutationFn: async (value: CreateTag) => {
            console.log(value)
            return await createTag(value)
        },
        onError: () => {
            setHasError(true);
        },
        onSuccess: (_data, _variables, _onMutateResult, context) => {
            context.client.invalidateQueries({ queryKey: [LIST_TAGS] });
            setHasError(false);
            onCreate();
        },
    });

    const form = useForm({
        defaultValues: {
            content: '',
            color: randomColor(),
            typ: 'MANUAL',
            auto: [{
                compare: 'IS',
                select: 'PROJECT_NAME',
                value: '',
            }],
        },
        onSubmit: async ({ value }) => await createTagMutation
            .mutateAsync({
                color:      value.color,
                content:    value.content,
                typ:        value.typ as TagType,

                auto:       value.typ === 'AUTO'
                            ?   value.auto.map(x => {
                                    return {
                                        compare: x.compare as AutoTagCompare,
                                        select: x.select as AutoTagSelect,
                                        value: x.value,
                                    }
                                })
                            :   []
            }),
    });

    const autoConfiguration = (index: number) => {
        return <>
            <Grid>
                <Grid.Col span={3}>
                    <form.Field
                        name={`auto[${index}].select`}
                        children={(field) => {
                            return <Select
                                label="Select an option"
                                placeholder="project.name"
                                id={field.name}
                                name={field.name}
                                value={field.state.value}
                                error={
                                    !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                }
                                onChange={(x) => field.handleChange(x as any)}
                                data={tagOptions}
                                withAsterisk
                            />
                        }}
                    />
                </Grid.Col>

                <Grid.Col span={2}>
                    <form.Subscribe
                        selector={(state) => [state.values.auto[index].select]}
                        children={([select]) => {
                            return <form.Field
                                name={`auto[${index}].compare`}
                                children={(field) => {
                                    return <Select
                                        label="Compare"
                                        placeholder="is"
                                        id={field.name}
                                        name={field.name}
                                        value={field.state.value}
                                        error={
                                            !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                        }
                                        onChange={(x) => {
                                            field.handleChange(x as any)
                                        }}
                                        data={(tagOptions.find(x => x.value === select) || { compare: [] }).compare}
                                        allowDeselect={false}
                                        withAsterisk
                                    />
                                }}
                            />
                        }}
                    />
                </Grid.Col>
                <Grid.Col span={7}>
                    <form.Field
                        name={`auto[${index}].value`}
                        children={(field) => {
                            return <TextInput
                                label="Value"
                                placeholder="Some value"
                                id={field.name}
                                name={field.name}
                                value={field.state.value}
                                error={
                                    !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                }
                                onBlur={field.handleBlur}
                                onChange={(e) => {
                                    field.handleChange(e.currentTarget.value);
                                }}
                                withAsterisk
                            />
                        }}
                    />
                </Grid.Col>
            </Grid>
        </>
    }

    const notification = () => {
        if (hasError) {
            return <Alert
                mt="sm"
                variant='light'
                color='red'
                title='Error while creating'
                data-cy="successfulUpdate"
                onClose={ () => setHasError(false) }
                withCloseButton
            >
                There was an error while creating the tag
            </Alert>;
        }
    }

    return <>
        { notification() }

        <form
            onSubmit={(e) => {
                e.preventDefault()
                e.stopPropagation()
                form.handleSubmit()
            }}
        >
            <Stack>
                <form.Field
                    name="content"
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
                                data-cy="content"
                                label="Content"
                                description="Tag content"
                                placeholder="New Tag"
                                id={field.name}
                                name={field.name}
                                value={field.state.value}
                                error={
                                    !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                }
                                onBlur={field.handleBlur}
                                onChange={(e) => {
                                    field.handleChange(e.target.value);
                                }}
                            />
                        </>
                    }}
                />

                <form.Field
                    name="color"
                    children={(field) => {
                        return <>
                            <ColorInput
                                data-1p-ignore
                                data-cy="color"
                                label="Color"
                                description="Tag Color"
                                id={field.name}
                                name={field.name}
                                value={field.state.value}
                                error={
                                    !field.state.meta.isValid && field.state.meta.errors.join(', ')
                                }
                                onBlur={field.handleBlur}
                                onChange={(e) => {
                                    field.handleChange(e);
                                }}
                            />
                        </>
                    }}
                />

                <form.Field
                    name="typ"
                    children={(field) => {
                        return <>
                            <InputWrapper
                                label="Type"
                                description="Select the type of Tag"
                            >
                                <Group>
                                    <SegmentedControl
                                        data={[{
                                            label: 'Manual',
                                            value: 'MANUAL'
                                        }, {
                                            label: 'Auto',
                                            value: 'AUTO'
                                        }]}
                                        value={field.state.value}
                                        onChange={(x: any) => {
                                            field.handleChange(x);
                                        }}
                                    />
                                </Group>
                            </InputWrapper>
                        </>
                    }}
                />

                <form.Subscribe
                    selector={(state) => [state.values.typ]}
                    children={([typ]) => {
                        if (typ !== 'AUTO') {
                            return <></>;
                        }

                        return <>
                            <form.Field name="auto" mode="array">
                                {(field) => {
                                    return <>
                                        <Stack>
                                            {field.state.value.map((_, i) => autoConfiguration(i))}

                                            <Group justify="flex-end">
                                                <Button
                                                    onClick={() => field.pushValue({
                                                        select: 'PROJECT_NAME',
                                                        compare: 'IS',
                                                        value: ''
                                                    })}
                                                >
                                                    Add another one
                                                </Button>
                                            </Group>
                                        </Stack>
                                    </>
                                }}
                            </form.Field>
                        </>
                    }}
                />

                <form.Subscribe
                    selector={(state) => [state.values.content, state.values.color]}
                    children={([content, color]) => <InputWrapper
                            label="Example Tag"
                        >
                            <Group>
                                <BadgeWrapper
                                    color={color}
                                >
                                    {content}
                                </BadgeWrapper>
                            </Group>
                        </InputWrapper>
                    }
                />

                <form.Subscribe
                    selector={(state) => [state.canSubmit, state.isSubmitting]}
                    children={([canSubmit, isSubmitting]) => (
                        <Group
                            justify="flex-end"
                            gap="sm"
                        >
                            <Button
                                data-cy="create"
                                mt="sm"
                                type="submit"
                                disabled={!canSubmit}
                                loading={isSubmitting}
                                onClick={() => {
                                    form.handleSubmit()
                                }}
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

export type CreateTagProps = {
    onCreate: () => void
}
