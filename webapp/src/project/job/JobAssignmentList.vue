<template>
    <n-table>
        <thead>
            <tr>
                <th width="1%"></th>
                <th width="25%">Name</th>
                <th width="20%">Runs</th>
                <th width="30%">Structure</th>
                <th></th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="job in jobs" :key="job.job_id">
                <td>
                    <eve-icon :id="job.type_id" type="icon" item />
                </td>
                <td>
                    <copy-text
                        :value="job.item_name"
                        :disabled="job.started"
                    />
                </td>
                <td>
                    <copy-text
                        :value="job.runs"
                        :disabled="job.started"
                        number
                    />
                </td>
                <td :style="{
                    color: job.started ? 'rgba(255, 255, 255, 0.2)' : ''
                }">
                    {{ job.structure_name }}
                </td>
                <td>
                    <div style="display: flex; justify-content: flex-end">
                        <n-button
                            @click="started(job); job.started = true;"
                            v-if="!job.started"
                        >
                            Started
                        </n-button>

                        <n-tag
                            v-if="job.started"
                            type="success"
                        >Started</n-tag>
                    </div>
                </td>
            </tr>
        </tbody>
    </n-table>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NCheckbox, NTable, NTag } from 'naive-ui';
import { type JobAssignmentEntry, ProjectService } from '@/sdk/project';

import CopyText from '@/components/CopyText.vue';
import EveIcon from '@/components/EveIcon.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import StructureWrapper from '@/structure/components/Wrapper.vue';

@Component({
    components: {
        NButton,
        NCheckbox,
        NTable,
        NTag,

        CopyText,
        EveIcon,
        FormatNumber,
        StructureWrapper,
    },
    emits: ['update:selected_ids']
})
class ProjectJobAssignmentList extends Vue {
    @Prop({
        type: Array,
        required: true,
    })
    public jobs!: JobAssignmentEntry[];

    public async started(job: JobAssignmentEntry) {
        await ProjectService
            .update_job_assignment(
                <any>this.$route.params.assignment_id,
                job.job_id,
            )
            .then(_ => {
            })
            .catch(e => {
                console.error(e);
                return [];
            });
    }
}

export default toNative(ProjectJobAssignmentList);
</script>
