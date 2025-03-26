<template>
    <n-modal v-model:show="show" :on-update:show="close">
        <card
            :bordered="false"
            title="Add Bulk Entries"
            style="width: 900px"
        >
            <parse-market-items
                @update:value="(x: any) => entries = x"
            />

            <action-group style="margin-right: 5px">
                <n-button
                    @click="close()"
                    quaternary
                >
                    Close
                </n-button>

                <n-button
                    @click="addEntries"
                    :disabled="entries.length === 0"
                    type="info"
                >
                    Add
                </n-button>
            </action-group>
        </card>
    </n-modal>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NInput, NModal } from 'naive-ui';

import type { IMarketEntry } from '@/project/service';

import ActionGroup from '@/components/ActionGroup.vue';
import Card from '@/components/Card.vue';
import FormatNumberInput from '@/components/inputs/FormatNumber.vue';
import ItemSelector from '@/components/selectors/ItemSelector.vue';
import ParseMarketItems from '@/components/inputs/ParseMarketItems.vue';

@Component({
    components: {
        NButton,
        NInput,
        NModal,

        ActionGroup,
        Card,
        FormatNumberInput,
        ItemSelector,
        ParseMarketItems,
    },
    emits: [
        'close',
        'close:value',
    ]
})
class BulkAddMarketEntryModal extends Vue {
    @Prop({
        type: Boolean,
        required: true,
    })
    public show!: boolean;

    public entries: IMarketEntry[] = [];

    public close() {
        this.$emit('close', true);
    }

    public async addEntries() {
        this.$emit('close:value', this.entries);
        this.entries = [];
    }
}

export default toNative(BulkAddMarketEntryModal);
</script>
