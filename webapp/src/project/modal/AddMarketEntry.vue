<template>
    <n-modal v-model:show="show" :on-update:show="close">
        <card :bordered="false" title="Add Entry" style="width: 900px">
            <n-form
                :model="newEntry"
                :rules="rules"
                style="margin-left: 5px; margin-right: 5px; margin-top: 10px"
            >
                <n-form-item label="Item" path="type_id">
                    <item-selector v-model:value="newEntry.type_id" />
                </n-form-item>

                <n-form-item label="Quantity" path="quantity">
                    <format-number-input
                        placeholder="Insert quantity"
                        style="width: 100%"
                        v-model:value="newEntry.quantity"
                    />
                </n-form-item>

                <n-form-item label="Cost" path="cost">
                    <format-number-input
                        placeholder="Insert Cost"
                        style="width: 100%"
                        v-model:value="newEntry.cost"
                    />
                </n-form-item>

                <n-form-item label="Source" path="source">
                    <n-input
                        placeholder="Insert Source"
                        style="width: 100%"
                        v-model:value="newEntry.source"
                    />
                </n-form-item>
            </n-form>

            <action-group style="margin-right: 5px">
                <n-button @click="close()" quaternary> Close </n-button>

                <n-button
                    @click="addEntry"
                    :disabled="!validForm()"
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
import {
    type FormRules,
    NButton,
    NForm,
    NFormItem,
    NInput,
    NModal,
} from 'naive-ui';

import type { IMarketEntry } from '@/project/service';

import ActionGroup from '@/components/ActionGroup.vue';
import Card from '@/components/Card.vue';
import FormatNumberInput from '@/components/inputs/FormatNumber.vue';
import ItemSelector from '@/components/selectors/ItemSelector.vue';

@Component({
    components: {
        NButton,
        NForm,
        NFormItem,
        NInput,
        NModal,

        ActionGroup,
        Card,
        FormatNumberInput,
        ItemSelector,
    },
    emits: ['close', 'close:value'],
})
class AddMarketEntryModal extends Vue {
    @Prop({
        type: Boolean,
        required: true,
    })
    public show!: boolean;

    public newEntry: IMarketEntry = <any>{};

    public close() {
        this.$emit('close', true);
        this.newEntry = <any>{};
    }

    public async addEntry() {
        this.$emit('close:value', this.newEntry);
        this.newEntry = <any>{};
    }

    public rules: FormRules = {
        type_id: [
            {
                required: true,
                message: 'The field is required',
            },
        ],
        quantity: [
            {
                required: true,
                message: 'The field is required',
            },
        ],
        cost: [
            {
                required: true,
                message: 'The field is required',
            },
        ],
        source: [
            {
                required: true,
                message: 'The field is required',
            },
        ],
    };

    public validForm(): boolean {
        if (
            this.newEntry.type_id &&
            this.newEntry.quantity &&
            (this.newEntry.cost || false) &&
            (this.newEntry.source || false)
        ) {
            return true;
        } else {
            return false;
        }
    }
}

export default toNative(AddMarketEntryModal);
</script>
