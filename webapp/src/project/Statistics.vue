<template>
    <div>
        <page-header title="RCIMade In Numbers" />

        <loader description="Loading statistics just for you" :busy=busy />

        <div v-if="!busy">
            <card title="General Statistics" id="GeneralStatistics">
                <n-row style="margin-left: 10px">
                    <n-col :span="5" style="margin-top: 5px">
                        <n-statistic label="Build caps">
                            <format-number :value="statistics.build_caps" />
                        </n-statistic>
                    </n-col>
                    <n-col :span="5" style="margin-top: 5px">
                        <n-statistic label="Build fittings">
                            <format-number :value="statistics.build_fittings" />
                        </n-statistic>
                    </n-col>
                    <n-col :span="5" style="margin-top: 5px">
                        <n-statistic label="Top Month Caps">
                            <format-number :value="statistics.top_month_count.count" /> in {{ format_date(statistics.top_month_count.month) }}
                        </n-statistic>
                    </n-col>
                </n-row>

                <n-row style="margin-left: 10px; margin-top: 25px">
                    <n-col :span="5" style="margin-top: 5px">
                        <n-statistic label="Reaction jobs started">
                            <format-number :value="statistics.jobs_reaction" />
                        </n-statistic>
                    </n-col>
                    <n-col :span="5" style="margin-top: 5px">
                        <n-statistic label="Manufacturing jobs started">
                            <format-number :value="statistics.jobs_manufacturing" />
                        </n-statistic>
                    </n-col>
                    <n-col :span="5" style="margin-top: 5px">
                        <n-statistic label="Total started jobs">
                            <format-number :value="statistics.jobs_reaction + statistics.jobs_manufacturing" />
                        </n-statistic>
                    </n-col>
                </n-row>

                <n-row style="margin-left: 10px; margin-top: 25px">
                    <n-col :span="5" style="margin-top: 5px">
                        <n-statistic label="Hauling cost">
                            <format-number :value="statistics.hauling_cost" />
                        </n-statistic>
                    </n-col>
                    <n-col :span="5" style="margin-top: 5px">
                        <n-statistic label="Reprocessing cost">
                            <format-number :value="statistics.reprocessing_cost" />
                        </n-statistic>
                    </n-col>
                    <n-col :span="5" style="margin-top: 5px">
                        <n-statistic label="Blueprint cost">
                            <format-number :value="statistics.blueprint_cost" />
                        </n-statistic>
                    </n-col>
                </n-row>

                <n-row style="margin-left: 10px; margin-top:25px">
                    <n-col :span="5" style="margin-top: 5px">
                        <n-statistic label="Material cost">
                            <format-number :value="statistics.cost_materials" />
                        </n-statistic>
                    </n-col>
                    <n-col :span="5" style="margin-top: 5px">
                        <n-statistic label="Industry Job Cost (includes SCC)">
                            <format-number :value="statistics.cost_jobs" />
                        </n-statistic>
                    </n-col>
                    <n-col :span="5" style="margin-top: 5px">
                        <n-statistic label="Total cost">
                            <format-number :value="statistics.blueprint_cost + statistics.hauling_cost + statistics.reprocessing_cost + statistics.cost_materials + statistics.cost_jobs" />
                        </n-statistic>
                    </n-col>
                </n-row>

                <n-row style="margin-left: 10px; margin-top:25px">
                    <n-col :span="12" style="margin-top: 5px">
                        <n-statistic label="Build hulls">
                            <v-chart class="chart" :option="caps_by_type" />
                        </n-statistic>
                    </n-col>
                </n-row>
            </card>

            <card-margin />

            <card title="Cost breakdown by dread">
                <n-row
                    style="margin-left: 10px"
                    v-for="(group, index) in cost_by_dread_order"
                    :key="index"
                >
                    <n-collapse>
                        <n-collapse-item :title="group.header">
                            <template
                                :key="entry.type_id"
                                v-for="(entry, index) in group.entries"
                            >
                                <template v-if="statistics.cost_by_dread[entry.type_id] && statistics.cost_by_dread[entry.type_id].length > 0">
                                    <h1>
                                        <div style="display: flex;">
                                            <eve-icon
                                                :id="entry.type_id"
                                                :width="48"
                                                style="margin-right: 5px"
                                            />
                                            <div>
                                                {{ entry.name }}
                                            </div>
                                        </div>
                                    </h1>
                                    <n-col :span="24">
                                        <v-chart
                                            class="chart_line"
                                            :option="stacked_line_chart(statistics.cost_by_dread[entry.type_id])"
                                        />
                                    </n-col>
                                </template>
                            </template>
                        </n-collapse-item>
                    </n-collapse>
                </n-row>

                <n-row>
                    <h1>
                        <div style="display: flex;">
                            <div>
                                All
                            </div>
                        </div>
                    </h1>
                    <n-col :span="24">
                        <v-chart
                            class="chart_line_all"
                            :option="stacked_line_chart(statistics.cost_by_dread[0])"
                        />
                    </n-col>
                </n-row>
            </card>

            <card-margin />

            <card title="Consumed Raw Materials">
                <n-row
                    style="margin-left: 10px"
                    v-for="group in statistics.consumed_raw_materials"
                    :key="group.header"
                >
                    <n-collapse>
                        <n-collapse-item :title="group.header">
                            <n-col
                                :span="24"
                            >
                                <h1>{{ group.header }}</h1>

                                <p v-if="group.header === 'Minerals'">
                                    At some point we switched over to using compressed ore.
                                    So the graphs, "Total Spend" and "Average Cost" are not 100% correct.
                                    <br>
                                    The numbers for "Total used" are correct, they take reprocessed ores into account.
                                </p>
                            </n-col>

                            <template
                                :key="material.type_id"
                                v-for="(material, index) in group.entries"
                            >
                                <n-divider v-if="index > 0" />

                                <n-col :span="5">
                                    <h3>
                                        <div style="display: flex;">
                                            <eve-icon
                                                :id="material.type_id"
                                                :width="24"
                                                style="margin-right: 5px"
                                            />
                                            <div>
                                                {{ material.item_name }}
                                            </div>
                                        </div>
                                    </h3>

                                    <n-statistic label="Total used">
                                        <format-number :value="material.quantity" />
                                    </n-statistic>

                                    <n-statistic label="Total spend (ISK)">
                                        <format-number :value="material.cost" />
                                    </n-statistic>

                                    <n-statistic label="Average cost per unit (ISK)">
                                        <format-number :value="material.cost / material.quantity" />
                                    </n-statistic>
                                </n-col>

                                <n-col :span="19">
                                    <v-chart
                                        class="chart_line"
                                        :option="default_material_chart(statistics.raw_materials_by_month[material.type_id])"
                                    />
                                </n-col>
                            </template>
                        </n-collapse-item>
                    </n-collapse>
                </n-row>
            </card>

            <card-margin />

            <card title="Produced Components">
                <n-row
                    style="margin-left: 10px"
                    v-for="group in statistics.produced_components"
                    :key="group.header"
                >
                    <n-collapse>
                        <n-collapse-item :title="group.header">
                            <n-col
                                :span="24"
                            >
                                <h1 style="margin-bottom: 0px;">{{ group.header }}</h1>
                            </n-col>

                            <template
                                :key="material.type_id"
                                v-for="(material, index) in group.entries"
                            >
                                <n-divider v-if="index > 0" />

                                <n-col :span="5">
                                    <h3>
                                        <div style="display: flex;">
                                            <eve-icon
                                                :id="material.type_id"
                                                :width="24"
                                                style="margin-right: 5px"
                                            />
                                            <div>
                                                {{ material.item_name }}
                                            </div>
                                        </div>
                                    </h3>

                                    <n-statistic label="Total produced">
                                        <format-number :value="material.quantity" />
                                    </n-statistic>

                                    <n-statistic label="Total runs">
                                        <format-number :value="material.runs" />
                                    </n-statistic>

                                    <n-statistic label="Average job cost per unit (ISK)">
                                        <format-number :value="average_component_cost(material.type_id)" />
                                    </n-statistic>
                                </n-col>

                                <n-col :span="19">
                                    <v-chart
                                        class="chart_line"
                                        :option="default_material_chart(statistics.components_by_month[material.type_id])"
                                    />
                                </n-col>
                            </template>
                        </n-collapse-item>
                    </n-collapse>
                </n-row>
            </card>
        </div>
    </div>
