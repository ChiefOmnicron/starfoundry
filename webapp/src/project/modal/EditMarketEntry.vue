<template>
    <n-modal v-model:show="show" :on-update:show="close">
        <card
            :bordered="false"
            title="Market Entry"
            style="width: 900px"
        >
            <n-form
                :model="newEntry"
                :rules="rules"
                style="margin-left: 5px; margin-right: 5px; margin-top: 10px"
            >
                <n-form-item
                    label="Item"
                    path="type_id"
                >
                    <eve-icon :id="newEntry.type_id" />
                    <item :type-id="newEntry.type_id" v-slot="{ item }">
                        {{ item.name }}
                    </item>
                </n-form-item>

                <n-form-item
                    label="Quantity"
                    path="quantity"
                >
                    <format-number-input
                        placeholder="Insert Quantity"
                        style="width: 100%"
                        v-model:value="newEntry.quantity"
                    />
                </n-form-item>

                <n-form-item
                    label="Cost"
                    path="cost"
                >
                    <format-number-input
                        placeholder="Insert cost"
                        style="width: 100%"
                        v-model:value="newEntry.cost"
                    />
                </n-form-item>

                <n-form-item
                    label="Source"
                    path="source"
                >
                    <n-input
                        placeholder="Insert Source"
                        style="width: 100%"
                        v-model:value="newEntry.source"
                    />
                </n-form-item>
            </n-form>

            <action-group style="margin-right: 5px">
                <n-button
                    @click="close()"
                    quaternary
                >
                    Close
                </n-button>

                <n-button
                    @click="addEntry"
                    :disabled="!validForm()"
                    type="info"
                >
                    Update
                </n-button>
            </action-group>
        </card>
    </n-modal>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { type FormRules, NButton, NForm, NFormItem, NInput, NModal } from 'naive-ui';

import { type IMarket } from '@/sdk/project';

import ActionGroup from '@/components/ActionGroup.vue';
import Card from '@/components/Card.vue';
import EveIcon from '@/components/EveIcon.vue';
import FormatNumberInput from '@/components/inputs/FormatNumber.vue';
import Item from '@/components/Item.vue';

@Component({
    components: {
        NButton,
        NForm,
        NFormItem,
        NInput,
        NModal,

        ActionGroup,
        Card,
        EveIcon,
        FormatNumberInput,
        Item,
    },
    emits: [
        'close',
        'close:value',
    ]
})
class EditMarketEntryModal extends Vue {
    @Prop({
        type: Boolean,
        required: true,
    })
    public show!: boolean;

    @Prop({
        type: Object,
        required: true,
    })
    public oldEntry!: IMarket;

    public newEntry: IMarket = <any>{};

    public created() {
        this.newEntry = JSON.parse(JSON.stringify(this.oldEntry));
    }

    public close() {
        this.$emit('close', true);
        this.newEntry = <any>{};
    }

    public async addEntry() {
        this.$emit('close:value', this.newEntry);
        this.newEntry = <any>{};
    }

    public rules: FormRules = {
        quantity: [{
            required: true,
            message: 'The field is required',
        }],
        cost: [{
            required: true,
            message: 'The field is required',
        }],
        source: [{
            required: true,
            message: 'The field is required',
        }],
    }

    public validForm(): boolean {
        if (this.newEntry.quantity &&
            (this.newEntry.cost || false) &&
            (this.newEntry.source || false)) {

            return true;
        } else {
            return false;
        }
    }
}

export default toNative(EditMarketEntryModal);
</script>
