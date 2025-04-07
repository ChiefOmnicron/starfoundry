<template>
    <n-tooltip
        trigger="click"
    >
        <template #trigger>
            <item
                :disabled="disabled"
                :type-id="value"
                style="cursor: pointer;"
                v-if="item"
                v-slot="{ item }"
            >
                <span @click="copyToClipboard(item.name)">
                    {{ item.name }}
                </span>
            </item>

            <format-number
                :disabled="disabled"
                :nullable="nullable"
                :value="value || 0"
                :with-comma="withComma"
                @click="copyToClipboard(value || 0)"
                style="cursor: pointer;"
                v-else-if="number"
            />

            <n-icon
                v-else-if="icon"
                @click="copyToClipboard(value)"
            >
                <Copy />
            </n-icon>

            <span
                @click="copyToClipboard(value)"
                :style="{
                    cursor: 'pointer',
                    color: disabled ? 'rgba(255, 255, 255, 0.2)' : ''
                }"
                v-else
            >
                {{ value }}
            </span>
        </template>

        Copied!
    </n-tooltip>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NIcon, NTooltip } from 'naive-ui';

import { Copy } from '@vicons/fa';
import FormatNumber from '@/components/FormatNumber.vue';
import Item from '@/components/Item.vue';

@Component({
    components: {
        NTooltip,
        NIcon,

        Copy,

        FormatNumber,
        Item,
    }
})
class CopyText extends Vue {
    @Prop({
        required: true
    })
    public value!: string | number;

    @Prop({
        type: Boolean,
        default: false,
    })
    public number!: boolean;

    @Prop({
        type: Boolean,
        default: false,
    })
    public item!: boolean;

    @Prop({
        type: Boolean,
        default: false,
    })
    public project!: boolean;

    @Prop({
        type: Boolean,
        default: false,
    })
    public disabled!: boolean;

    @Prop({
        type: Boolean,
        default: false,
    })
    public nullable!: boolean;

    @Prop({
        type: Boolean,
        required: false,
        default: false
    })
    public withComma!: boolean;

    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public icon!: boolean;

    public copyToClipboard(value: string | number) {
        if (!value) {
            return;
        }

        if (Number.isFinite(Number(value))) {
            navigator.clipboard.writeText(Number(value).toFixed(2));
        } else {
            navigator.clipboard.writeText(<string>value);
        }
    }
}

export default toNative(CopyText);
</script>