</template>

<script lang="ts">
import axios from 'axios';

import { Component, Vue, toNative } from 'vue-facing-decorator';
import { NCol, NCollapse, NCollapseItem, NDivider, NImage, NRow, NStatistic } from 'naive-ui';

import { events } from '@/main';
import { ROUTE_CHANGE } from '@/event_bus';

import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { LineChart, BarChart, PieChart } from "echarts/charts";
import {
  AxisPointerComponent,
  GridComponent,
  LegendComponent,
  MarkLineComponent,
  TitleComponent,
  TooltipComponent,
} from "echarts/components";
import VChart from "vue-echarts";

import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import EveIcon from '@/components/EveIcon.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import Loader from '@/components/Loader.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NCol,
        NCollapse,
        NCollapseItem,
        NDivider,
        NImage,
        NRow,
        NStatistic,
        VChart,

        Card,
        CardMargin,
        EveIcon,
        FormatNumber,
        Loader,
        PageHeader,
    }
})
class ProjectsView extends Vue {
    public busy = false;

    public statistics: IStatistics = <any>{};

    public caps_by_type = {};

    public number_formatter = Intl.NumberFormat('en', { notation: 'compact' });

    public cost_by_dread_order = [{
        header: 'Faction Dreads',
        entries: [{
            name:    'Zirnitra',
            type_id: 52907,
        }, {
            name:    'Revelation Navy Issue',
            type_id: 73790,
        }, {
            name:    'Naglfar Fleet Issue',
            type_id: 73787,
        }, {
            name:    'Phoenix Navy Issue',
            type_id: 73793,
        }, {
            name:    'Moros Navy Issue',
            type_id: 73792,
        }],
    }, {
        header: 'Dreads',
        entries: [{
            name:    'Revelation',
            type_id: 19720,
        }, {
            name:    'Naglfar',
            type_id: 19722,
        }, {
            name:    'Phoenix',
            type_id: 19726,
        },  {
            name:    'Moros',
            type_id: 19724,
        }]
    }, {
        header: 'T2 Dreads',
        entries: [{
            name:    'Bane',
            type_id: 77283,
        }, {
            name:    'Valravn',
            type_id: 77288,
        }, {
            name:    'Karura',
            type_id: 77284,
        }, {
            name:    'Hubris',
            type_id: 77281,
        }]
    }, {
        header: 'FAX',
        entries: [{
            name:    'Apostle',
            type_id: 37604,
        }, {
            name:    'Lif',
            type_id: 37606,
        }, {
            name:    'Minokawa',
            type_id: 37605,
        }, {
            name:    'Ninazu',
            type_id: 37607,
        }]
    }, {
        header: 'Carrier',
        entries: [{
            name:    'Archon',
            type_id: 23757,
        }, {
            name:    'Nidhoggur',
            type_id: 24483,
        }, {
            name:    'Chimera',
            type_id: 23915,
        }, {
            name:    'Thanatos',
            type_id: 23911,
        }]
    }, {
        header: 'Supers',
        entries: [{
            name:    'Aeon',
            type_id: 23919,
        }, {
            name:    'Hel',
            type_id: 22852,
        }, {
            name:    'Wyvern',
            type_id: 23917,
        }, {
            name:    'Nyx',
            type_id: 23913,
        }, ]
    }, {
        header: 'Titans',
        entries: [{
            name:    'Avatar',
            type_id: 11567,
        }, {
            name:    'Ragnarok',
            type_id: 23773,
        }, {
            name:    'Leviathan',
            type_id: 3764,
        }, {
            name:    'Erebus',
            type_id: 671,
        }]
    }, {
        header: 'Jump Freighter',
        entries: [{
            name:    'Ark',
            type_id: 28850,
        }, {
            name:    'Nomad',
            type_id: 28846,
        }, {
            name:    'Rhea',
            type_id: 28844,
        }, {
            name:    'Obelisk',
            type_id: 20187,
        }]
    }, {
        header: 'Other',
        entries: [{
            name:    'Rorqual',
            type_id: 28352,
        }]
    }];

