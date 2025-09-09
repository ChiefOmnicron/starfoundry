<template>
    <n-grid v-if="job" :cols="24">
        <n-grid-item>
            <eve-icon :id="job.type_id" />
        </n-grid-item>
        <n-grid-item span="10">
            {{ job.item_name }}
        </n-grid-item>
        <n-grid-item span="2">
            {{ job.runs }}
        </n-grid-item>
        <n-grid-item span="10">
            {{ status(job.status) }}
        </n-grid-item>
    </n-grid>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NButton, NGrid, NGridItem } from 'naive-ui';
import EveIcon from '@/components/EveIcon.vue';
import type { IJob } from '@/sdk/project';

@Component({
    components: {
        NButton,
        NGrid,
        NGridItem,

        EveIcon,
    },
})
class ProjectJobCard extends Vue {
    @Prop({
        type: Object,
        required: true,
    })
    public job!: IJob;

    public status(status: string): string {
        switch (status) {
            case 'BUILDING':
                return 'Building';
            case 'WAITING_FOR_MATERIALS':
                return 'Waiting for Materials';
            case 'DONE':
                return 'Done';
            default:
                return 'Unknown';
        }
    }
}

export default toNative(ProjectJobCard);
</script>
