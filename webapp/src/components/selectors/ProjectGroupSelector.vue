<template>
    <n-select
        :disabled="readonly"
        :loading="busy"
        :multiple="multiple"
        :options="options"
        @update:value="updateValue"
        data-cy="ProjectGroupSelector"
        filterable
        placeholder="Select project group"
        v-model:value="value"
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NText, NSelect, type SelectOption } from 'naive-ui';
import { ProjectGroup, ProjectGroupService } from '@/project_group/service';

// Usage:
//
// ```html
// <project-group-selector
//     v-model:value="data.project_group"
// />
// ```
//
// Returns a UUID of the project group.
//
@Component({
    components: {
        NText,
        NSelect,
    },
    emits: ['error', 'update:value'],
})
class ProjectGroupSelector extends Vue {
    @Prop({
        required: false,
    })
    public groups!: string | string[];

    @Prop({
        type: String,
        required: false,
    })
    public projectPermission!: string;

    @Prop({
        type: String,
        required: false,
    })
    public structurePermission!: string;

    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public multiple!: boolean;

    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public skipDefault!: boolean;

    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public readonly!: boolean;

    public options: SelectOption[] = [];
    public value: string | string[] | null = this.groups;

    public busy: boolean = false;

    public async created() {
        if (!this.skipDefault) {
            this.options = [
                {
                    label: 'Default',
                    value: '00000000-0000-0000-0000-000000000000',
                },
            ];
        }

        this.busy = true;
        await ProjectGroupService.list({
            projects: this.projectPermission || 'READ,WRITE',
            structures: this.structurePermission || 'READ,WRITE',
        })
            .then((x: ProjectGroup[]) => {
                this.busy = false;

                x.map((x: ProjectGroup) => {
                    this.options.push({
                        label: x.name,
                        value: x.id,
                    });
                });
            })
            .catch((e) => {
                this.busy = false;
                this.$emit('error', true);
            });
    }

    public updateValue(value: string | string[] | null) {
        this.value = value;
        this.$emit('update:value', this.value);
    }
}

export default toNative(ProjectGroupSelector);
</script>
