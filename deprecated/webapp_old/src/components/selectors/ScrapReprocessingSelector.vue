<template>
    <n-select
        :options="scrapReprocessingOptions()"
        @update:value="handleScrapReprcessingUpdsate"
        v-model:value="selected"
        filterable
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { ScrapReprocessing } from '@/appraisal/service';

import { NSelect, type SelectOption } from 'naive-ui';

@Component({
    components: {
        NSelect,
    },
    emits: ['update:reprocessing'],
})
class ScrapReprocessingSelector extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public reprocessing!: String;

    public selected = 'Lvl5';

    public created() {
        this.selected = 'Lvl5';
    }

    public handleScrapReprcessingUpdsate(value: ScrapReprocessing) {
        this.$emit('update:reprocessing', value);
    }

    public scrapReprocessingOptions(): SelectOption[] {
        return [
            {
                label: 'Level 0 (50%)',
                value: 'Lvl0',
            },
            {
                label: 'Level 1 (51%)',
                value: 'Lvl1',
            },
            {
                label: 'Level 2 (52%)',
                value: 'Lvl2',
            },
            {
                label: 'Level 3 (53%)',
                value: 'Lvl3',
            },
            {
                label: 'Level 4 (54%)',
                value: 'Lvl4',
            },
            {
                label: 'Level 5 (55%)',
                value: 'Lvl5',
            },
        ];
    }
}

export default toNative(ScrapReprocessingSelector);
</script>
