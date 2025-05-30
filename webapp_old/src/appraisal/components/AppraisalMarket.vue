<template>
    <span>
        {{ marketName() }}
    </span>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

@Component({})
class AppraisalMarket extends Vue {
    @Prop({
        type: Number,
        required: true,
    })
    public marketId!: number;

    public marketName(): string {
        let entry = [
            {
                label: 'Jita 4-4',
                key: 60003760,
            },
            {
                label: 'Amarr',
                key: 60008494,
            },
            {
                label: 'E3OI-U',
                key: 1040278453044,
            },
            {
                label: 'UALX-3',
                key: 1046664001931,
            },
            {
                label: 'K7D-II',
                key: 1043661023026,
            },
        ].find((x) => x.key === this.marketId);

        if (entry) {
            return entry.label;
        } else {
            return 'Unknown';
        }
    }
}

export default toNative(AppraisalMarket);
</script>