    public async created() {
        events.$emit(
            ROUTE_CHANGE,
            this.$route.name
        );

        this.busy = true;
        this.statistics = (await axios.get<IStatistics>('/api/v1/projects/statistics')).data;
        this.busy = false;

        use([
            AxisPointerComponent,
            BarChart,
            CanvasRenderer,
            GridComponent,
            LegendComponent,
            LineChart,
            MarkLineComponent,
            PieChart,
            TitleComponent,
            TooltipComponent,
        ]);

        this.caps_by_type = this.default_pie_chart(this.statistics.caps_buid_by_type);
    }

    public format_date(date: string): string {
        return new Date(date).toLocaleString('en-EN', {
            month: 'long'
        });
    }

    public average_material_cost(
        type_id: number,
    ): number {
        let reduced = this.statistics
            .raw_materials_by_month[type_id]
            .reduce((prev: any, curr: any) => {
                prev.cost += curr.cost;
                prev.quantity += curr.quantity;
                return prev;
            }, { cost: 0, quantity: 0 });

        return reduced.cost / reduced.quantity;
    }

    public average_component_cost(
        type_id: number,
    ): number {
        let reduced = this.statistics
            .components_by_month[type_id]
            .reduce((prev: any, curr: any) => {
                if (!curr.runs) {
                    return prev;
                }

                prev.cost += curr.cost;
                prev.runs += curr.runs;
                return prev;
            }, { cost: 0, runs: 0 });

        return reduced.cost / reduced.runs;
    }

