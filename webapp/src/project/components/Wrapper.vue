<template>
    <div>
        <page-header :title="project.name + ' ' + header" />

        <loader
            description="Loading Project"
            :busy="busy"
        />

        <slot v-if="!busy" :project="project"></slot>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { Service, type IJob, type IJobGroup } from '@/project/service';
import { Project, ProjectService } from '@/sdk/project';
import type { ProjectUuid } from '@/sdk/utils';

import Loader from '@/components/Loader.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        Loader,
        PageHeader,
    }
})
class ProjectServiceWrapper extends Vue {
    @Prop({
        type:     String,
        required: false,
    })
    public header!: string;

    // Project id
    @Prop({
        type: String,
        required: true,
    })
    public projectId!: string;

    public job_groups: IJobGroup[] = [];

    public project: Project = <any>{
        name: ''
    };

    public busy: boolean = true;

    public async created() {
        console.error('call of deprecated function ProjectService.created');
        await ProjectService
            .fetch(<ProjectUuid>this.$route.params.projectId)
            .then(x => {
                this.project = x;
                this.busy = false;
            })
            // TODO: show error + screen
            .catch(_ => this.busy = false);
    }

    // DEPRECATED
    public async jobs() {
        console.error('call of deprecated function ProjectService.jobs');
        this.job_groups = await Service.fetch_jobs(<ProjectUuid>this.projectId)
    }

    // DEPRECATED
    public async jobs_by_status(status: string): Promise<IJob[]> {
        console.error('call of deprecated function ProjectService.jobs_by_status');
        return await Service
            .fetch_jobs_by_status(<ProjectUuid>this.projectId);
    }
}

export default toNative(ProjectServiceWrapper);
</script>
