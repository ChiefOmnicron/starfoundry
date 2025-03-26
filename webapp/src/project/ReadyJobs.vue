<template>
    <div>
        <page-header title="Industry Jobs Over All Projects" />

        <loader
            description="Loading"
            :busy="busy"
        />

        <n-tabs type="line" v-if="!busy">
            <n-tab-pane name="ready" tab="Ready to start">
                <n-space
                    v-if="!busy && projects.length > 0"
                    justify="space-between"
                >
                    <span>Number of startable jobs: {{ count_startable_jobs(projects) }}</span>

                    <div>
                        <n-button
                            @click="create_job_assignment"
                            :disabled="selected_jobs.length === 0"
                            style="margin-bottom: 7px"
                        >
                            Create Job assignment
                        </n-button>
                        <n-button
                            @click="show_check_ressources_modal = true"
                            :disabled="selected_jobs.length === 0"
                            style="margin-bottom: 7px"
                        >
                            Check Resources
                        </n-button>
                    </div>
                </n-space>

                <div>
                    <template v-for="(project, index) in projects" :key="project.id">
                        <card
                            v-if="!busy && (project.jobs || []).length > 0"
                            content-style="padding: 0"
                            style="margin-bottom: 10px"
                            :title="project.name"
                        >
                            <template #title>
                                <project-reference :project-id="project.id" header />
                            </template>

                            <template #action>
                                <n-button @click="show_export_modal(project.jobs)">
                                    Export
                                </n-button>
                                <n-button @click="project.jobs = []">
                                    Hide all
                                </n-button>
                            </template>


                            <card no-title>
                                <project-job-list
                                    :jobs="project.jobs"
                                    v-model:selected_ids="selected_jobs[index]"
                                />
                            </card>
                        </card>
                    </template>
                </div>
            </n-tab-pane>
            <n-tab-pane
                tab="Active Jobs"
                name="active_jobs"
                v-if="!busy && projects.length > 0"
            >
                <n-space
                    v-if="!busy && projects.length > 0"
                    justify="end"
                >
                    <div>
                        <n-button
                            @click="refreshActiveJobs()"
                            style="margin-bottom: 7px"
                        >
                            Refresh
                        </n-button>
                    </div>
                </n-space>

                <template v-for="project in projects" :key="project.id">
                    <project-active-jobs
                        :project-id="project.id"
                        :with-project-name="true"
                        style="margin-bottom: 10px"
                        hide-if-empty
                    />
                </template>
            </n-tab-pane>
        </n-tabs>

        <no-entries
            description="No jobs ready to start. Either create more projects, or wait for the current jobs to finsih"
            v-if="!busy && projects.length === 0"
        />
        <item-export-modal
            :data="selected_project"
            :dataFields="['item.name', 'runs']"
            :dataFieldsIngame="['item.name', 'type_id']"
            :show="show_export"
            @close="close_export_modal"
        />
        <check-resources-modal
            v-if="show_check_ressources_modal"
            :show="show_check_ressources_modal"
            :project-job-ids="selected_jobs.flat()"
            @close="show_check_ressources_modal = false"
        />
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { NAnchor, NAnchorLink, NButton, NSpace, NTabs, NTabPane } from 'naive-ui';
import { Service, type IProject, type IJob } from '@/project/service';
import { events } from '@/main';
import { REFRESH, ROUTE_CHANGE } from '@/event_bus';
import { ProjectService } from '@/sdk/project';
import { ROUTE_PROJECT_ASSIGNMENTS } from '@/project/router';

import ActionGroup from '@/components/ActionGroup.vue';
import Card from '@/components/Card.vue';
import CheckResourcesModal from '@/project/components/CheckResourcesModal.vue';
import ItemExportModal from '@/components/ItemExportModal.vue';
import Loader from '@/components/Loader.vue';
import NoEntries from '@/components/NoEntries.vue';
import PageHeader from '@/components/PageHeader.vue';
import ProjectActiveJobs from '@/project/job/ActiveJobs.vue';
import ProjectJobList from '@/project/job/JobList.vue';
import ProjectReference from '@/components/ProjectReference.vue';

@Component({
    components: {
        NAnchor,
        NAnchorLink,
        NButton,
        NSpace,
        NTabs,
        NTabPane,

        ActionGroup,
        Card,
        CheckResourcesModal,
        ItemExportModal,
        Loader,
        NoEntries,
        PageHeader,
        ProjectActiveJobs,
        ProjectJobList,
        ProjectReference,
    }
})
class ProjectsView extends Vue {
    public busy: boolean = false;

    public projects: IProjectWithJobs[] = [];
    public count: number                = 0;

    public show_export: boolean = false;
    public selected_project: IJob[] = [];
    public selected_jobs: any[] = [];

    public show_check_ressources_modal: boolean = false;

    public async created() {
        this.busy = true;
        await this.load();

        events.$emit(
            ROUTE_CHANGE,
            this.$route.name
        );
        this.busy = false;
    }

    private async load() {
        this.projects = <any>await ProjectService
            .list({
                status: 'IN_PROGRESS',
            })
            .catch(e => {
                console.error(e);
                return [];
            });

        for (let project of this.projects) {
            project.jobs = await Service
                .fetch_jobs_by_status(project.id);
        }
    }

    public count_startable_jobs(projects: IProjectWithJobs[]): number {
        return projects
            .map(x => (x.jobs || []).length)
            .reduce((prev: number, curr: number) => prev += curr, 0);
    }

    public show_export_modal(data: IJob[]) {
        this.selected_project = data;
        this.show_export = true;
    }

    public close_export_modal() {
        this.show_export = false;
        this.selected_project = [];
    }

    public async create_job_assignment() {
        const assignment_id = await ProjectService.create_job_assignment(
            this.selected_jobs.flat()
        );
        this.$router.push({
            name: ROUTE_PROJECT_ASSIGNMENTS,
            params: {
                assignment_id
            }
        });
    }

    public refreshActiveJobs() {
        events.$emit(REFRESH, true);
    }
}

interface IProjectWithJobs extends IProject {
    jobs: IJob[];
}

export default toNative(ProjectsView);
</script>
