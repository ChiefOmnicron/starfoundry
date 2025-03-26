<template>
    <n-tree-select
        @update:value="handleScrapReprcessingUpdsate"
        :options="scrapReprocessingOptions()"
        default-value="Lvl5"
        filterable
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { ScrapReprocessing } from '@/appraisal/service';

import { NTreeSelect, type SelectOption } from 'naive-ui';

@Component({
    components: {
        NTreeSelect,
    },
    emits: [
        'update:reprocessing'
    ],
})
class ScrapReprocessingSelector extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public reprocessing!: String;

    public handleScrapReprcessingUpdsate(value: ScrapReprocessing) {
        this.$emit('update:reprocessing', value);
    }

    public scrapReprocessingOptions(): SelectOption[] {
        return [{
            label: 'Scrap Reprocessing Lvl 0 (50%)',
            key: 'Lvl0'
        }, {
            label: 'Scrap Reprocessing Lvl 1 (51%)',
            key: 'Lvl1'
        }, {
            label: 'Scrap Reprocessing Lvl 2 (52%)',
            key: 'Lvl2'
        }, {
            label: 'Scrap Reprocessing Lvl 3 (53%)',
            key: 'Lvl3'
        }, {
            label: 'Scrap Reprocessing Lvl 4 (54%)',
            key: 'Lvl4'
        }, {
            label: 'Scrap Reprocessing Lvl 5 (55%)',
            key: 'Lvl5'
        }];
    }
}

export default toNative(ScrapReprocessingSelector);
</script>
