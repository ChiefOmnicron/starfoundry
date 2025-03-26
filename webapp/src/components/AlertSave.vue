<template>
    <n-alert
        title="Sucess"
        @close="close()"
        closable
        style="margin-bottom: 10px"
        type="success"
        v-if="visible"
    >
        Successfully saved
    </n-alert>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NAlert } from 'naive-ui';

@Component({
    components: {
        NAlert,
    },
    emits: [
        'close'
    ]
})
class AlertSave extends Vue {
    @Prop({
        default: false,
        required: true,
        type: Boolean,
    })
    public visible!: boolean;

    public close() {
        this.$emit('close', true)
    }
}

export default toNative(AlertSave);
</script>
