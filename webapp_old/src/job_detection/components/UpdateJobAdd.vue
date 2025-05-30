<template>
    <div>
        <n-table>
            <tr>
                <th style="width: 100px">Project</th>

                <td style="padding: 0">
                    <project-selector
                        :project-id="projectId"
                        @update:project-id="updateProjectId"
                    />
                </td>
            </tr>
            <tr>
                <th>Structure</th>

                <td style="padding: 0">
                    <structure-selector v-model:value="structureId" />
                </td>
            </tr>
            <tr v-if="projectId">
                <th>Old Project</th>

                <td style="padding: 10px">
                    <n-switch v-model:value="deleteFromSource">
                        <template #checked>
                            Delete job from old Project
                        </template>
                        <template #unchecked>
                            Reset job in old Project
                        </template>
                    </n-switch>
                </td>
            </tr>
            <tr v-if="jobs.length > 0">
                <th>Existing Jobs</th>

                <td style="padding: 10px">
                    <template v-for="job in jobs" :key="job.id">
                        <project-job-card :job="job" />
                    </template>
                </td>
            </tr>
        </n-table>

        <card-margin />

        <action-group>
            <n-button
                :disabled="!projectId || !structureId"
                @click="add"
                type="info"
            >
                Add to project
            </n-button>
        </action-group>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { ProjectUuid, StructureId, TypeId } from '@/sdk/utils';
import { JobDetectionService } from '@/job_detection/service';

import { NButton, NSwitch, NTable } from 'naive-ui';
import { ProjectService, type IJob } from '@/sdk/project';
import ActionGroup from '@/components/ActionGroup.vue';
import CardMargin from '@/components/CardMargin.vue';
import ProjectJobCard from '@/components/ProjectJobCard.vue';
import ProjectSelector from '@/components/selectors/ProjectSelector.vue';
import StructureSelector from '@/components/selectors/StructureSelector.vue';

@Component({
    components: {
        NButton,
        NSwitch,
        NTable,

        ActionGroup,
        CardMargin,
        ProjectSelector,
        ProjectJobCard,
        StructureSelector,
    },
    emits: ['update:projectId'],
})
class UpdateJobAdd extends Vue {
    @Prop({
        type: String,
        required: false,
    })
    public projectId!: ProjectUuid | undefined;

    @Prop({
        type: Number,
        required: true,
    })
    public typeId!: TypeId;

    @Prop({
        type: Number,
        required: true,
    })
    public jobId!: TypeId;

    public jobs: IJob[] = [];
    public structureId: StructureId | null = null;
    public deleteFromSource: boolean = false;

    public created() {
        this.fetchJobs(this.projectId);
    }

    public async fetchJobs(projectId: ProjectUuid | undefined) {
        if (!projectId) {
            return;
        }

        (await ProjectService.fetch(projectId))
            .fetchJobs({
                type_id: this.typeId,
            })
            .then((x) => {
                this.jobs = x;
            });
    }

    public updateProjectId(value: ProjectUuid | undefined) {
        this.$emit('update:projectId', value);
        this.fetchJobs(value);
    }

    public async add() {
        if (!this.projectId) {
            return;
        }
        if (!this.structureId) {
            return;
        }

        await JobDetectionService.updateJobAdd(this.jobId, {
            delete_from_source: this.deleteFromSource,
            structure_id: this.structureId,
            target_project_uuid: this.projectId,
        });

        this.$emit('jobUpdated', true);
    }
}

export default toNative(UpdateJobAdd);
</script>
