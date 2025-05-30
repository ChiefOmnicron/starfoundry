<template>
    <div>
        <div v-if="is_editing">
            <n-input-group>
                <n-input-number
                    v-model:value="stored_value"
                    :default-value="stored_value"
                    :parse="parse"
                    :format="format"
                    style="width: 100%"
                />
                <n-button type="info" ghost @click="save"> Save </n-button>
                <n-button ghost @click="cancel"> Cancel </n-button>
            </n-input-group>
        </div>
        <div v-else style="display: flex; justify-content: space-between">
            <format-number :value="stored_value" v-if="stored_value" />

            <span v-else>0</span>

            <n-icon @click="is_editing = true" size="16">
                <edit-regular />
            </n-icon>
        </div>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { EditRegular } from '@vicons/fa';
import { NButton, NIcon, NInputGroup, NInputNumber } from 'naive-ui';

import { formatNumber } from '@/utils';

import FormatNumber from '@/components/FormatNumber.vue';
import FormatNumberInput from '@/components/inputs/FormatNumber.vue';

@Component({
    components: {
        NButton,
        NIcon,
        NInputGroup,
        NInputNumber,

        EditRegular,

        FormatNumber,
        FormatNumberInput,
    },
})
class EditableNumberComponent extends Vue {
    @Prop
    public default_value: number | undefined;

    // Holds the selected system id
    public stored_value: number | null = null;
    public is_editing: boolean = false;

    private old_value: number | null = null;

    public created() {
        this.stored_value = <number>this.default_value;
        this.old_value = <number>this.default_value;
    }

    public save() {
        this.is_editing = false;
        this.old_value = this.stored_value;
        this.$emit('update:stored_value', this.stored_value);
    }

    public cancel() {
        this.stored_value = this.old_value;
        this.is_editing = false;
    }

    public format(input: number | null): string {
        if (!input || Number.isNaN(input)) {
            return '';
        }
        return formatNumber(input);
    }

    public parse(input: string): number | null {
        if (!input) {
            return 0;
        }

        const nums = input.replace(/\./g, '').trim();
        if (/^\d+(\.(\d+)?)?$/.test(nums)) {
            return Number(nums);
        }
        return nums === '' ? null : Number.NaN;
    }
}

export default toNative(EditableNumberComponent);
</script>
