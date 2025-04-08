<template>
    <n-select
        @update:value="handleGasDecompressionUpdate"
        :options="gasDecompressionOptions()"
        v-model:value="selected"
        filterable
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, Watch, toNative } from 'vue-facing-decorator';

import type { GasDecompression } from '@/appraisal/service';

import { NSelect, type SelectOption } from 'naive-ui';

@Component({
    components: {
        NSelect,
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

    @Prop({
        type: String,
        required: true,
    })
    public structure!: 'ATHANOR' | 'TATARA';

    public selected = 'TataraLvl5';

    public created() {
        this.setDecompression('TataraLvl5');
    }

    public handleGasDecompressionUpdate(value: GasDecompression) {
        this.$emit('update:decompression', value);
    }

    public gasDecompressionOptions(): SelectOption[] {
        console.log(this.structure)
        if (this.structure === 'ATHANOR') {
            return [{
                label: 'Level 0 (84%)',
                value: 'AthanorLvl0'
            }, {
                label: 'Level 1 (85%)',
                value: 'AthanorLvl1'
            }, {
                label: 'Level 2 (86%)',
                value: 'AthanorLvl2'
            }, {
                label: 'Level 3 (87%)',
                value: 'AthanorLvl3'
            }, {
                label: 'Level 4 (88%)',
                value: 'AthanorLvl4'
            }, {
                label: 'Level 5 (89%)',
                value: 'AthanorLvl5'
            }];
        } else if (this.structure === 'TATARA') {
            return [{
                label: 'Level 0 (90%)',
                value: 'TataraLvl0'
            }, {
                label: 'Level 1 (91%)',
                value: 'TataraLvl1'
            }, {
                label: 'Level 2 (92%)',
                value: 'TataraLvl2'
            }, {
                label: 'Level 3 (93%)',
                value: 'TataraLvl3'
            }, {
                label: 'Level 4 (94%)',
                value: 'TataraLvl4'
            }, {
                label: 'Level 5 (95%)',
                value: 'TataraLvl5'
            }];
        } else {
            return [];
        }
    }

    public setDecompression(structure: string) {
        this.selected = structure;
        this.$emit('update:decompression', structure);
    }

    @Watch('structure')
    public structureWatch(newValue: string) {
        if (newValue === 'ATHANOR') {
            this.setDecompression('AthanorLvl5');
        } else {
            this.setDecompression('TataraLvl5');
        }
    }
}

export default toNative(GasDecompressionSelector);
</script>
