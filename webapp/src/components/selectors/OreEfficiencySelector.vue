<template>
    <n-tree-select
        @update:value="handleOreReprocessingUpdate"
        :options="oreReprocessingOptions()"
        :override-default-node-click-behavior="selectOverride"
        default-value="NsTataraT2"
        filterable
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { OreReprocessing } from '@/appraisal/service';

import { NTreeSelect, type SelectOption, type TreeOverrideNodeClickBehaviorReturn, type TreeSelectOption } from 'naive-ui';

@Component({
    components: {
        NTreeSelect,
    },
    emits: [
        'update:efficiency'
    ],
})
class OreReprocessingSelector extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public efficiency!: String;

    public handleOreReprocessingUpdate(value: OreReprocessing) {
        this.$emit('update:efficiency', value);
    }

    public oreReprocessingOptions(): SelectOption[] {
        return [{
            label: 'Highsec',
            key: 'hs',
            children: [{
                label: 'Athanor - No Rig (73.81%)',
                key: 'HsAthanorNoRig'
            }, {
                label: 'Athanor - T1 Rig (75.28%)',
                key: 'HsAthanorT1'
            }, {
                label: 'Athanor - T2 Rig (78.23%)',
                key: 'HsAthanorT2'
            }, {
                label: 'Tatara - No Rig (76.34%)',
                key: 'HsTataraNoRig'
            }, {
                label: 'Tatara - T1 Rig (77.86%)',
                key: 'HsTataraT1'
            }, {
                label: 'Tatara - T2 Rig (80.92%)',
                key: 'HsTataraT2'
            }]
        }, {
            label: 'Lowsec',
            key: 'ls',
            children: [{
                label: 'Athanor - No Rig (73.81%)',
                key: 'LsAthanorNoRig'
            }, {
                label: 'Athanor - T1 Rig (79.80%)',
                key: 'LsAthanorT1'
            }, {
                label: 'Athanor - T2 Rig (82.93%)',
                key: 'LsAthanorT2'
            }, {
                label: 'Tatara - No Rig (76.34%)',
                key: 'LsTataraNoRig'
            }, {
                label: 'Tatara - T1 Rig (82.54%)',
                key: 'LsTataraT1'
            }, {
                label: 'Tatara - T2 Rig (85.77%)',
                key: 'LsTataraT2'
            }]
        }, {
            label: 'Nullsec',
            key: 'ns',
            children: [{
                label: 'Athanor - No Rig (73.81%)',
                key: 'NsAthanorNoRig'
            }, {
                label: 'Athanor - T1 Rig (84.32%)',
                key: 'NsAthanorT1'
            }, {
                label: 'Athanor - T2 Rig (87.62%)',
                key: 'NsAthanorT2'
            }, {
                label: 'Tatara - No Rig (76.34%)',
                key: 'NsTataraNoRig'
            }, {
                label: 'Tatara - T1 Rig (87.21%)',
                key: 'NsTataraT1'
            }, {
                label: 'Tatara - T2 Rig (90.63%)',
                key: 'NsTataraT2'
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

export default toNative(OreReprocessingSelector);
</script>
