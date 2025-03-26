<template>
    <div>
        <page-header title="Jobs ready to be started" />

        <loader
            description="Loading"
            :busy="busy"
        />

        <n-space v-if="!busy" justify="space-between">
            <span>Number of startable jobs: {{ jobs_to_start }}</span>

            <div>
                <n-button
                    @click="show_check_ressources_modal = true"
                    style="margin-bottom: 7px"
                >
                    Check Resources
                </n-button>
            </div>
        </n-space>

        <div>
            <template v-for="assignment in assignments" :key="assignment.header">
                <card
                    v-if="!busy && assignment.entries.length > 0"
                    content-style="padding: 0"
                    style="margin-bottom: 10px"
                    :title="assignment.header"
                >
                    <card no-title>
                        <project-job-assignment-list :jobs="assignment.entries" />
                    </card>
                </card>
            </template>
        </div>

        <check-resources-modal
            v-if="show_check_ressources_modal"
            :show="show_check_ressources_modal"
            :project-job-ids="job_ids"
            @close="show_check_ressources_modal = false"
        />
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { NAnchor, NAnchorLink, NButton, NSpace } from 'naive-ui';
import { type IJobAssignmentGroup, ProjectService } from '@/sdk/project';
import { type Uuid } from '@/sdk/utils';

import Card from '@/components/Card.vue';
import CheckResourcesModal from '@/project/components/CheckResourcesModal.vue';
import Loader from '@/components/Loader.vue';
import PageHeader from '@/components/PageHeader.vue';
import ProjectJobAssignmentList from '@/project/job/JobAssignmentList.vue';

@Component({
    components: {
        NAnchor,
        NAnchorLink,
        NButton,
        NSpace,

        Card,
        CheckResourcesModal,
        Loader,
        PageHeader,
        ProjectJobAssignmentList,
    }
})
class ProjectsView extends Vue {
    public busy: boolean = false;

    public assignments: IJobAssignmentGroup[] = [];
    public job_ids: Uuid[] = [];
    public jobs_to_start: number = 0;

    public timer!: number;

    public show_check_ressources_modal: boolean = false;

    public async created() {
        this.busy = true;
        await this.load();
        this.busy = false;

        this.timer = <any>setInterval(async () => {
            await this.load();
        }, 3000);
    }

    public beforeUnmount() {
        clearInterval(this.timer)
    }

    private async load() {
        this.job_ids = [];

        this.assignments = <any>await ProjectService
            .fetch_job_assignment(<any>this.$route.params.assignment_id)
            .then(entries => {
                this.jobs_to_start = 0;

                for (let entry of entries) {
                    this.jobs_to_start += entry.entries
                        .filter(x => !x.started)
                        .map(x => {
                            this.job_ids.push(x.job_id);

                            return x;
                        })
                        .length
                }

                return entries;
            })
            .catch(e => {
                console.error(e);
                return [];
            });
    }
}

export default toNative(ProjectsView);
</script>
