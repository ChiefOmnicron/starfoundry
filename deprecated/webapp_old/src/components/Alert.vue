<template>
    <n-alert
        :title="title"
        @close="close()"
        closable
        style="margin-bottom: 10px"
        type="error"
        v-if="visible"
    >
        {{ description }}
    </n-alert>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NAlert } from 'naive-ui';

@Component({
    components: {
        NAlert,
    },
    emits: ['close'],
})
class Alert extends Vue {
    @Prop({
        default: false,
        required: true,
        type: Boolean,
    })
    public visible!: boolean;

    @Prop({
        required: true,
        type: String,
    })
    public description!: string;

    @Prop({
        required: true,
        type: String,
    })
    public title!: string;

    public close() {
        this.$emit('close', true);
    }
}

export default toNative(Alert);

export interface IAlertConfiguration {
    active: boolean;

    description: string;
    title: string;
}
</script>
