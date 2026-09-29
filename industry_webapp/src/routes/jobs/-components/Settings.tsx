import { Button, Group, InputBase, InputWrapper, Stack, TreeSelect, type TreeNodeData } from "@mantine/core";
import type { TabFilterDefinition, TabFilterFilterDefinition } from "@starfoundry/components/misc";
import { TagSelector } from "@starfoundry/components/selectors";
import { useListTags } from "@starfoundry/components/services/tags/list";
import { useForm } from "@tanstack/react-form";

export function ProjectJobSetting({
    filter,
    onChange = () => {},
}: ProjectJobSettingProps) {
    const {
        data: tags
    } = useListTags({
        auto: true,
        manual: true,
    });

    const form = useForm({
        defaultValues: {
            filterName: filter?.filter_name || '',
            tags: (filter?.filters.find(x => x.key === 'tags')?.value as string || '').split(',').filter(x => x.length > 0),
        },
        onSubmit: async ({ value }) => {
            // TODO: create filter endpoint
            // TODO: create the filter
            // TODO: invalidate the tag filters
            const filterEntries: TabFilterFilterDefinition[] = [];

            if (value.tags && value.tags.length > 0) {
                filterEntries.push({
                    key: 'tags',
                    value: value.tags.join(','),
                })
            }

            const filterInfo: TabFilterDefinition = {
                filter_name: value.filterName,
                filters: filterEntries,
            }
            onChange(filterInfo)
        },
    });

    return <form
        onSubmit={(e) => {
            e.preventDefault();
            e.stopPropagation();
        }}
    >
        <Stack>
            <form.Field
                name="filterName"
                children={(field) => {
                    return <InputBase
                        id={field.name}
                        name={field.name}
                        value={field.state.value}
                        onBlur={field.handleBlur}
                        label="Filter Name"
                        onChange={(e) => field.handleChange(e.target.value)}
                        disabled={!!filter}
                    />
                }}
            />

            <TreeSelect
                label="Job Type"
                placeholder="Pick one or more"
                data={JOB_TYPES}
                mode="checkbox"
                defaultExpandAll
            />

            <form.Field
                name="tags"
                children={(field) => {
                    return <InputWrapper
                        label="Tags"
                        description="Filter by tags"
                    >
                        <TagSelector
                            onSelect={(tag) => {
                                if (field.state.value.includes(tag.id)) {
                                    field.handleChange(field.state.value.filter(x => x !== tag.id));
                                } else {
                                    field.handleChange([...field.state.value, tag.id]);
                                }
                            }}
                            tags={tags || []}
                            selected={field.state.value}
                        />
                    </InputWrapper>
                }}
            />

            <Group
                justify="flex-end"
            >
                <form.Subscribe
                    selector={(state) => [state.canSubmit]}
                    children={([canSubmit, isSubmitting]) => (
                        <>
                            <Button
                                onClick={() => {
                                    form.handleSubmit()
                                }}
                                disabled={!canSubmit || isSubmitting}
                            >
                                Add
                            </Button>
                        </>
                    )}
                />
            </Group>
        </Stack>
    </form>
}

export type ProjectJobSettingProps = {
    filter?: TabFilterDefinition,
    onChange?: (filter: TabFilterDefinition) => void,
}

const JOB_TYPES: TreeNodeData[] = [{
    value: 'REACTION',
    label: 'Reactions',
    children: [{
        value: 'INTERMEDIATE',
        label: 'Intermediate Materials',
    }, {
        value: 'COMPOSITE',
        label: 'Composite',
    }, {
        value: 'MOLECULAR',
        label: 'Molecular-Forged',
    }, {
        value: 'HYBRID',
        label: 'Hybrid Polymers',
    }],
}, {
    value: 'MANUFACTURING',
    label: 'Manufacturing',
    children: [{
        value: 'COMPONENTS',
        label: 'Components',
        children: [{
            value: 'CONSTRUCTION_COMPONENTS',
            label: 'Construction'
        }, {
            value: 'CAPITAL_CONSTRUCTION_COMPONENTS',
            label: 'Capital Construction'
        }, {
            value: 'ADVANCED_CAPITAL_CONSTRUCTION_COMPONENTS',
            label: 'Advanced Capital Construction'
        }, {
            value: 'STRUCTURE_COMPONENTS',
            label: 'Structure'
        }],
    }, {
        value: 'MODULES',
        label: 'Modules',
        children: [{
            value: 'T1_MODULE',
            label: 'T1 Modules'
        }, {
            value: 'T2_MODULE',
            label: 'T2 Modules'
        }, {
            value: 'RIGS',
            label: 'Rigs'
        }, {
            value: 'STRUCTURE_RIGS',
            label: 'Structure Rigs'
        }, {
            value: 'CHARGES',
            label: 'Charges'
        }, {
            value: 'TOOLS',
            label: 'Tools'
        }]
    }, {
        value: 'FINAL',
        label: 'Products',
        children: [{
            value: 'SHIPS',
            label: 'Ships'
        }, {
            value: 'STRUCTURES',
            label: 'Structures'
        }, {
            value: 'DEPLOYABLE',
            label: 'Deployable'
        }]
    }],
}];
