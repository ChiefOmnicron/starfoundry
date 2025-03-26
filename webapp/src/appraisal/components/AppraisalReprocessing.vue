<template>
    <n-table style="margin-bottom: 10px">
        <tr>
            <th style="width: 20%">Ore Efficiency</th>
            <td style="text-align: left;">
                <ore-efficiency-selector
                    v-model:efficiency="reprocessing.ore_reprocessing"
                />
            </td>
        </tr>
        <tr>
            <th>Gas Decompression</th>
            <td style="text-align: left;">
                <gas-decompression-selector
                    v-model:decompression="reprocessing.gas_decompression"
                />
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

import { NSwitch, NTable, NTreeSelect } from 'naive-ui';
import CopyText from '@/components/CopyText.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import GasDecompressionSelector from '@/components/selectors/GasDecompressionSelector.vue';
import OreBlacklistSelector from '@/components/selectors/OreBlacklistSelector.vue';
import OreEfficiencySelector from '@/components/selectors/OreEfficiencySelector.vue';
import ScrapReprocessingSelector from '@/components/selectors/ScrapReprocessingSelector.vue';

@Component({
    components: {
        NSwitch,
        NTable,
        NTreeSelect,

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

    public handleUpdateValue() {
        this.$emit('update:reprocessing', this.reprocessing);
    }
}

export default toNative(AppraisalReprocessing);
</script>
