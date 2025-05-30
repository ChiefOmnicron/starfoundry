<template>
    <div>
        <page-header title="Cost Estimate" />

        <div v-show="!invalid_setup">
            <card no-title>
                <n-form
                    style="
                        margin-left: 10px;
                        margin-right: 10px;
                        margin-top: 10px;
                    "
                >
                    <n-form-item label="Structure Group">
                        <structure-group-selector
                            v-model:value="structure_group"
                            @no-structures="no_structures"
                        />
                    </n-form-item>

                    <n-form-item label="Stock">
                        <n-input
                            v-model:value="stock"
                            type="textarea"
                            placeholder="Insert your Stock"
                            rows="10"
                        />
                    </n-form-item>
                </n-form>

                <product-selector v-model:products="products_input" />
            </card>

            <card-margin />

            <card v-if="cost_estimate" no-title>
                <div style="margin: 10px">
                    Market:
                    <format-number
                        :value="cost_estimate.market_cost_total"
                    /><br />
                    Manufacturing:
                    <format-number
                        :value="cost_estimate.manufacturing_cost_total"
                    /><br />
                    Excess:
                    <format-number
                        :value="cost_estimate.excess_cost_total"
                    /><br />
                    Total:
                    <format-number
                        :value="
                            cost_estimate.market_cost_total +
                            cost_estimate.manufacturing_cost_total
                        "
                    /><br />
                    Total (-excess):
                    <format-number
                        :value="
                            cost_estimate.market_cost_total +
                            cost_estimate.manufacturing_cost_total -
                            cost_estimate.excess_cost_total
                        "
                    /><br />
                    Profit (5%):
                    <format-number
                        :value="
                            (cost_estimate.market_cost_total +
                                cost_estimate.manufacturing_cost_total) *
                            1.05
                        "
                    /><br />
                    Profit (7.5%):
                    <format-number
                        :value="
                            (cost_estimate.market_cost_total +
                                cost_estimate.manufacturing_cost_total) *
                            1.075
                        "
                    /><br />
                    Profit (10%):
                    <format-number
                        :value="
                            (cost_estimate.market_cost_total +
                                cost_estimate.manufacturing_cost_total) *
                            1.1
                        "
                    /><br />
                    Profit (-excess + 5%):
                    <format-number
                        :value="
                            (cost_estimate.market_cost_total +
                                cost_estimate.manufacturing_cost_total -
                                cost_estimate.excess_cost_total) *
                            1.05
                        "
                    /><br />
                    Profit (-excess + 7.5%):
                    <format-number
                        :value="
                            (cost_estimate.market_cost_total +
                                cost_estimate.manufacturing_cost_total -
                                cost_estimate.excess_cost_total) *
                            1.075
                        "
                    /><br />
                    Profit (-excess + 10%):
                    <format-number
                        :value="
                            (cost_estimate.market_cost_total +
                                cost_estimate.manufacturing_cost_total -
                                cost_estimate.excess_cost_total) *
                            1.1
                        "
                    /><br />
                </div>
            </card>

            <card-margin />

            <n-space justify="end">
                <n-button @click="$router.back()" quaternary>Cancel</n-button>

                <n-button
                    @click="create_estimate"
                    type="info"
                    :disabled="busy"
                    :loading="busy"
                >
                    Create estimate
                </n-button>
            </n-space>

            <card-margin />

            <card
                title="Excess"
                v-show="
                    !invalid_setup &&
                    cost_estimate &&
                    cost_estimate.excess_entries.length > 0
                "
            >
                <div>
                    <data-table
                        :definitions="tableDefinition()"
                        :entries="tableEntries()"
                    />
                </div>
            </card>
        </div>

        <n-result
            v-if="invalid_setup"
            status="error"
            title="Setup not complete"
            description="Please make sure that you have created at least one structure group."
            style="margin-top: 100px"
        />
    </div>
</template>

<script lang="ts">
import { h } from 'vue';
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { events } from '@/main';
import { ROUTE_CHANGE } from '@/event_bus';
import { NButton, NForm, NFormItem, NInput, NResult, NSpace } from 'naive-ui';

import {
    Service,
    type IProduct,
    type ICostEstimateResponse,
    type IExcessCostEntry,
} from '@/project/service';
import { type IParsedRow, ItemService } from '@/services/item';

import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import DataTable, {
    type IDataTableDefinition,
} from '@/components/DataTable.vue';
import EveIcon from '@/components/EveIcon.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import PageHeader from '@/components/PageHeader.vue';
import ProductSelector from '@/project/selectors/ProductSelector.vue';
import StructureGroupSelector from '@/components/selectors/StructureGroupSelector.vue';

@Component({
    components: {
        NButton,
        NForm,
        NFormItem,
        NInput,
        NResult,
        NSpace,

        Card,
        CardMargin,
        DataTable,
        FormatNumber,
        PageHeader,
        ProductSelector,
        StructureGroupSelector,
    },
})
class SystemIndex extends Vue {
    public products_input: string = '';
    public structure_group: string = '';
    public stock: string = '';

    public invalid_setup: boolean = true;

    public cost_estimate: null | ICostEstimateResponse = null;
    public busy = false;

    public async created() {
        events.$emit(ROUTE_CHANGE, this.$route.name);
    }

    public async create_estimate() {
        this.busy = true;

        let stocks = await ItemService.parse<IParsedRow>(this.stock, true);
        let products = await ItemService.parse<IProduct>(
            this.products_input,
            true,
        );

        this.cost_estimate = await Service.cost_estimate({
            structure_group: this.structure_group,
            products,
            stocks,
        });

        this.busy = false;
    }

    public no_structures(status: boolean) {
        this.invalid_setup = !status;
    }

    public tableEntries(): IExcessCostEntry[] {
        if (!this.cost_estimate) {
            return [];
        } else {
            return this.cost_estimate.excess_entries;
        }
    }

    public tableDefinition(): IDataTableDefinition[] {
        return [
            {
                header: '',
                key: 'icon',
                width: 40,
                render(row) {
                    return h(EveIcon, {
                        id: row.type_id,
                        type: 'icon',
                    });
                },
            },
            {
                header: 'Name',
                key: 'type_id',
                width: 500,
                item: true,
                copy: true,
            },
            {
                header: 'Quantity',
                key: 'quantity',
                width: 200,
                number: true,
                copy: true,
            },
            {
                header: 'Cost',
                key: 'cost',
                width: 200,
                number: true,
                copy: true,
            },
        ];
    }
}
export default toNative(SystemIndex);
</script>
