<template>
    <n-button :href="link()" tag="a" target="_blank" text>
        <format-number :value="milliseconds()" type="primary" date />
    </n-button>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NButton } from 'naive-ui';
import FormatNumber from '@/components/FormatNumber.vue';

@Component({
    components: {
        NButton,

        FormatNumber,
    },
})
class Nakamura extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public endDate!: string;

    public link() {
        return 'https://time.nakamura-labs.com/?#' + this.milliseconds() / 1000;
    }

    public milliseconds(): number {
        return new Date(this.endDate).valueOf();
    }
}

export default toNative(Nakamura);
</script>
