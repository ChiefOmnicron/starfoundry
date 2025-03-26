<template>
    <span v-if="project">
        <slot :project="project"></slot>
    </span>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { Project, ProjectService } from '@/sdk/project';
import type { ProjectUuid } from '@/sdk/utils';

@Component({
    components: {},
    emits: [
        'loading',
    ]
})
class ProjectWrapper extends Vue {
    // Type-Id of the item to resolve
    @Prop({
        type:     String,
        required: true,
    })
    public projectUuid!: ProjectUuid;

    public project: Project = <any>{};

    public async created() {
        this.$emit('loading', true);

        await ProjectService
            .fetch(this.projectUuid)
            .then(x => {
                this.project = x;

                this.$emit('loading', false);
            })
            .catch(e => {
                this.$emit('loading', false);
            });
    }
}

export default toNative(ProjectWrapper);
</script>