    public default_material_chart(data: IMarketByMonth[] | IComponentsByMonth[]) {
        if (data.length !== 12) {
            if (!data.find(x => x.month === 1)) {
                data.push(<any>{ month: 1, cost: 0, quantity: 0 });
            }
            if (!data.find(x => x.month === 2)) {
                data.push(<any>{ month: 2, cost: 0, quantity: 0 });
            }
            if (!data.find(x => x.month === 3)) {
                data.push(<any>{ month: 3, cost: 0, quantity: 0 });
            }
            if (!data.find(x => x.month === 4)) {
                data.push(<any>{ month: 4, cost: 0, quantity: 0 });
            }
            if (!data.find(x => x.month === 5)) {
                data.push(<any>{ month: 5, cost: 0, quantity: 0 });
            }
            if (!data.find(x => x.month === 6)) {
                data.push(<any>{ month: 6, cost: 0, quantity: 0 });
            }
            if (!data.find(x => x.month === 7)) {
                data.push(<any>{ month: 7, cost: 0, quantity: 0 });
            }
            if (!data.find(x => x.month === 8)) {
                data.push(<any>{ month: 8, cost: 0, quantity: 0 });
            }
            if (!data.find(x => x.month === 9)) {
                data.push(<any>{ month: 9, cost: 0, quantity: 0 });
            }
            if (!data.find(x => x.month === 10)) {
                data.push(<any>{ month: 10, cost: 0, quantity: 0 });
            }
            if (!data.find(x => x.month === 11)) {
                data.push(<any>{ month: 11, cost: 0, quantity: 0 });
            }
            if (!data.find(x => x.month === 12)) {
                data.push(<any>{ month: 12, cost: 0, quantity: 0 });
            }
        }

        data.sort(
            (
                a: IMarketByMonth | IComponentsByMonth,
                b: IMarketByMonth | IComponentsByMonth
            ) => a.month - b.month
        );

        return {
            darkMode: true,
            tooltip: {
                trigger: 'axis'
            },
            legend: {
                textStyle: {
                    color: 'white'
                }
            },
            xAxis: {
                type: 'category',
                data: ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'],
            },
            yAxis: [{
                id:   0,
                type: 'value',
                axisLabel: {
                    formatter: (val: number) => this.number_formatter.format(val),
                },
            }, {
                id:   1,
                type: 'value',
                axisLabel: {
                    formatter: (val: number) => this.number_formatter.format(val),
                },
            }],
            series: [
                {
                    name: 'Average cost per unit / month',
                    type: 'line',
                    data: data
                        .map(x => {
                            if (x.cost === 0) {
                                return '-';
                            }

                            return Math.floor((x.cost / x.quantity) * 100) / 100;
                        }),
                },
                {
                    yAxisIndex: 1,
                    name: 'Used by month',
                    type: 'bar',
                    data: data
                        .map(x => {
                            if (x.cost === 0) {
                                return '-';
                            }

                            return x.quantity;
                        }),
                }
            ]
        };
    }

