import { createTag, type AutoTagCompare, type AutoTagSelect, type CreateTag, type TagType } from "@internal/services/tags/create";
import { Button, ColorInput, Grid, Group, InputBase, InputWrapper, Pill, SegmentedControl, Select, Stack, TextInput } from "@mantine/core";
import { useForm } from "@tanstack/react-form";
import { useMutation } from "@tanstack/react-query";
import { useState, type ReactElement } from "react";

export function CreateTag(): ReactElement {
    const [tagColor, setTagColor] = useState<string>(randomColor());
    const [tagContent, setTagContent] = useState<string>('New Tag');
    const [tagType, setTagType] = useState<TagType>('MANUAL');

    const createTagMutation = useMutation({
        mutationFn: async (value: CreateTag) => {
            console.log(value)
            return await createTag(value)
        },
    });

    const form = useForm({
        defaultValues: {
            content: tagContent,
            color: tagColor,
            typ: 'MANUAL',
            auto: {
                compare: 'IS',
                select: 'PROJECT_NAME',
                value: '',
            },
        },
        onSubmit: async ({ value }) => await createTagMutation
            .mutateAsync({
                color:      value.color,
                content:    value.content,
                typ:        value.typ as TagType,

                auto:       value.typ === 'AUTO'
                            ?   {
                                    compare:    (value.auto.compare || 'IS') as AutoTagCompare,
                                    select:     (value.auto.select || 'PROJECT_NAME') as AutoTagSelect,
                                    value:      value.auto.value || '',
                                }
                            :   undefined
            })
            .then(x => {
                console.log(x)
            })
            .catch(e => {
                console.error(e)
            }),
    });

    const autoConfiguration = () => {
        if (tagType !== 'AUTO') {
            return <></>;
        }

        const optionsProject = [{
            label:  'project.name',
            value:  'PROJECT_NAME',
        }, {
            label:  'project.orderer',
            value:  'PROJECT_ORDERER',
        }, {
            label:  'project.note',
            value:  'PROJECT_NOTE',
        }, {
            label:  'project.product',
            value:  'PROJECT_PRODUCT',
        }, {
            label:  'project.status',
            value:  'PROJECT_STATUS',
        }];

        const optionsCompare = [{
            label:  'is',
            value:  'IS',
        }, {
            label:  'is not',
            value:  'IS_NOT',
        }, {
            label:  'contains',
            value:  'CONTAINS',
        }, {
            label:  'pattern',
            value:  'PATTERN',
        }];

        return <>
            <Grid>
                <Grid.Col span={3}>
                    <form.Field
                        name="auto.select"
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
                                data={optionsProject}
                                withAsterisk
                            />
                        }}
                    />
                </Grid.Col>

                <Grid.Col span={2}>
                    <form.Field
                        name="auto.compare"
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
                                data={optionsCompare}
                                withAsterisk
                            />
                        }}
                    />
                </Grid.Col>
                <Grid.Col span={7}>
                    <form.Field
                        name="auto.value"
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

    return <>
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
                                    setTagContent(e.target.value);
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
                                    setTagColor(e);
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
                                            setTagType(x);
                                        }}
                                    />
                                </Group>
                            </InputWrapper>
                        </>
                    }}
                />

                { autoConfiguration() }

                <InputWrapper
                    label="Example Tag"
                >
                    <Group>
                        <Pill
                            style={{
                                backgroundColor: form.getFieldValue('color'),
                            }}
                        >
                            {form.getFieldValue('content')}
                        </Pill>
                    </Group>
                </InputWrapper>

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

const randomColor = () => {
    return '#' + Math.floor(Math.random() * 16777215).toString(16);
}
