import { Alert, Button, ColorInput, Grid, Group, InputBase, InputWrapper, SegmentedControl, Select, Stack, TextInput } from "@mantine/core";
import { BadgeWrapper } from "@internal/wrapper/Badge";
import { LIST_TAGS, type AutoTagCompare, type AutoTagSelect, type Tag, type TagType } from "@internal/services/tags/list";
import { tagOptions } from "@internal/services/tags/options";
import { updateTag } from "@internal/services/tags/update";
import { useForm } from "@tanstack/react-form";
import { useMutation } from "@tanstack/react-query";
import { useState, type ReactElement } from "react";
import type { CreateTag } from "@internal/services/tags/create";

export function UpdateTag({
    tag,
    onUpdate,
}: UpdateTagProps): ReactElement {
    const [hasError, setHasError] = useState<boolean>(false);

    const updateTagMutation = useMutation({
        mutationFn: async (value: CreateTag) => {
            return await updateTag(tag.id, value)
        },
        onError: () => {
            setHasError(true);
        },
        onSuccess: (_data, _variables, _onMutateResult, context) => {
            context.client.invalidateQueries({ queryKey: [LIST_TAGS] });
            setHasError(false);
            onUpdate();
        },
    });

    const form = useForm({
        defaultValues: {
            content: tag.content,
            color: tag.color,
            typ: tag.typ,
            auto: tag.auto,
        },
        onSubmit: async ({ value }) => await updateTagMutation
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
                                data-cy="update"
                                mt="sm"
                                type="submit"
                                disabled={!canSubmit}
                                loading={isSubmitting}
                                onClick={() => {
                                    form.handleSubmit()
                                }}
                            >
                                Update
                            </Button>
                        </Group>
                    )}
                />
            </Stack>
        </form>
    </>
}

export type UpdateTagProps = {
    tag: Tag,
    onUpdate: () => void
}
