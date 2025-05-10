<template>
    <div>
        <page-header v-if="!busy" :title="project.name" />

        <loader :busy="busy" />

        <card title="General Information" v-if="!busy">
            <info-table :data="generalInfo(project)" :field-width="200" />
        </card>

        <card-margin />

        <card title="Finance" v-if="!busy">
            <info-table
                :data="financeInfo(project.finance)"
                :field-width="200"
            />
        </card>

        <card-margin />

        <card title="Products" v-if="!busy">
            <info-table :data="productInfo(project)" :field-width="200" />
        </card>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { type VNode, h } from 'vue';

import {
    Project,
    ProjectService,
    type IFinance,
    type IProduct,
} from '@/sdk/project';
import { type Uuid } from '@/sdk/utils';

import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import EveIcon from '@/components/EveIcon.vue';
import InfoTable from '@/components/InfoTable.vue';
import Loader from '@/components/Loader.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        Card,
        CardMargin,
        InfoTable,
        Loader,
        PageHeader,
    },
})
class ProjectOverview extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public projectId!: Uuid;

    public busy: boolean = false;

    public project: Project = <any>{};

    public async created() {
        console.log('b');
        this.busy = true;
        ProjectService.fetch(this.projectId)
            .then((x) => {
                this.project = x;
                document.title = this.project.name;
            })
            .catch((e) => {
                console.error(e);
            })
            .then((_) => (this.busy = false));
    }

    public generalInfo(project: Project) {
        return [
            {
                field: 'Orderer',
                value: project.orderer,
            },
            {
                field: 'Sell Price',
                value: project.price,
                is_number: true,
            },
            {
                field: 'Note',
                value: project.note,
            },
        ];
    }

    public financeInfo(finance: IFinance) {
        return [
            {
                field: 'Job Cost',
                value: finance.jobs,
                is_number: true,
            },
            {
                field: 'Market cost',
                value: finance.market,
                is_number: true,
            },
            {
                field: 'Miscellaneous costs',
                value: finance.misc,
                is_number: true,
            },
            {
                field: 'Excess Cost',
                value: finance.excess,
                is_number: true,
            },
            {
                field: 'Total Cost',
                value:
                    finance.jobs +
                    finance.market +
                    finance.misc -
                    finance.excess,
                is_number: true,
            },
            {
                field: 'Sell price',
                value: finance.sell_price,
                is_number: true,
            },
            {
                field: 'Profit',
                value:
                    (finance.sell_price || 0) -
                    (finance.jobs +
                        finance.market +
                        finance.misc -
                        finance.excess),
                is_number: true,
            },
        ];
    }

    public productInfo(project: Project) {
        console.log(project);
        return [
            {
                field: 'Products',
                value: project.products,
                table: [
                    {
                        header: '',
                        key: 'icon',
                        visible: true,
                        width: 25,
                        render(row: IProduct): VNode {
                            return h(EveIcon, {
                                id: row.type_id,
                            });
                        },
                    },
                    {
                        header: 'Name',
                        key: 'type_id',
                        visible: true,
                        item: true,
                        width: 500,
                    },
                    {
                        header: 'Quantity',
                        key: 'quantity',
                        visible: true,
                    },
                ],
            },
        ];
    }
}

export default toNative(ProjectOverview);
</script>
