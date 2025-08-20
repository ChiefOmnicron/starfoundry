<template>
    <div>
        <div v-if="is_editing">
            <n-button-group>
                <n-button
                    ghost
                    type="error"
                    @click="save('WAITING_FOR_MATERIALS')"
                >
                    Needs Materials
                </n-button>
                <n-button ghost type="info" @click="save('BUILDING')">
                    Building
                </n-button>
                <n-button ghost type="success" @click="save('DONE')">
                    Done
                </n-button>
            </n-button-group>
        </div>
        <div v-else style="display: flex; justify-content: space-between">
            <n-tag type="success" v-if="stored_value === 'DONE'">Done</n-tag>
            <n-tag type="info" v-else-if="stored_value === 'BUILDING'"
                >Building</n-tag
            >
            <n-tag type="error" v-else>Needs Materials</n-tag>

            <n-icon @click="is_editing = true" size="16">
                <edit-regular />
            </n-icon>
        </div>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NButton, NButtonGroup, NIcon, NSelect, NTag } from 'naive-ui';
import { EditRegular } from '@vicons/fa';

import FormatNumber from '@/components/FormatNumber.vue';
import FormatNumberInput from '@/components/inputs/FormatNumber.vue';

@Component({
    components: {
        NButton,
        NButtonGroup,
        NIcon,
        NSelect,
        NTag,

        EditRegular,

        FormatNumber,
        FormatNumberInput,
    },
})
class EditableSelectComponent extends Vue {
    @Prop({
        type: String,
        required: false,
    })
    public default_value!: string;

    // Holds the selected system id
    public stored_value: string | null = null;
    public is_editing: boolean = false;

    public created() {
        this.stored_value = <string>this.default_value;
    }

    public save(status: string) {
        this.is_editing = false;
        this.stored_value = status;
        this.$emit('update:stored_value', this.stored_value);
    }
}

export default toNative(EditableSelectComponent);
</script>
