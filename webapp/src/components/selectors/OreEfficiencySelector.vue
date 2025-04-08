<template>
    <n-select
        @update:value="handleOreReprocessingUpdate"
        :options="oreReprocessingOptions()"
        v-model:value="selected"
        filterable
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, Watch, toNative } from 'vue-facing-decorator';

import type { OreReprocessing } from '@/appraisal/service';

import { NSelect, type SelectOption, type TreeOverrideNodeClickBehaviorReturn, type TreeSelectOption } from 'naive-ui';

@Component({
    components: {
        NSelect,
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

    @Prop({
        type: String,
        required: true,
    })
    public system!: 'HS' | 'LS' | 'NS';

    @Prop({
        type: String,
        required: true,
    })
    public structure!: 'ATHANOR' | 'TATARA';

    public selected = 'NsTataraT2';

    public created() {
        this.setReprocessing('NsTataraT2');
    }

    public handleOreReprocessingUpdate(value: OreReprocessing) {
        this.$emit('update:efficiency', value);
    }

    public oreReprocessingOptions(): SelectOption[] {
        if (this.system === 'HS' && this.structure === 'ATHANOR') {
            return [{
                label: 'No Rig (73.81%)',
                value: 'HsAthanorNoRig'
            }, {
                label: 'T1 Rig (75.28%)',
                value: 'HsAthanorT1'
            }, {
                label: 'T2 Rig (78.23%)',
                value: 'HsAthanorT2'
            }];
        } else if (this.system === 'HS' && this.structure === 'TATARA') {
            return [{
                label: 'No Rig (76.34%)',
                value: 'HsTataraNoRig'
            }, {
                label: 'T1 Rig (77.86%)',
                value: 'HsTataraT1'
            }, {
                label: 'T2 Rig (80.92%)',
                value: 'HsTataraT2'
            }];
        } else if (this.system === 'LS' && this.structure === 'ATHANOR') {
            return [{
                label: 'No Rig (73.81%)',
                value: 'LsAthanorNoRig'
            }, {
                label: 'T1 Rig (79.80%)',
                value: 'LsAthanorT1'
            }, {
                label: 'T2 Rig (82.93%)',
                value: 'LsAthanorT2'
            }];
        } else if (this.system === 'LS' && this.structure === 'TATARA') {
            return [{
                label: 'No Rig (76.34%)',
                value: 'LsTataraNoRig'
            }, {
                label: 'T1 Rig (82.54%)',
                value: 'LsTataraT1'
            }, {
                label: 'T2 Rig (85.77%)',
                value: 'LsTataraT2'
            }];
        } else if (this.system === 'NS' && this.structure === 'ATHANOR') {
            return [{
                label: 'No Rig (73.81%)',
                value: 'NsAthanorNoRig'
            }, {
                label: 'T1 Rig (84.32%)',
                value: 'NsAthanorT1'
            }, {
                label: 'T2 Rig (87.62%)',
                value: 'NsAthanorT2'
            }];
        } else if (this.system === 'NS' && this.structure === 'TATARA') {
            return [{
                label: 'No Rig (76.34%)',
                value: 'NsTataraNoRig'
            }, {
                label: 'T1 Rig (87.21%)',
                value: 'NsTataraT1'
            }, {
                label: 'T2 Rig (90.63%)',
                value: 'NsTataraT2'
            }];
        } else {
            return [];
        }
    }

    public setReprocessing(structure: string) {
        this.selected = structure;
        this.$emit('update:efficiency', structure);
    }

    @Watch('structure')
    public structureWatch(newValue: string) {
        if (newValue === 'ATHANOR' && this.system === 'HS') {
            this.setReprocessing('HsAthanorT2');
        } else if (newValue === 'ATHANOR' && this.system === 'LS') {
            this.setReprocessing('LsAthanorT2');
        } else if (newValue === 'ATHANOR' && this.system === 'NS') {
            this.setReprocessing('NsAthanorT2');
        }

        if (newValue === 'TATARA' && this.system === 'HS') {
            this.setReprocessing('HsTataraT2');
        } else if (newValue === 'TATARA' && this.system === 'LS') {
            this.setReprocessing('LsTataraT2');
        } else if (newValue === 'TATARA' && this.system === 'HS') {
            this.setReprocessing('NsTataraT2');
        }
    }

    @Watch('system')
    public systemWatch(newValue: string) {
        if (this.structure === 'ATHANOR' && newValue === 'HS') {
            this.setReprocessing('HsAthanorT2');
        } else if (this.structure === 'ATHANOR' && newValue === 'LS') {
            this.setReprocessing('LsAthanorT2');
        } else if (this.structure === 'ATHANOR' && newValue === 'NS') {
            this.setReprocessing('NsAthanorT2');
        }

        if (this.structure === 'TATARA' && newValue === 'HS') {
            this.setReprocessing('HsTataraT2');
        } else if (this.structure === 'TATARA' && newValue === 'LS') {
            this.setReprocessing('LsTataraT2');
        } else if (this.structure === 'TATARA' && newValue === 'HS') {
            this.setReprocessing('NsTataraT2');
        }
    }
}

export default toNative(OreReprocessingSelector);
</script>
