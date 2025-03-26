<template>
    <n-tree-select
        @update:value="handleGasDecompressionUpdate"
        :options="gasDecompressionOptions()"
        :override-default-node-click-behavior="selectOverride"
        default-value="TataraLvl5"
        filterable
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { GasDecompression } from '@/appraisal/service';

import { NTreeSelect, type SelectOption, type TreeOverrideNodeClickBehaviorReturn, type TreeSelectOption } from 'naive-ui';

@Component({
    components: {
        NTreeSelect,
    },
    emits: [
        'update:decompression'
    ],
})
class GasDecompressionSelector extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public decompression!: String;

    public handleGasDecompressionUpdate(value: GasDecompression) {
        this.$emit('update:decompression', value);
    }

    public gasDecompressionOptions(): SelectOption[] {
        return [{
            label: 'Athanor',
            key: 'athanor',
            children: [{
                label: 'Gas Decompression Lvl 0 (84%)',
                key: 'AthanorLvl0'
            }, {
                label: 'Gas Decompression Lvl 1 (85%)',
                key: 'AthanorLvl1'
            }, {
                label: 'Gas Decompression Lvl 2 (86%)',
                key: 'AthanorLvl2'
            }, {
                label: 'Gas Decompression Lvl 3 (87%)',
                key: 'AthanorLvl3'
            }, {
                label: 'Gas Decompression Lvl 4 (88%)',
                key: 'AthanorLvl4'
            }, {
                label: 'Gas Decompression Lvl 5 (89%)',
                key: 'AthanorLvl5'
            }]
        }, {
            label: 'Tatara',
            key: 'tatara',
            children: [{
                label: 'Gas Decompression Lvl 0 (90%)',
                key: 'TataraLvl0'
            }, {
                label: 'Gas Decompression Lvl 1 (91%)',
                key: 'TataraLvl1'
            }, {
                label: 'Gas Decompression Lvl 2 (92%)',
                key: 'TataraLvl2'
            }, {
                label: 'Gas Decompression Lvl 3 (93%)',
                key: 'TataraLvl3'
            }, {
                label: 'Gas Decompression Lvl 4 (94%)',
                key: 'TataraLvl4'
            }, {
                label: 'Gas Decompression Lvl 5 (95%)',
                key: 'TataraLvl5'
            }]
        }];
    }

    public selectOverride(option: { option: TreeSelectOption }): TreeOverrideNodeClickBehaviorReturn {
        if (option.option.children) {
            return 'toggleExpand';
        }

        return 'default';
    }
}

export default toNative(GasDecompressionSelector);
</script>
