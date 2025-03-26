<template>
    <wrapper
        :projectId="$route.params.projectId"
        header="Jobs"
        ref="project_wrapper_ref"
    >
        <n-tabs type="line" v-if="!busy">
            <n-tab-pane name="ready" tab="Ready to start" v-if="ready_jobs">
                <action-group v-if="ready_jobs.length > 0">
                    <n-button @click="create_job_assignment" :disabled="selected_jobs.length === 0">
                        Create Build order
                    </n-button>
                    <n-button
                        @click="show_check_ressources_modal = true"
                        :disabled="selected_jobs.length === 0"
                        style="margin-bottom: 7px"
                    >
                        Check Resources
                    </n-button>
                    <n-button @click="refresh">
                        Refresh
                    </n-button>
                    <n-button @click="show_export = true">
                        Export
                    </n-button>
                </action-group>

                <card no-title v-if="ready_jobs && ready_jobs.length > 0">
                    <project-job-list
                        :jobs="ready_jobs"
                        v-model:selected_ids="selected_jobs"
                    />
                </card>

                <no-entries
                    description="Nothing to do"
                    v-if="!busy && ready_jobs.length === 0"
                />
            </n-tab-pane>
            <n-tab-pane
                tab="Active Jobs"
                name="active_jobs"
                v-if="project_wrapper_ref"
            >
                <project-active-jobs :project-id="projectId" />
            </n-tab-pane>
            <n-tab-pane name="all_jobs" tab="All jobs" v-if="project_wrapper_ref">
                <project-job-overview :project-id="projectId" />
            </n-tab-pane>
        </n-tabs>

        <item-export-modal
            :data="ready_jobs"
            :dataFields="['item.name', 'runs']"
            :dataFieldsIngame="['item.name', 'type_id']"
            :show="show_export"
            @close="show_export = false"
            v-if="ready_jobs"
        />
        <check-resources-modal
            v-if="show_check_ressources_modal"
            :show="show_check_ressources_modal"
            :project-job-ids="selected_jobs"
            @close="show_check_ressources_modal = false"
        />
    </wrapper>
</template>

<script lang="ts">
import { Component, Prop, Ref, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NInputNumber, NSelect, NTabs, NTabPane, NTable, NTag, type SelectOption } from 'naive-ui';
import { Service, type IJob, type IJobGroup } from '@/project/service';
import { ProjectService } from '@/sdk/project';
import { ROUTE_PROJECT_ASSIGNMENTS } from '@/project/router';
import { type Uuid } from '@/sdk/utils';

import Wrapper from '@/project/components/Wrapper.vue';

import ActionGroup from '@/components/ActionGroup.vue';
import Card from '@/components/Card.vue';
import CheckResourcesModal from '@/project/components/CheckResourcesModal.vue';
import EditableNumberInput from '@/components/inputs/EditableNumber.vue';
import EditableStatusSelector from '@/project/components/EditableStatusSelector.vue';
import EveIcon from '@/components/EveIcon.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import ItemExportModal from '@/components/ItemExportModal.vue';
import ItemSelector from '@/components/selectors/ItemSelector.vue';
import NoEntries from '@/components/NoEntries.vue';
import ProjectActiveJobs from '@/project/job/ActiveJobs.vue';
import ProjectJobList from '@/project/job/JobList.vue';
import ProjectJobOverview from '@/project/job/Overview.vue';

@Component({
    components: {
        NButton,
        NInputNumber,
        NSelect,
        NTabs,
        NTabPane,
        NTable,
        NTag,

        ActionGroup,
        Card,
        CheckResourcesModal,
        EditableNumberInput,
        EditableStatusSelector,
        EveIcon,
        FormatNumber,
        ItemExportModal,
        ItemSelector,
        NoEntries,
        ProjectActiveJobs,
        ProjectJobList,
        ProjectJobOverview,
        Wrapper,
    }
})
class JobsView extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public projectId!: Uuid;

    @Ref('project_wrapper_ref')
    // wrapper
    public project_wrapper_ref!: any;

    public buildable_items: SelectOption[] = [];
    public busy: boolean = true;

    public job_lists: IJobGroup[] = [];

    public new_entries: IJob[] = [];
    public new_entry: IJob = <any>{};

    public old_entries: IJobGroup[] = [];

    public ready_jobs: IJob[] = <any>null;
    public selected_jobs: any[] = [];

    public show_export = false;
    public show_check_ressources_modal: boolean = false;

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

    public async created() {
        this.busy = true;
        this.ready_jobs = await Service
            .fetch_jobs_by_status(this.projectId);
        this.busy = false;
    }

    public async mounted() {
        this.busy = true;
        await this.project_wrapper_ref.jobs();
        this.old_entries = this.project_wrapper_ref.job_groups;
        this.busy = false;
    }

    public async refresh() {
        this.busy = true;
        this.ready_jobs = await Service
            .fetch_jobs_by_status(this.projectId);
        this.busy = false;
    }

    public async create_job_assignment() {
        const assignment_id = await ProjectService.create_job_assignment(
            this.selected_jobs
        );
        this.$router.push({
            name: ROUTE_PROJECT_ASSIGNMENTS,
            params: {
                assignment_id
            }
        });
    }
}

export default toNative(JobsView);
</script>
