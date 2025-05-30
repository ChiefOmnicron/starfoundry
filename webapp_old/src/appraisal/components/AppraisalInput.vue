<template>
    <n-input
        type="textarea"
        placeholder="Insert the items that should be appraised.
Either copy and paste from your inventory or insert fits.

Format:
Tritanium x1000
Isogen x1000"
        rows="10"
        @update:value="updateValue"
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NButton, NInput } from 'naive-ui';

@Component({
    components: {
        NButton,
        NInput,
    },
    emits: ['update:value'],
})
class AppraisalInput extends Vue {
    @Prop({
        type: String,
        required: false,
    })
    public existing!: string;

    public value: String = '';

    public created() {
        if (this.existing) {
            this.value = this.existing;
        }
    }

    public updateValue(value: string) {
        this.$emit('update:value', value);
    }
}

export default toNative(AppraisalInput);
</script>
