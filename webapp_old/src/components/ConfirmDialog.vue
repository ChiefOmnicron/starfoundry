<template>
    <n-modal
        v-model:show="show"
        :on-update:show="close"
        :on-after-leave="() => (confirmDeleteText = '')"
        title="Delete Entry?"
        style="width: 600px"
    >
        <card content-style="padding: 0" no-title>
            <div style="margin: 10px">
                <n-alert type="warning" style="margin-bottom: 10px">
                    <slot></slot>
                </n-alert>

                <n-input
                    @keydown.enter.prevent
                    data-cy="confirmDialogConfirmInput"
                    placeholder="delete"
                    v-model:value="confirmDeleteText"
                />
            </div>

            <template #footer>
                <action-group justify="end" style="margin: 10px">
                    <n-button @click="close" quaternary> Cancel </n-button>

                    <n-button
                        :disabled="confirmDeleteText.toLowerCase() !== 'delete'"
                        @click="confirm_delete"
                        data-cy="confirmDialogDeleteButton"
                    >
                        Delete
                    </n-button>
                </action-group>
            </template>
        </card>
    </n-modal>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NAlert, NButton, NInput, NModal, NSpace } from 'naive-ui';

import ActionGroup from './ActionGroup.vue';
import Card from '@/components/Card.vue';

@Component({
    components: {
        NAlert,
        NButton,
        NInput,
        NModal,
        NSpace,

        ActionGroup,
        Card,
    },
})
class ConfirmDialog extends Vue {
    @Prop({
        required: true,
    })
    public confirm!: () => void;

    @Prop({
        required: true,
    })
    public show!: boolean;

    public confirmDeleteText: string = '';

    public close() {
        this.$emit('update:show', false);
    }

    public confirm_delete() {
        this.confirm();
        this.close();
    }
}

export default toNative(ConfirmDialog);
</script>
