<template>
    <n-select
        :loading="busy"
        :options="options"
        :value="projectId"
        @update:value="updateValue"
        clearable
        data-cy="ProjectSelector"
        filterable
        placeholder="Select project"
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NText, NSelect, type SelectOption } from 'naive-ui';
import { Project, ProjectService } from '@/sdk/project';

import type { ProjectUuid } from '@/sdk/utils';

// Usage:
//
// ```html
// <project-group-selector
//     v-model:value="data.project_group"
// />
// ```
//
// Returns a UUID of the project
//
@Component({
    components: {
        NText,
        NSelect,
    },
    emits: [
        'error',
        'update:projectId',
    ],
})
class ProjectSelector extends Vue {
    @Prop({
        required: false,
    })
    public projectId!: ProjectUuid;

    public options: SelectOption[] = [];
    public value: ProjectUuid | null = this.projectId;

    public busy: boolean = false;

    public async created() {
        this.busy = true;
        await ProjectService
            .list({
                status: 'IN_PROGRESS'
            })
            .then((x: Project[]) => {
                this.busy = false;

                x.map((x: Project) => {
                    this.options.push({
                        label: x.name,
                        value: x.id,
                    });
                });
            })
            .catch(e => {
                this.busy = false;
                this.$emit('error', e)
            });
    }

    public updateValue(value: ProjectUuid | null) {
        console.log(value)
        this.value = value;
        this.$emit('update:projectId', value);
    }
}

export default toNative(ProjectSelector);
</script>
