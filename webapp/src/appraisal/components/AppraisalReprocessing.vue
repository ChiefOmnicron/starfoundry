<template>
    <n-table style="margin-bottom: 10px">
        <tr>
            <th style="width: 20%">Ore Efficiency</th>
            <td style="text-align: left;">
                <n-grid :cols="3" x-gap="10">
                    <n-grid-item>
                        <n-select
                            v-model:value="selectedStructureOre"
                            :options="structures()"
                        />
                    </n-grid-item>

                    <n-grid-item>
                        <n-select
                            v-model:value="selectedSystem"
                            :options="systems()"
                        />
                    </n-grid-item>

                    <n-grid-item>
                        <ore-efficiency-selector
                            v-model:efficiency="reprocessing.ore_reprocessing"
                            :structure="selectedStructureOre"
                            :system="selectedSystem"
                        />
                    </n-grid-item>
                </n-grid>
            </td>
        </tr>
        <tr>
            <th>Gas Decompression</th>
            <td style="text-align: left;">
                <n-grid :cols="2" x-gap="10">
                    <n-grid-item>
                        <n-select
                            v-model:value="selectedStructureGas"
                            :options="structures()"
                        />
                    </n-grid-item>

                    <n-grid-item>
                        <gas-decompression-selector
                            v-model:decompression="reprocessing.gas_decompression"
                            :structure="selectedStructureGas"
                        />
                    </n-grid-item>
                </n-grid>
            </td>
        </tr>
        <tr>
            <th>Scrap Reprocessing</th>
            <td style="text-align: left;">
                <scrap-reprocessing-selector
                    v-model:reprocessing="reprocessing.scrap_reprocessing"
                />
            </td>
        </tr>
    </n-table>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { IReprocessingOptions } from '@/appraisal/service';

import { NGrid, NGridItem, NSelect, NSwitch, NTable, type SelectOption } from 'naive-ui';
import CopyText from '@/components/CopyText.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import GasDecompressionSelector from '@/components/selectors/GasDecompressionSelector.vue';
import OreBlacklistSelector from '@/components/selectors/OreBlacklistSelector.vue';
import OreEfficiencySelector from '@/components/selectors/OreEfficiencySelector.vue';
import ScrapReprocessingSelector from '@/components/selectors/ScrapReprocessingSelector.vue';

@Component({
    components: {
        NGrid,
        NGridItem,
        NSelect,
        NSwitch,
        NTable,

        CopyText,
        FormatNumber,
        GasDecompressionSelector,
        OreBlacklistSelector,
        OreEfficiencySelector,
        ScrapReprocessingSelector,
    },
    emits: [
        'update:reprocessing'
    ],
})
class AppraisalReprocessing extends Vue {
    @Prop({
        type: Object,
        required: true,
    })
    public reprocessing!: IReprocessingOptions;

    public selectedStructureOre: string = 'TATARA';
    public selectedStructureGas: string = 'TATARA';
    public selectedSystem: string = 'NS';

    public handleUpdateValue() {
        this.$emit('update:reprocessing', this.reprocessing);
    }

    public structures(): SelectOption[] {
        return [{
            label: 'Athanor',
            value: 'ATHANOR'
        }, {
            label: 'Tatara',
            value: 'TATARA'
        }];
    }

    public systems(): SelectOption[] {
        return [{
            label: 'Highsec',
            value: 'HS'
        }, {
            label: 'Lowsec',
            value: 'LS'
        }, {
            label: 'Nullsec',
            value: 'NS'
        }];
    }
}

export default toNative(AppraisalReprocessing);
</script>
