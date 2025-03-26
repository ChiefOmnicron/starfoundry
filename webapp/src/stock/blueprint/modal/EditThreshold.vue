<template>
    <n-modal v-model:show="show" :on-update:show="close">
        <card
            :bordered="false"
            title="Blueprints that should be stocked"
            style="width: 900px"
        >
            <n-form
                :model="entry"
                :rules="rules"
                style="margin-left: 5px; margin-right: 5px; margin-top: 10px"
            >
                <n-form-item
                    label="Item"
                    path="type_id"
                >
                    <item-selector
                        blueprints
                        v-model:value="entry.type_id"
                    />
                </n-form-item>

                <n-form-item
                    label="Number of blueprints that should be stocked"
                    path="want"
                >
                    <format-number-input
                        placeholder="Want"
                        style="width: 100%"
                        v-model:value="entry.want"
                    />
                </n-form-item>

                <n-form-item
                    label="Under this amount the stock is critical"
                    path="critical"
                >
                    <format-number-input
                        placeholder="Critical"
                        style="width: 100%"
                        v-model:value="entry.critical"
                    />
                </n-form-item>

                <n-form-item
                    label="Number of runs the BPC needs to have left"
                    path="min_runs"
                >
                    <format-number-input
                        placeholder="Min runs"
                        style="width: 100%"
                        v-model:value="entry.min_runs"
                    />
                </n-form-item>

                <n-form-item
                    label="Minimum Material Efficiency the BPC needs to have"
                    path="min_me"
                >
                    <format-number-input
                        placeholder="Min Material Efficiency'"
                        style="width: 100%"
                        v-model:value="entry.min_me"
                    />
                </n-form-item>

                <n-form-item
                    label="Minimum Time Efficiency the BPC needs to have"
                    path="min_te"
                >
                    <format-number-input
                        placeholder="Min Time Efficiency"
                        style="width: 100%"
                        v-model:value="entry.min_te"
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
                    Edit
                </n-button>
            </action-group>
        </card>
    </n-modal>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { type FormRules, NButton, NForm, NFormItem, NInput, NModal } from 'naive-ui';

import { type IStockBlueprintThreshold } from '@/sdk/stockBlueprint';

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
    emits: [
        'close',
        'close:value',
    ]
})
class EditThresholdModal extends Vue {
    @Prop({
        type: Boolean,
        required: true,
    })
    public show!: boolean;

    @Prop({
        type: Object,
        required: true,
    })
    public entry!: IStockBlueprintThreshold;

    public close() {
        this.$emit('close', true);
    }

    public async addEntry() {
        this.$emit('close:value', this.entry);
    }

    public rules: FormRules = {
        type_id: [{
            required: true,
            message: 'The field is required',
        }],
        want: [{
            required: true,
            message: 'The field is required',
        }],
        critical: [{
            required: true,
            message: 'The field is required',
        }],
        minRun: [{
            required: true,
            message: 'The field is required',
        }],
        minMe: [{
            required: true,
            message: 'The field is required',
        }],
        minTe: [{
            required: true,
            message: 'The field is required',
        }],
    }

    public validForm(): boolean {
        if (this.entry.type_id &&
            this.entry.critical) {

            return true;
        } else {
            return false;
        }
    }
}

export default toNative(EditThresholdModal);
</script>
