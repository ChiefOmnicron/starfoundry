<template>
    <div>
        <page-header title="Industry Jobs" />

        <loader description="Loading industry jobs" :busy="busy" />

        <n-text v-if="!busy">Active jobs: {{ jobs.length }}</n-text>

        <n-card content-style="padding: 0" v-if="!busy">
            <n-space justify="end" style="margin: 10px">
                <n-button @click="refresh" :loading="refresh_active">
                    Refresh
                </n-button>
            </n-space>

            <n-table v-if="!busy && jobs.length > 0">
                <thead>
                    <tr>
                        <th width="40px"></th>
                        <th width="250px">Activity</th>
                        <th width="500px">Name</th>
                        <th width="100px">Runs</th>
                        <th width="200px">Remaining</th>
                        <th width="300px">End date (EVE-Time)</th>
                        <th width="200px" style="text-align: right">Cost</th>
                        <th width="30px"></th>
                        <th width="400px">Owner</th>
                    </tr>
                </thead>
                <tbody>
                    <template v-for="j in jobs" :key="j.job_id">
                        <tr>
                            <td>
                                <eve-icon :id="j.blueprint_type_id" type="bp" />
                            </td>
                            <td>
                                {{ j.activity_id }}
                            </td>
                            <td>
                                <item :tid="j.blueprint_type_id">
                                    <template v-slot="{ item }">
                                        {{ rename(item.name) }}
                                    </template>
                                </item>
                            </td>
                            <td>
                                {{ j.runs }}
                            </td>
                            <td>
                                <n-tag v-if="j.remaining === 0" type="success"
                                    >Done</n-tag
                                >
                                <format-number
                                    v-if="(j.remaining || 0) > 0"
                                    :value="j.remaining || 0"
                                    time
                                />
                            </td>
                            <td>
                                <n-button
                                    text
                                    tag="a"
                                    target="_blank"
                                    type="primary"
                                    :href="
                                        'https://time.nakamura-labs.com/?#' +
                                        new Date(j.end_date).valueOf() / 1000
                                    "
                                >
                                    {{ format_end_date(j.end_date) }}
                                </n-button>
                            </td>
                            <td style="text-align: right">
                                <format-number :value="j.cost" />
                                ISK
                            </td>
                            <td>
                                <eve-icon
                                    v-if="!j.corporation_id"
                                    :id="j.installer_id"
                                    character
                                />
                                <eve-icon
                                    v-if="j.corporation_id"
                                    :id="j.corporation_id"
                                    corporation
                                />
                            </td>
                            <td>
                                <character-info
                                    v-if="!j.corporation_id"
                                    :character-id="j.installer_id"
                                >
                                    <template v-slot="{ info }">
                                        {{ info.character }}
                                    </template>
                                </character-info>

                                <div v-if="j.corporation_id">
                                    <corporation-info
                                        :corporation-id="j.corporation_id"
                                    >
                                        <template v-slot="{ info }">
                                            {{ info.corporation }}
                                        </template>
                                    </corporation-info>
                                    <character-info
                                        :character-id="j.installer_id"
                                    >
                                        <template v-slot="{ info }">
                                            {{ info.character }}
                                        </template>
                                    </character-info>
                                </div>
                            </td>
                        </tr>
                    </template>
                </tbody>
            </n-table>

            <no-entries
                description="No active jobs"
                v-if="!busy && jobs.length === 0"
            />
        </n-card>
    </div>
</template>

<script lang="ts">
import { events } from '@/main';
import {
    NButton,
    NCard,
    NCountdown,
    NSpace,
    NTable,
    NTag,
    NText,
} from 'naive-ui';
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { ROUTE_CHANGE } from '@/event_bus';
import { formatDate } from '@/utils';

import { Service, type IIndustryJob } from '@/industry/service';

import CharacterInfo from '@/characters/components/CharacterInfo.vue';
import CorporationInfo from '@/characters/components/CorporationInfo.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import Item from '@/components/Item.vue';
import EveIcon from '@/components/EveIcon.vue';
import Loader from '@/components/Loader.vue';
import NoEntries from '@/components/NoEntries.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NButton,
        NCard,
        NCountdown,
        NSpace,
        NTable,
        NTag,
        NText,

        CharacterInfo,
        CorporationInfo,
        FormatNumber,
        Item,
        EveIcon,
        Loader,
        NoEntries,
        PageHeader,
    },
})
class IndustryJobs extends Vue {
    public busy: boolean = false;
    public refresh_active: boolean = false;

    public jobs: IIndustryJob[] = [];
    public selected: number = 0;

    public async created() {
        events.$emit(ROUTE_CHANGE, this.$route.name);

        this.busy = true;
        await this.load();
        this.busy = false;

        this.timerRefresh();
    }

    public format_end_date(input: string): string {
        return formatDate(new Date(input).valueOf());
    }

    public timerRefresh() {
        setInterval(() => {
            for (let i = 0; i < this.jobs.length; i++) {
                // Typescript hates calculating dates
                const start: any = new Date();
                const end: any = new Date(this.jobs[i].end_date);
                const remaining = Math.floor((end - start) / 1000);
                this.jobs[i].remaining = remaining > 0 ? remaining : 0;
            }
        }, 1000);
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
        this.refresh_active = true;
        await this.load();
        this.refresh_active = false;
    }

    private async load() {
        Service.character_jobs()
            .then((x) =>
                x.sort((a: IIndustryJob, b: IIndustryJob) =>
                    a.end_date.localeCompare(b.end_date),
                ),
            )
            .then(
                (x) =>
                    (this.jobs = x.map((y) => {
                        switch (y.activity_id) {
                            case 'MATERIAL_EFFICIENCY_RESEARCH':
                                y.activity_id = 'ME Research';
                                break;
                            case 'TIME_EFFICIENCY_RESEARCH':
                                y.activity_id = 'TE Research';
                                break;
                            case 'REACTIONS':
                                y.activity_id = 'Reaction';
                                break;
                            case 'MANUFACTURING':
                                y.activity_id = 'Manufacturing';
                                break;
                            case 'COPYING':
                                y.activity_id = 'Copying';
                                break;
                            default:
                                y.activity_id = y.activity_id;
                        }
                        return y;
                    })),
            );
    }
}

export default toNative(IndustryJobs);
</script>
