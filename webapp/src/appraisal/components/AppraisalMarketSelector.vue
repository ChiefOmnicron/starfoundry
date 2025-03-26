<template>
    <n-tree-select
        @update:value="handleMarketSelect"
        :options="markets()"
        :default-value="60003760"
        filterable
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NTreeSelect, type SelectOption } from 'naive-ui';

@Component({
    components: {
        NTreeSelect,
    },
    emits: [
        'update:market'
    ],
})
class AppraisalMarketSelector extends Vue {
    @Prop({
        type: Number,
        required: true,
    })
    public market!: number;

    public handleMarketSelect(value: number) {
        this.$emit('update:market', value);
    }

    public markets(): SelectOption[] {
        return [{
            label: 'Jita 4-4',
            key: 60003760,
        }, {
            label: 'Amarr',
            key: 60008494,
        }, {
            label: 'E3OI-U',
            key: 1040278453044,
        }, {
            label: 'UALX-3',
            key: 1046664001931,
        }, {
            label: 'K7D-II',
            key: 1043661023026,
        }];
    }
}

export default toNative(AppraisalMarketSelector);
</script>
