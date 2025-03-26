<template>
    <n-modal v-model:show="show" :on-update:show="close">
        <card
            title="Check Resources"
            style="width: 900px"
        >
            <check-resources
                :project-job-ids="projectJobIds"
                @close="close"
            />
        </card>
    </n-modal>
</template>

<script lang="ts">
import { Component, Vue, Prop, toNative } from 'vue-facing-decorator';
import { NModal } from 'naive-ui';

import type { Uuid } from '@/sdk/utils';

import Card from '@/components/Card.vue';
import CheckResources from '@/project/CheckResources.vue';

@Component({
    components: {
        NModal,

        Card,
        CheckResources,
    },
    emits: ['close']
})
class CheckResourcesModal extends Vue {
    @Prop({
        required: true,
        type: Array<Uuid>
    })
    public projectJobIds!: Uuid[];

    @Prop({
        required: true,
        type: Boolean,
    })
    public show!: boolean;

    public close() {
        this.$emit('close', true);
    }
}

export default toNative(CheckResourcesModal);
</script>
