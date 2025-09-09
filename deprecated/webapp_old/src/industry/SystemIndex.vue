<template>
    <div>
        <page-header title="System Index" />

        <system-selector v-model:value="system_id" />

        <card-margin />

        <loader :busy="busy" description="Loading index" />

        <card title="System Index" v-if="system_id && !busy">
            <v-chart class="chart" :option="chart_options" />
        </card>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative, Watch } from 'vue-facing-decorator';
import { events } from '@/main';
import { ROUTE_CHANGE } from '@/event_bus';

import { use } from 'echarts/core';
import { CanvasRenderer } from 'echarts/renderers';
import { LineChart } from 'echarts/charts';
import {
    AxisPointerComponent,
    GridComponent,
    LegendComponent,
    MarkLineComponent,
    TitleComponent,
    TooltipComponent,
} from 'echarts/components';
import { Service, type IIndustryIndex } from '@/industry/service';

import VChart from 'vue-echarts';
import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import Loader from '@/components/Loader.vue';
import PageHeader from '@/components/PageHeader.vue';
import SystemSelector from '@/components/selectors/SystemSelector.vue';

@Component({
    components: {
        VChart,

        Card,
        CardMargin,
        Loader,
        PageHeader,
        SystemSelector,
    },
})
class SystemIndex extends Vue {
    public chart_options = {};

    public system_id: number = <any>undefined;
    public busy: boolean = false;

    public async created() {
        events.$emit(ROUTE_CHANGE, this.$route.name);

        use([
            AxisPointerComponent,
            CanvasRenderer,
            GridComponent,
            LegendComponent,
            LineChart,
            MarkLineComponent,
            TitleComponent,
            TooltipComponent,
        ]);
    }

    @Watch('system_id')
    public async load_graph() {
        let azbel_marker = {
            data: [
                {
                    xAxis: 1680470259000,
                    label: {
                        formatter: 'Azbel Public',
                        textStyle: {
                            color: '#ccc',
                        },
                    },
                },
            ],
        };
        let tatara_marker = {
            data: [
                {
                    xAxis: 1681229973000,
                    label: {
                        formatter: 'Tatara Public',
                        textStyle: {
                            color: '#ccc',
                        },
                    },
                },
            ],
        };

        this.busy = true;
        let entries = await Service.industry_index(this.system_id);
        this.busy = false;

        this.chart_options = this.default_chart_config(entries);
        this.add_series(
            this.chart_options,
            entries.map((x: IIndustryIndex) => [
                x.timestamp,
                (x.reaction * 100).toFixed(2),
            ]),
            'Reaction',
        );
        this.add_series(
            this.chart_options,
            entries.map((x: IIndustryIndex) => [
                x.timestamp,
                (x.manufacturing * 100).toFixed(2),
            ]),
            'Manufacturing',
        );
        this.add_series(
            this.chart_options,
            entries.map((x: IIndustryIndex) => [
                x.timestamp,
                (x.invention * 100).toFixed(2),
            ]),
            'Invention',
        );
        this.add_series(
            this.chart_options,
            entries.map((x: IIndustryIndex) => [
                x.timestamp,
                (x.copying * 100).toFixed(2),
            ]),
            'Copying',
        );
        this.add_series(
            this.chart_options,
            entries.map((x: IIndustryIndex) => [
                x.timestamp,
                (x.research_material * 100).toFixed(2),
            ]),
            'Research Material',
        );
        this.add_series(
            this.chart_options,
            entries.map((x: IIndustryIndex) => [
                x.timestamp,
                (x.research_time * 100).toFixed(2),
            ]),
            'Research Time',
        );

        /*let entries = (await Service.industry_index(this.system_id))
            .sort((a: IIndustryIndex, b: IIndustryIndex) => a.timestamp - b.timestamp);

        this.reaction_options = this.default_chart_config(entries);
        this.add_series(
            this.reaction_options,
            entries
                .filter((x: IIndustryIndex) => x.system_id === 30002019)
                .filter((x: IIndustryIndex) => x.timestamp >= 1680307200)
                .map((x: IIndustryIndex) => [x.timestamp * 1000, (x.reaction * 100).toFixed(2)]),
            'F-NMX6',
            tatara_marker
        );
        this.add_series(
            this.reaction_options,
            entries
                .filter((x: IIndustryIndex) => x.system_id === 30003693)
                .filter((x: IIndustryIndex) => x.timestamp >= 1680307200)
                .map((x: IIndustryIndex) => [x.timestamp * 1000, (x.reaction * 100).toFixed(2)]),
            '0-ARFO',
            tatara_marker
        );

        this.manufacturing_options = this.default_chart_config(entries);
        this.add_series(
            this.manufacturing_options,
            entries
                .filter((x: IIndustryIndex) => x.system_id === 30002019)
                .filter((x: IIndustryIndex) => x.timestamp >= 1680307200)
                .map((x: IIndustryIndex) => [x.timestamp * 1000, (x.manufacturing * 100).toFixed(2)]),
            'F-NMX6',
            azbel_marker
        );
        this.add_series(
            this.manufacturing_options,
            entries
                .filter((x: IIndustryIndex) => x.system_id === 30003693)
                .filter((x: IIndustryIndex) => x.timestamp >= 1680307200)
                .map((x: IIndustryIndex) => [x.timestamp * 1000, (x.manufacturing * 100).toFixed(2)]),
            '0-ARFO',
            azbel_marker
        );*/
    }

    private add_series(chart: any, data: any, name: string) {
        chart.series.push({
            data,
            name,
            showSymbol: false,
            type: 'line',
        });
    }

    private default_chart_config(entries: IIndustryIndex[]) {
        return {
            tooltip: {
                trigger: 'axis',
            },
            legend: {
                show: true,
                type: 'plain',
                orient: 'vertical',
                left: 'left',
                top: 'middle',
                inactiveColor: '#333',
                textStyle: {
                    color: '#ccc',
                },
            },
            xAxis: {
                type: 'time',
                data: entries.map((x: IIndustryIndex) => x.timestamp / 1000),
            },
            yAxis: {
                type: 'value',
                min: 0,
                axisTick: {
                    show: false,
                    color: '#f00',
                },
            },
            series: [],
            useUTC: true,
        };
    }
}

export default toNative(SystemIndex);
</script>

<style scoped>
.chart {
    height: 300px;
}
</style>
