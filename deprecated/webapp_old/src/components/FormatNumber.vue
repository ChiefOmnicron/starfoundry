<template>
    <span
        :style="{
            fontVariantNumeric: 'tabular-nums',
            color: textColor(),
        }"
        :type="type"
    >
        {{ format() }}
    </span>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { formatDate, formatDateUTC, formatNumber, formatTime } from '@/utils';
import { NText } from 'naive-ui';

@Component({
    components: {
        NText,
    },
})
class FormatNumber extends Vue {
    @Prop({
        type: Number,
        required: true,
    })
    public value!: number;

    @Prop({
        type: Boolean,
        required: false,
    })
    public time!: boolean;

    @Prop({
        type: Boolean,
        required: false,
    })
    public date!: boolean;

    @Prop({
        type: Boolean,
        required: false,
        default: true,
    })
    public utc!: boolean;

    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public withComma!: boolean;

    @Prop({
        type: String,
        required: false,
        default: 'default',
    })
    public type!: string;

    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public disabled!: boolean;

    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public nullable!: boolean;

    public format(): string {
        if (this.time) {
            return formatTime(this.value || 0);
        } else if (this.date && this.utc) {
            return formatDateUTC(this.value || 0);
        } else if (this.date) {
            return formatDate(this.value || 0);
        } else if (this.nullable && !this.value) {
            return '-/-';
        } else {
            return formatNumber(this.value || 0, this.withComma);
        }
    }

    public textColor() {
        if (this.disabled) {
            return 'rgba(255, 255, 255, 0.2)';
        } else {
            return '';
        }
    }
}

export default toNative(FormatNumber);
</script>
