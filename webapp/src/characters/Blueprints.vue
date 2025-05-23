<template>
    <div>
        <page-header title="Character Blueprints" />

        <n-card content-style="padding: 0">
            <div style="margin: 10px" v-if="!busy">
                <filter-text
                    v-model:entries="blueprints"
                    :filters="filters"
                    :options="filterOptions"
                    style="width: 100%"
                />

                <filter-element
                    style="margin-top: 5px"
                    :filters="filters"
                    :options="filterOptions"
                />
            </div>

            <n-table v-if="!busy">
                <thead>
                    <tr>
                        <th width="100">TypeId</th>
                        <th width="34"></th>
                        <th width="500">Name</th>
                        <th width="100">Material</th>
                        <th width="100">Time</th>
                        <th width="200">Price</th>
                        <th width="200">Status</th>
                    </tr>
                </thead>
                <tbody>
                    <tr
                        v-for="entry in blueprints.slice(0, 50)"
                        :key="entry.type_id"
                    >
                        <td>{{ entry.type_id }}</td>
                        <td>
                            <eve-icon
                                type="bp"
                                :id="entry.type_id"
                                :width="32"
                            />
                        </td>
                        <td>{{ entry.name }}</td>
                        <td>{{ entry.me }}</td>
                        <td>{{ entry.te }}</td>
                        <td><format-number :value="entry.price" /> ISK</td>
                        <td>
                            <n-tag v-if="entry.stored" type="success"
                                >Stored</n-tag
                            >
                            <n-tag v-else type="error">Missing</n-tag>
                        </td>
                    </tr>
                </tbody>
            </n-table>

            <loader :busy="busy" />
        </n-card>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { NCard, NSpin, NTable, NTag } from 'naive-ui';

import { events } from '@/main';
import { ROUTE_CHANGE } from '@/event_bus';

import FilterText, { type IFilterOption } from '@/components/Filter.vue';
import FilterElement from '@/components/FilterElement.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import EveIcon from '@/components/EveIcon.vue';
import Loader from '@/components/Loader.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NCard,
        NSpin,
        NTable,
        NTag,

        FilterText,
        FilterElement,
        FormatNumber,
        EveIcon,
        Loader,
        PageHeader,
    },
})
class CharacterSettings extends Vue {
    public busy: boolean = false;

    public blueprints: IBluepintTotalEntry[] = [];

    public filters: any = {};
    public filterOptions: { [key: string]: IFilterOption } = {};

    public stats: any = {};

    public async created() {
        events.$emit(ROUTE_CHANGE, this.$route.name);

        this.busy = true;
        //this.blueprints = await Service.blueprints_total();

        this.filterOptions = {
            name: {
                label: 'Name',
            },
            status: {
                label: 'Status',
                options: ['Stored', 'Missing'],
            },
        };

        this.stats_calc();

        this.busy = false;
    }

    public stats_calc() {
        this.stats = {
            count: this.blueprints.length,
            stored: this.blueprints
                .filter((x) => x.stored)
                .reduce((a, b) => (a += 1), 0),
            missing: this.blueprints
                .filter((x) => !x.stored)
                .reduce((a, b) => (a += 1), 0),
            worth_stored: this.blueprints
                .filter((x) => x.stored)
                .reduce((a, b) => a + b.price, 0),
            worth_missing: this.blueprints
                .filter((x) => !x.stored)
                .reduce((a, b) => a + b.price, 0),
            worth_total: this.blueprints.reduce((a, b) => a + b.price, 0),
        };
    }
}

export interface IBluepintTotalEntry {
    name: string;
    type_id: number;
    price: number;
    me?: number;
    te?: number;
    stored: boolean;
}

export default toNative(CharacterSettings);
</script>
