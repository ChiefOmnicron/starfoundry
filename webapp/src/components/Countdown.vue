<template>
    <div>
        <format-number
            :time="true"
            :value="remaining"
            v-if="remaining > 0"
        />

        <n-tag
            type="success"
            v-else="remaining === 0"
        >
            Done
        </n-tag>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NCountdown, NTag } from 'naive-ui';
import FormatNumber from '@/components/FormatNumber.vue';

@Component({
    components: {
        NCountdown,
        NTag,

        FormatNumber,
    }
})
class Countdown extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public endDate!: string;

    public remaining: number = 0;
    private intervalId!: number;

    public mounted() {
        this.updateTime();

        this.intervalId = setInterval(() => {
            this.updateTime();
        }, 1000);
    }

    public unmounted() {
        clearInterval(this.intervalId);
    }

    private updateTime() {
        // Typescript hates calculating dates
        const start: any = new Date();
        const end: any = new Date(this.endDate);
        const remaining = Math.floor((end - start) / 1000);
        this.remaining = remaining > 0 ? remaining : 0;
    }
}

export default toNative(Countdown);
</script>