    public stacked_line_chart(data: ICostByDread[]) {
        if (!data) {
            return;
        }

        return {
            darkMode: true,
            tooltip: {
                trigger: 'axis',
                axisPointer: {
                    label: {
                        formatter: (params: any): string => {
                            let item = data
                                .find(x => x.project_name === params.value) || {
                                    item_name: 'Unknown'
                                };

                            return `${params.value} (${item.item_name})`
                        }
                    }
                }
            },
            legend: {
                textStyle: {
                    color: 'white'
                }
            },
            xAxis: {
                type: 'category',
                data: data.map(x => x.project_name),
            },
            yAxis: [{
                id:   0,
                type: 'value',
                axisLabel: {
                    formatter: (val: number) => this.number_formatter.format(val),
                },
            }],
            series: [
                {
                    yAxisIndex: 0,
                    name: 'Market',
                    type: 'line',
                    stack: 'Total',
                    areaStyle: {},
                    data: data
                        .map(x => {
                            if (x.market === 0) {
                                return '-';
                            }

                            return x.market;
                        }),
                },
                {
                    yAxisIndex: 0,
                    name: 'Hauling',
                    type: 'line',
                    stack: 'Total',
                    areaStyle: {},
                    data: data
                        .map(x => {
                            if (x.hauling === 0) {
                                return '-';
                            }

                            return x.hauling;
                        }),
                },
                {
                    yAxisIndex: 0,
                    name: 'Reprocessing',
                    type: 'line',
                    stack: 'Total',
                    areaStyle: {},
                    data: data
                        .map(x => {
                            if (x.reprocessing === 0) {
                                return '-';
                            }

                            return x.reprocessing;
                        }),
                },
                {
                    yAxisIndex: 0,
                    name: 'BPCs',
                    type: 'line',
                    stack: 'Total',
                    areaStyle: {},
                    data: data
                        .map(x => {
                            if (x.bpc === 0) {
                                return '-';
                            }

                            return x.bpc;
                        }),
                },
                {
                    yAxisIndex: 0,
                    name: 'Reactions',
                    type: 'line',
                    stack: 'Total',
                    areaStyle: {},
                    data: data
                        .map(x => {
                            if (x.reactions === 0) {
                                return '-';
                            }

                            return x.reactions;
                        }),
                },
                {
                    yAxisIndex: 0,
                    name: 'Manufacturing',
                    type: 'line',
                    stack: 'Total',
                    areaStyle: {},
                    data: data
                        .map(x => {
                            if (x.manufacturing === 0) {
                                return '-';
                            }

                            return x.manufacturing;
                        }),
                },
            ]
        };
    }

    private default_pie_chart(data: any) {
        return {
            darkMode: true,
            tooltip: {},
            legend: {
                textStyle: {
                    color: 'white'
                },
                left: 'center',
            },
            series: [
                {
                    type: 'pie',
                    data,
                }
            ]
        };
    }
}

interface IStatistics {
    build_caps:             number;
    build_fittings:         number;

    top_month_count:        { month: string, count: number };

    jobs_reaction:          number,
    jobs_manufacturing:     number,

    cost_jobs:              number;
    cost_materials:         number;

    caps_buid_by_type:      { name: string, value: number }[];

    consumed_raw_materials: { header: string, entries: IConsumedMaterials[] }[];
    produced_components:    { header: string, entries: IProducedComponents[] }[];

    raw_materials_by_month: { [key: number]: IMarketByMonth[] };
    components_by_month:    { [key: number]: IComponentsByMonth[] };

    cost_by_dread:          { [key: number]: ICostByDread[] };

    blueprint_cost:         number;
    hauling_cost:           number;
    reprocessing_cost:      number;
}

interface IConsumedMaterials {
    item_name:   string;
    quantity:    number;
    type_id:     number;
    group_id:    number;
    category_id: number;
    cost:        number;
}

interface IMarketByMonth {
    month:     number;
    item_name: string;
    quantity:  number;
    type_id:   number;
    cost:      number;
}

interface IProducedComponents {
    item_name:   string;
    quantity:    number;
    type_id:     number;
    group_id:    number;
    category_id: number;
    cost:        number;
    runs:        number;
}

interface IComponentsByMonth {
    month:     number;
    item_name: string;
    runs:      number;
    quantity:  number;
    type_id:   number;
    cost:      number;
}

interface ICostByDread {
    project_name:  string;
    item_name:     string;
    reactions:     number;
    manufacturing: number;
    market:        number;
    hauling:       number,
    reprocessing:  number,
    bpc:           number,
}

export default toNative(ProjectsView);
</script>

<style scoped>
.chart {
    height: 700px;
}

.chart_line {
    height: 250px;
}

.chart_line_all {
    height: 500px;
}
</style>
