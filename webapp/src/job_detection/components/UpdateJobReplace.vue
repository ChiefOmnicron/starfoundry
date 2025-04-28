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
                        <n-grid :cols="24">
                            <n-grid-item>
                                <n-checkbox v-model:checked="job.selected" />
                            </n-grid-item>
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
                </td>
            </tr>
        </n-table>

        <card-margin />

        <action-group>
            <n-button
                :disabled="
                    !projectId ||
                    !structureId ||
                    jobs.filter((x) => x.selected).length === 0
                "
                @click="add"
                type="info"
            >
                Replace selected
            </n-button>
        </action-group>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { ProjectUuid, StructureId, TypeId } from '@/sdk/utils';
import { JobDetectionService } from '@/job_detection/service';

import {
    NButton,
    NCheckbox,
    NGrid,
    NGridItem,
    NSwitch,
    NTable,
} from 'naive-ui';
import { ProjectService, type IJob } from '@/sdk/project';
import ActionGroup from '@/components/ActionGroup.vue';
import CardMargin from '@/components/CardMargin.vue';
import ProjectJobCard from '@/components/ProjectJobCard.vue';
import ProjectSelector from '@/components/selectors/ProjectSelector.vue';
import StructureSelector from '@/components/selectors/StructureSelector.vue';
import EveIcon from '@/components/EveIcon.vue';

@Component({
    components: {
        NButton,
        NCheckbox,
        NGrid,
        NGridItem,
        NSwitch,
        NTable,

        ActionGroup,
        CardMargin,
        EveIcon,
        ProjectSelector,
        ProjectJobCard,
        StructureSelector,
    },
    emits: ['update:projectId'],
})
class UpdateJobReplace extends Vue {
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

    public jobs: IJobExpand[] = [];
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
                this.jobs = x.map(
                    (x) =>
                        <IJobExpand>{
                            selected: false,
                            ...x,
                        },
                );
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

        await JobDetectionService.updateJobReplace(this.jobId, {
            delete_from_source: this.deleteFromSource,
            job_uuids: this.jobs.filter((x) => x.selected).map((x) => x.id),
            structure_id: this.structureId,
            target_project_uuid: this.projectId,
        });

        this.$emit('jobUpdated', true);
    }

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

export default toNative(UpdateJobReplace);

interface IJobExpand extends IJob {
    selected: boolean;
}
</script>
