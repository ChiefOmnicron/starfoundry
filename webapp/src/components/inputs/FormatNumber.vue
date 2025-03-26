<template>
    <n-input-number
        :default-value="defaultValue"
        :disabled="readonly"
        :format="format"
        :parse="parse"
        :placeholder="placeholder"
        v-model:value="value"
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { formatNumber } from '@/utils';
import { NInputNumber } from 'naive-ui';

@Component({
    components: {
        NInputNumber,
    }
})
class FormatNumberInput extends Vue {
    @Prop({
        type: Boolean,
        default: false
    })
    public withComma!: boolean;

    @Prop({
        type: Number,
        required: false,
    })
    public defaultValue!: number;

    @Prop({
        type: String,
        default: 'Please Input'
    })
    public placeholder!: string;

    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public readonly!: boolean;

    // Holds the selected system id
    public value: number | null = null;

    public format(input: number | null): string {
        if (!input || Number.isNaN(input)) {
            return '';
        }
        return formatNumber(input, this.withComma);
    }

    public parse(input: string): number | null {
        if (!input) {
            return 0;
        }

        const nums = input.replace(/\./g, '').trim();
        if (/^\d+(\.(\d+)?)?$/.test(nums)) {
            return Number(nums)
        };
        return nums === '' ? null : Number.NaN;
    }
}

export default toNative(FormatNumberInput);
</script>
  