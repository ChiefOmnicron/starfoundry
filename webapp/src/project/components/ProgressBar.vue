<template>
    <div style="width: 100%; overflow: hidden">
        <div v-for="job in jobs" :key="job.id"
            :style="{
                width:           (100 / jobs.length) + '%',
                backgroundColor: background_color_by_status(job),
            }"
            style="height: 15px; float: left"
        ></div>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { ProjectService, type IJob, type IJobGroup } from '@/sdk/project';

@Component({
    components: {}
})
class ProgressBar extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public projectId!: string;

    public jobs: IJob[]  = [];

    public async created() {
        await ProjectService
            .fetch(this.projectId)
            .then(x => x.fetchJobsGrouped({}))
            .then(x => this.process_jobs(x));
    }

    public background_color_by_status(job: IJob) {
        switch(job.status) {
            case "WAITING_FOR_MATERIALS":
                return 'rgba(232, 128, 128, 0.5)'
            case "BUILDING":
                return 'rgba(112, 192, 232, 0.5)'
            case "DONE":
                return 'rgba(99, 226, 183, 0.5)'
            default:
                return ''
        }
    }

    private process_jobs(groups: IJobGroup[]) {
        for (let group of groups) {
            this.jobs.push(...group.entries);
        }
    }
}

export default toNative(ProgressBar);
</script>
