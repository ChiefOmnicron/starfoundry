<template>
    <div style="padding-bottom: 18px">
        <label style="padding-left: 2px">
            {{ label }}
        </label>

        <label v-if="required" style="color: #e88080"> * </label>

        <label v-if="info"> &nbsp;|&nbsp; </label>

        <n-button @click="showInfo = true" text type="info" v-if="info">
            Info
        </n-button>

        <slot></slot>

        <n-modal v-model:show="showInfo" v-if="info">
            <slot name="help"></slot>
        </n-modal>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NButton, NModal } from 'naive-ui';

@Component({
    components: {
        NButton,
        NModal,
    },
})
class FormItem extends Vue {
    public showInfo: boolean = false;

    @Prop({
        required: true,
        type: String,
    })
    public label!: string;

    @Prop({
        default: false,
        required: false,
        type: Boolean,
    })
    public info!: boolean;

    @Prop({
        default: false,
        required: false,
        type: Boolean,
    })
    public required!: boolean;
}

export default toNative(FormItem);
</script>
