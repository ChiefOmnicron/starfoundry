<template>
    <card
        v-for="job_list in job_groups"
        :key="job_list.header"
        style="margin-bottom: 10px"
        :title="header(job_list.header)"
    >
        <n-table>
            <thead>
                <tr>
                    <th width="32px"></th>
                    <th width="200px">Name</th>
                    <th width="125px">Runs</th>
                    <th width="250px">Status</th>
                    <th width="250px">Price</th>
                    <th width="110px"></th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="job in job_list.entries" :key="job.type_id">
                    <td>
                        <eve-icon :id="job.type_id" type="icon" item />
                    </td>
                    <td>
                        <copy-text :value="job.item_name" />
                    </td>
                    <td>
                        <format-number :value="job.runs" />
                    </td>
                    <td style="max-width: 250px;">
                        <n-select v-model:value="job.status" :options="status_options" />
                    </td>
                    <td>
                        <n-input-number v-model:value="job.cost" />
                    </td>
                    <td>
                        <n-button type="info" :quaternary="true" @click="updateJob(job)">Update</n-button>
                        <n-button type="error" :quaternary="true" @click="deleteJob(job)">Delete</n-button>
                    </td>
                </tr>
            </tbody>
        </n-table>
    </card>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NInputNumber, NSelect, NTable, NTag, type SelectOption } from 'naive-ui';

import type { ProjectUuid } from '@/sdk/utils';
import { Project, ProjectService, type IJob, type IJobGroup } from '@/sdk/project';

import Card from '@/components/Card.vue';
import CopyText from '@/components/CopyText.vue';
import EveIcon from '@/components/EveIcon.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import ItemSelector from '@/components/selectors/ItemSelector.vue';

@Component({
    components: {
        NButton,
        NInputNumber,
        NSelect,
        NTable,
        NTag,

        Card,
        CopyText,
        EveIcon,
        FormatNumber,
        ItemSelector,
    }
})
class ProjectJobOverview extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public projectId!: ProjectUuid;

    public buildable_items: SelectOption[] = [];
    public busy: boolean = true;
    public show_export: boolean = false;

    public job_lists: IJobGroup[] = [];

    public new_entries: IJob[] = [];
    public new_entry: IJob = <any>{};

    public buildable_entries = [];
    public job_groups: IJobGroup[] = [];

    public status_options = [{
        label: 'Waiting for Materials',
        value: 'WAITING_FOR_MATERIALS'
    }, {
        label: 'Building',
        value: 'BUILDING'
    }, {
        label: 'Done',
        value: 'DONE'
    }];

    private project!: Project;

    public async mounted() {
        this.busy = true;

        this.project = await ProjectService.fetch(this.projectId);

        this.job_groups = await this.project
            .fetchJobsGrouped({})
            .catch(e => {
                console.error(e);
                return [];
            });

        this.busy = false;
    }

    public async deleteJob(job: IJob) {
        await this.project.deleteJob(job.id);
        this.job_groups = await this.project.fetchJobsGrouped({});
    }

    public async updateJob(job: IJob) {
        await this.project.updateJob(job.id, job);
        this.job_groups = await this.project.fetchJobsGrouped({});
    }

    public header(header: string): string {
        switch(header) {
            case 'INTERMEDIATE_REACTIONS':
                return 'Intermediate Reactions'
            case 'COMPOSITE_REACTIONS':
                return 'Composite Reactions'
            case 'BIOCHEM_REACTIONS':
                return 'Biochem Reactions'
            case 'HYBRID_REACTIONS':
                return 'Hybrid Reactions'
            case 'CONSTRUCTION_COMPONENTS':
                return 'Construction Components'
            case 'ADVANCED_CAPITAL_CONSTRUCTION_COMPONENTS':
                return 'Advanced Capital Construction Components'
            case 'CAPITAL_CONSTRUCTION_COMPONENTS':
                return 'Capital Construction Components'
            case 'TOOLS':
                return 'Tools'
            case 'T1_STUFF':
                return 'T1 Stuff'
            case 'T2_STUFF':
                return 'T2 Stuff'
            case 'CHARGES':
                return 'Charges'
            case 'SHIPS':
                return 'Ships'
            default:
                return 'Unknown'
        }
    }
}

export default toNative(ProjectJobOverview);
</script>
