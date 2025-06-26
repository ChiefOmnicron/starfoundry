<template>
    <span>
        {{ marketName() }}
    </span>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { MARKETS } from '@/appraisal/service';

@Component({})
class AppraisalMarket extends Vue {
    @Prop({
        type: Number,
        required: true,
    })
    public marketId!: number;

    public marketName(): string {
        let entry = MARKETS.find((x) => x.key === this.marketId);

        if (entry) {
            return entry.label;
        } else {
            return 'Unknown';
        }
    }
}

export default toNative(AppraisalMarket);
</script>
