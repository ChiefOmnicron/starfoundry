<template>
    <card danger title="Danger Zone">
        <n-space justify="space-between" style="padding: 10px">
            <div>
                <b>{{ objectTitle }}</b
                ><br />
                {{ objectDescription }}
            </div>

            <n-button
                @click="deleteObject"
                data-cy="deleteObject"
                type="error"
                style="margin-top: 7px"
                ghost
            >
                Delete
            </n-button>
        </n-space>

        <confirm-dialog v-model:show="showConfirm" :confirm="confirmRemove">
            Are you sure you want to delete the entry? This cannot be
            reversed!<br />
            Please type in "delete" to confirm.
        </confirm-dialog>
    </card>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NButton, NSpace } from 'naive-ui';
import Card from '@/components/Card.vue';
import ConfirmDialog from './ConfirmDialog.vue';

@Component({
    components: {
        NButton,
        NSpace,

        Card,
        ConfirmDialog,
    },
    emits: ['delete'],
})
class DeleteObject extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public objectTitle!: string;

    @Prop({
        required: true,
        type: String,
    })
    public objectDescription!: string;

    public showConfirm: boolean = false;

    public deleteObject() {
        this.showConfirm = true;
    }

    public confirmRemove() {
        this.$emit('delete');
    }
}

export default toNative(DeleteObject);
</script>
