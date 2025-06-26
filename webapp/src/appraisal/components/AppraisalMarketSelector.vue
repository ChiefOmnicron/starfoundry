<template>
    <n-tree-select
        :options="markets()"
        @update:value="handleMarketSelect"
        v-model:value="selectedMarket"
        filterable
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, Watch, toNative } from 'vue-facing-decorator';

import { NTreeSelect, type SelectOption } from 'naive-ui';
import { MARKETS } from '@/appraisal/service';

@Component({
    components: {
        NTreeSelect,
    },
    emits: ['update:market'],
})
class AppraisalMarketSelector extends Vue {
    @Prop({
        type: Number,
        required: true,
    })
    public market!: number;

    public selectedMarket: number = 60003760;

    public handleMarketSelect(value: number) {
        this.$emit('update:market', value);
    }

    public markets(): SelectOption[] {
        return MARKETS;
    }

    @Watch('market')
    public marketWatcher(newValue: number) {
        this.selectedMarket = newValue;
        this.$emit('update:market', newValue);
    }
}

export default toNative(AppraisalMarketSelector);
</script>
