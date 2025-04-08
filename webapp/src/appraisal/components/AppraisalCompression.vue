<template>
    <n-table style="margin-bottom: 10px">
        <tr>
            <th style="width: 20%">Ore Efficiency</th>
            <td style="text-align: left;" colspan="3">
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
                            v-model:efficiency="compression.ore_reprocessing"
                            :structure="selectedStructureOre"
                            :system="selectedSystem"
                        />
                    </n-grid-item>
                </n-grid>
            </td>
        </tr>
        <tr>
            <th style="width: 20%">Additional Options</th>
            <td style="text-align: left;">
                <n-switch
                    v-model:value="compression.allow_minerals"
                >
                    <template #checked>
                        Allow minerals
                    </template>
                    <template #unchecked>
                        Disallow minerals
                    </template>
                </n-switch>
            </td>
            <td style="text-align: left;">
                <n-switch v-model:value="compression.allow_uncompressed_gas">
                    <template #checked>
                        Allow uncompressed gas
                    </template>
                    <template #unchecked>
                        Disallow uncompressed gas
                    </template>
                </n-switch>
            </td>
        </tr>
        <tr>
            <th style="width: 20%"></th>
            <td style="text-align: left;">
                <n-switch v-model:value="compression.allow_uncompressed_ore">
                    <template #checked>
                        Allow uncompressed ores
                    </template>
                    <template #unchecked>
                        Disallow uncompressed ores
                    </template>
                </n-switch>
            </td>
            <td style="text-align: left;">
                <n-switch v-model:value="compression.allow_uncompressed_moon">
                    <template #checked>
                        Allow uncompressed moon ores
                    </template>
                    <template #unchecked>
                        Disallow uncompressed moon ores
                    </template>
                </n-switch>
            </td>
        </tr>
        <tr>
            <th>Blacklist Ores</th>
            <td style="text-align: left" colspan="3">
                <ore-blacklist-selector
                    v-model:blacklist="compression.blacklist"
                />
            </td>
        </tr>
        <tr>
            <th>Gas Decompression</th>
            <td style="text-align: left;" colspan="3">
                <n-grid :cols="2" x-gap="10">
                    <n-grid-item>
                        <n-select
                            v-model:value="selectedStructureGas"
                            :options="structures()"
                        />
                    </n-grid-item>

                    <n-grid-item>
                        <gas-decompression-selector
                            v-model:decompression="compression.gas_decompression"
                            :structure="selectedStructureGas"
                        />
                    </n-grid-item>
                </n-grid>
            </td>
        </tr>
    </n-table>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { ICompressionOptions } from '@/appraisal/service';

import { NGrid, NGridItem, NSelect, NSwitch, NTable, type SelectOption } from 'naive-ui';
import CopyText from '@/components/CopyText.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import GasDecompressionSelector from '@/components/selectors/GasDecompressionSelector.vue';
import OreBlacklistSelector from '@/components/selectors/OreBlacklistSelector.vue';
import OreEfficiencySelector from '@/components/selectors/OreEfficiencySelector.vue';

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
    },
    emits: [
        'update:compression'
    ],
})
class AppraisalCompression extends Vue {
    @Prop({
        type: Object,
        required: true,
    })
    public compression!: ICompressionOptions;

    public selectedStructureOre: string = 'TATARA';
    public selectedStructureGas: string = 'TATARA';
    public selectedSystem: string = 'NS';

    public handleUpdateValue() {
        this.$emit('update:compression', this.compression);
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

export default toNative(AppraisalCompression);
</script>
