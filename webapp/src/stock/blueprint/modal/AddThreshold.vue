<template>
    <n-modal v-model:show="show" :on-update:show="close">
        <card
            :bordered="false"
            title="Blueprints that should be stocked"
            style="width: 900px"
        >
            <n-form
                :model="newEntry"
                :rules="rules"
                style="margin-left: 5px; margin-right: 5px; margin-top: 10px"
            >
                <n-form-item label="Item" path="type_id">
                    <item-selector
                        blueprints
                        v-model:value="newEntry.type_id"
                    />
                </n-form-item>

                <n-form-item
                    label="Number of blueprints that should be stocked"
                    path="want"
                >
                    <format-number-input
                        placeholder="Want"
                        style="width: 100%"
                        v-model:value="newEntry.want"
                    />
                </n-form-item>

                <n-form-item
                    label="Under this amount the stock is critical"
                    path="critical"
                >
                    <format-number-input
                        placeholder="Critical"
                        style="width: 100%"
                        v-model:value="newEntry.critical"
                    />
                </n-form-item>

                <n-form-item
                    label="Number of runs the BPC needs to have left"
                    path="min_runs"
                >
                    <format-number-input
                        placeholder="Min runs"
                        style="width: 100%"
                        v-model:value="newEntry.min_runs"
                    />
                </n-form-item>

                <n-form-item
                    label="Number of runs the BPC needs to have left"
                    path="min_me"
                >
                    <format-number-input
                        placeholder="Min Material Efficiency"
                        style="width: 100%"
                        v-model:value="newEntry.min_me"
                    />
                </n-form-item>

                <n-form-item
                    label="Minimum Time Efficiency the BPC needs to have"
                    path="min_te"
                >
                    <format-number-input
                        placeholder="Min Time Efficiency"
                        style="width: 100%"
                        v-model:value="newEntry.min_te"
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
    emits: ['close', 'close:value'],
})
class AddThresholdModal extends Vue {
    @Prop({
        type: Boolean,
        required: true,
    })
    public show!: boolean;

    public newEntry: IStockBlueprintThreshold = <any>{
        min_runs: 0,
        min_me: 0,
        min_te: 0,
    };

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
        want: [
            {
                type: 'number',
                required: true,
                message: 'The field is required',
            },
        ],
        critical: [
            {
                type: 'number',
                required: true,
                message: 'The field is required',
            },
        ],
        minRun: [
            {
                type: 'number',
                required: true,
                message: 'The field is required',
            },
        ],
        minMe: [
            {
                type: 'number',
                required: true,
                message: 'The field is required',
            },
        ],
        minTe: [
            {
                type: 'number',
                required: true,
                message: 'The field is required',
            },
        ],
    };

    public validForm(): boolean {
        if (this.newEntry.type_id && this.newEntry.critical) {
            return true;
        } else {
            return false;
        }
    }
}

export default toNative(AddThresholdModal);
</script>
