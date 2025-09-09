<template>
    <n-modal v-model:show="show" :on-update:show="close">
        <card title="Export" style="width: 900px" :bordered="false">
            <item-export
                :format-list="dataFields"
                :format-ingame="dataFieldsIngame"
                :data="data"
                style="margin-bottom: 5px"
                @close="close()"
            />
        </card>
    </n-modal>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NModal, NSpace } from 'naive-ui';

import Card from '@/components/Card.vue';
import ItemExport from '@/components/ItemExport.vue';

@Component({
    components: {
        NButton,
        NModal,
        NSpace,

        Card,
        ItemExport,
    },
    emits: ['close'],
})
class ItemExportModal extends Vue {
    @Prop({
        type: Array,
        required: true,
    })
    public data: any;

    @Prop({
        type: Array,
        required: false,
        default: ['name', 'quantity'],
    })
    public dataFields: string[] = <any>null;

    @Prop({
        type: Array,
        required: false,
        default: () => ['type_id', 'name'],
    })
    public dataFieldsIngame: string[] = <any>null;

    @Prop({
        type: Boolean,
        required: true,
    })
    public show!: boolean;

    public close() {
        this.$emit('close', true);
    }
}

export default toNative(ItemExportModal);
</script>
