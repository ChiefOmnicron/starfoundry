<template>
    <div v-if="!(jobs.length === 0 && hideIfEmpty)">
        <loader description="Loading industry jobs" :busy=busy />

        <no-entries description="No active jobs" v-if="!busy && jobs.length === 0" />

        <card
            content-style="padding: 0"
            :no-title="!withProjectName"
            v-if="!busy && jobs.length > 0"
        >
            <template #title v-if="withProjectName">
                <project-reference :project-id="project.id" header />
            </template>

            <data-table
                :definitions="tableDefinition()"
                :entries="jobs"
                v-if="!busy && jobs.length > 0"
            />
        </card>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { h } from 'vue';

import { events } from '@/main';
import { REFRESH } from '@/event_bus';

import { Project, ProjectService, type IActiveJob } from '@/sdk/project';
import type { Uuid } from '@/sdk/utils';

import { NButton, NSpace, NTable, NTag, NText } from 'naive-ui';
import Card from '@/components/Card.vue';
import Countdown from '@/components/Countdown.vue';
import DataTable, { type IDataTableDefinition } from '@/components/DataTable.vue';
import EveIcon from '@/components/EveIcon.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import Loader from '@/components/Loader.vue';
import Nakamura from '@/components/Nakamura.vue';
import NoEntries from '@/components/NoEntries.vue';
import ProjectReference from '@/components/ProjectReference.vue';

@Component({
    components: {
        NButton,
        NSpace,
        NTable,
        NTag,
        NText,

        Card,
        Countdown,
        DataTable,
        EveIcon,
        FormatNumber,
        Loader,
        Nakamura,
        NoEntries,
        ProjectReference,
    }
})
class ActiveProjectJobs extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public projectId!: Uuid;

    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public withProjectName!: boolean;

    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public hideIfEmpty!: boolean;

    public busy: boolean = false;
    public jobs: IActiveJob[] = [];
    public selected: number = 0;

    public project!: Project;

    private refreshIntervalId!: number;

    public async created() {
        this.busy = true;
        ProjectService
            .fetch(this.projectId)
            .then(x => {
                this.project = x;
                this.busy = false;
            })
            .catch(e => {
                console.error(e);
            })
            .then(_ => this.loadJobs());

        this.timerRefresh();

        events.$on(REFRESH, () => this.refresh());
    }

    public unmounted() {
        clearInterval(this.refreshIntervalId);
    }

    public timerRefresh() {
        // refresh the shown data every 10 seconds
        this.refreshIntervalId = <any>setInterval(() => {
            this.refresh();
        }, 10000);
    }

    public rename(name: string): string {
        if (name) {
            return name
                .replace(' Blueprint', '')
                .replace(' Reaction Formula', '');
        } else {
            return '';
        }
    }

    public async refresh() {
        await this.loadJobs();
    }

    public renameActivity(activity: string): string {
        switch (activity) {
            case 'MATERIAL_EFFICIENCY_RESEARCH':
                return 'ME Research';
            case 'TIME_EFFICIENCY_RESEARCH':
                return 'TE Research';
            case 'REACTIONS':
                return 'Reaction';
            case 'MANUFACTURING':
                return 'Manufacturing';
            case 'COPYING':
                return 'Copying';
            default:
                return activity;
        };
    }

    public tableDefinition(): IDataTableDefinition[] {
        return [{
            header: '',
            key: 'icon',
            visible: true,
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
            header: 'Activity',
            key: 'activity',
            width: 200,
            visible: true,
            transform: (value: string): string => this.renameActivity(value),
        }, {
            header: 'Name',
            key: 'type_id',
            width: 400,
            visible: true,
            item: true,
            copy: true,
        }, {
            header: 'Runs',
            key: 'runs',
            width: 50,
            visible: true,
            copy: true,
        }, {
            header: 'Structure Name',
            key: 'structure_name',
            width: 350,
            visible: true,
        }, {
            header: 'Remaining',
            key: 'remaining',
            width: 200,
            visible: true,
            render(row: IActiveJob) {
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
            render(row: IActiveJob) {
                return h(
                    Nakamura,
                    {
                        endDate: row.end_date,
                    }
                )
            }
        }, {
            header: 'Cost',
            key: 'cost',
            width: 200,
            visible: true,
            number: true,
        }];
    }

    private async loadJobs() {
        this.project
            .activeJobs()
            .then(x => {
                this.jobs = x;
            });
    }
}

export default toNative(ActiveProjectJobs);
</script>
