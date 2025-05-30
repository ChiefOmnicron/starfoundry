<template>
    <div>
        <n-tag data-cy="preparing" v-if="status === 'PREPARING'"
            >Preparing</n-tag
        >
        <n-tag
            data-cy="cancelled"
            type="error"
            v-else-if="status === 'CANCELLED'"
            >Cancelled</n-tag
        >
        <n-tag data-cy="paused" type="warning" v-else-if="status === 'PAUSED'"
            >Paused</n-tag
        >
        <n-tag data-cy="done" type="success" v-else-if="status === 'DONE'"
            >Done</n-tag
        >
        <n-tag
            data-cy="in_progress"
            type="info"
            v-else-if="status === 'IN_PROGRESS'"
            >In Progress</n-tag
        >
        <n-tag data-cy="default" type="error" v-else
            >Invalid Status {{ status }}</n-tag
        >
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NTag } from 'naive-ui';

@Component({
    components: {
        NTag,
    },
})
class StatusTag extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public status!: string;
}

export default toNative(StatusTag);
</script>
