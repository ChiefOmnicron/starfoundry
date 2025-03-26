<template>
    <div>
        <page-header title="Job Detection Logs" />

        <loader description="Loading Projects" :busy="busy" />

        <action-group
                v-if="!busy && jobDetectionLogs.length > 0"
                justify="end"
            >
            <div>
                <n-button
                    @click="refresh()"
                    style="margin-bottom: 7px"
                    :disabled="refreshing"
                    :loading="refreshing"
                >
                    Refresh
                </n-button>
            </div>
        </action-group>

        <card v-if="!busy" no-title>
            <data-table
                :definitions="tableDefinition()"
                :entries="jobDetectionLogs"
                v-if="!busy && jobDetectionLogs.length > 0"
            />

            <no-entries
                description="No projects yet"
                v-if="!busy && jobDetectionLogs.length === 0"
            />
        </card>

        <n-modal
            v-model:show="showUpdateJob"
        >
            <card
                title="Update Job"
                style="width: 1000px"
            >
                <template #description>
                    <div style="margin-left: 10px; margin-top: 10px;">
                        To update a job you have different options.

                        <p> - 'Replace' - Replaces one or more job entries in the target project. If it's already assigned to a project, the old project will remove the link to the job so that it can be filled again.</p>
                        <p> - 'Add' - Adds it as a new entry to the project. If it's already assigned to a project, the old project will remove the link to the job so that it can be filled again.</p>
                        <p> - 'Delete' - Will delete the job from the project. The job in that project can then be filled again.</p>
                        <p> - 'Ignore' - Adds the job to an ignore list and will not attempt to reassign it.</p>
                    </div>
                </template>

                <div style="margin: 5px;">
                    <n-tabs type="line" animated>
                        <n-tab-pane name="replace" tab="Replace">
                            <update-job-replace
                                @jobUpdated="updatedJob()"
                                v-model:project-id="selectedProjectId"
                                :type-id="selectedTypeId"
                                :job-id="selectedJobId"
                            />
                        </n-tab-pane>
                        <n-tab-pane name="add" tab="Add">
                            <update-job-add
                                @jobUpdated="updatedJob()"
                                v-model:project-id="selectedProjectId"
                                :type-id="selectedTypeId"
                                :job-id="selectedJobId"
                            />
                        </n-tab-pane>
                        <n-tab-pane name="delete" tab="Delete">
                            <update-job-delete
                                @jobUpdated="updatedJob()"
                                :job-id="selectedJobId"
                            />
                        </n-tab-pane>
                    </n-tabs>
                </div>
            </card>
        </n-modal>
    </div>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Vue, toNative } from 'vue-facing-decorator';
import { h } from 'vue';

import type { JobUuid, ProjectUuid, TypeId } from '@/sdk/utils';

import { NButton, NModal, NTabs, NTabPane } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import Card from '@/components/Card.vue';
import Countdown from '@/components/Countdown.vue';
import DataTable, { type IDataTableDefinition } from '@/components/DataTable.vue';
import EveIcon from '@/components/EveIcon.vue';
import Loader from '@/components/Loader.vue';
import Nakamura from '@/components/Nakamura.vue';
import NoEntries from '@/components/NoEntries.vue';
import PageHeader from '@/components/PageHeader.vue';
import ProjectReference from '@/components/ProjectReference.vue';
import UpdateJobAdd from './components/UpdateJobAdd.vue';
import UpdateJobDelete from './components/UpdateJobDelete.vue';
import UpdateJobReplace from './components/UpdateJobReplace.vue';

@Component({
    components: {
        NButton,
        NModal,
        NTabs,
        NTabPane,

        ActionGroup,
        Card,
        Countdown,
        DataTable,
        Loader,
        Nakamura,
        NoEntries,
        PageHeader,
        ProjectReference,
        UpdateJobAdd,
        UpdateJobDelete,
        UpdateJobReplace,
    }
})
class ProjectsView extends Vue {
    public busy = false;
    public showUpdateJob = false;
    public refreshing = false;

    public jobDetectionLogs: IJobDetectionLog[] = [];

    public selectedProjectId!: ProjectUuid | undefined;
    public selectedTypeId!: TypeId | undefined;
    public selectedJobId!: JobUuid | undefined;

    private refreshIntervalId!: number;

    public async created() {
        this.busy = true;

        axios
            .get(`/api/v1/job-detection`)
            .then(x => x.data)
            .then(x => {
                this.jobDetectionLogs = x;

                this.busy = false;
            });

        this.timerRefresh();
    }

    public unmounted() {
        clearInterval(this.refreshIntervalId);
    }

    public timerRefresh() {
        // refresh the shown data every 10 seconds
        this.refreshIntervalId = <any>setInterval(() => {
            this.refreshing = true;
            this.refresh();
        }, 10000);
    }

    public async refresh() {
        axios
            .get(`/api/v1/job-detection`)
            .then(x => x.data)
            .then(x => {
                this.jobDetectionLogs = x;

                this.busy = false;
                this.refreshing = false;
            });
    }

    public async updatedJob() {
        await this.refresh();
        this.showUpdateJob = false;
    }

    public tableDefinition(): IDataTableDefinition[] {
        return [{
            header: '',
            key: 'icon',
            width: 40,
            render(row) {
                return h(
                    EveIcon,
                    {
                        id: row.type_id,
                        type: 'icon',
                    }
                )
            },
        }, {
            header: 'Name',
            key: 'type_id',
            width: 500,
            item: true,
            copy: true,
        }, {
            header: 'Runs',
            key: 'runs',
            width: 200,
            number: true,
            copy: true,
        }, {
            header: 'Time Remaining',
            key: 'remaining',
            width: 200,
            visible: true,
            render(row: IJobDetectionLog) {
                return h(
                    Countdown,
                    {
                        endDate: row.end_date,
                    }
                )
            }
        }, {
            header: 'End Date',
            key: 'end_date',
            width: 200,
            visible: true,
            render(row: IJobDetectionLog) {
                return h(
                    Nakamura,
                    {
                        endDate: row.end_date,
                    }
                )
            }
        }, {
            header: 'Assigned Project',
            key: 'project_uuid',
            width: 200,
            render: (row: IJobDetectionLog, _: number) => {
                if (row.project_uuid) {
                    return h(
                        ProjectReference,
                        {
                            projectId: row.project_uuid,
                        },
                    );
                }

                return h(
                    'label',
                    {},
                    () => 'Not assigned'
                );
            }
        }, {
            header: '',
            key: 'edit',
            visible: true,
            width: 50,
            render: (row: IJobDetectionLog, _: number) => {
                return h(
                    NButton,
                    {
                        type: 'info',
                        quaternary: true,
                        onClick: () => {
                            this.selectedProjectId = row.project_uuid,
                            this.selectedTypeId = row.type_id;
                            this.selectedJobId = row.job_id;
                            this.showUpdateJob = true;
                        }
                    },
                    () => 'Edit'
                );
            }
        }];
    }
}

export default toNative(ProjectsView);

interface IJobDetectionLog {
    type_id:       TypeId;
    runs:          number;
    end_date:      string;
    job_id:        JobUuid;
    project_uuid?: ProjectUuid;
}
</script>
