<template>
    <n-card content-style="padding: 0" data-cy="card" :style="[borderStyle()]">
        <div style="border-bottom: 1px solid #48484e" v-if="!noTitle">
            <n-space
                justify="space-between"
                style="
                    margin-left: 10px;
                    margin-right: 10px;
                    margin-top: 5px;
                    margin-bottom: 5px;
                "
            >
                <div>
                    <slot name="title">
                        <h2 data-cy="header" style="margin: 0px" v-if="title">
                            {{ title || '' }}
                        </h2>
                        <h3
                            data-cy="subheader"
                            style="margin: 0px"
                            v-if="!title && subtitle"
                        >
                            {{ subtitle || '' }}
                        </h3>
                    </slot>
                </div>

                <div>
                    <slot name="action" />
                </div>
            </n-space>
        </div>

        <slot name="description" />

        <slot />

        <slot name="footer" />
    </n-card>
</template>

<script lang="ts">
// Slots:
// - default: Content of the card
// - title: Replaces the default title with the given one
// - action: Action group in the header
// - footer: Adds it to the bottom of the Card
//
// Options:
// - title: Title of the Card - Translated
// - subtitle: Like the title, but smaller - Translated
// - danger: Adds a red border
// - noTitle: Doesn't show any title
// - description: Adds a description, before the content - Translated
//
// Example:
// <card title="Cool title">
//         <template #action>
//             <button-group />
//         </template>
//
//         <div>Default content</div>
//
//         <template #footer>
//         <n-button />
//         </template>
// </card>
//
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NCard, NSpace } from 'naive-ui';

@Component({
    components: {
        NCard,
        NSpace,
    },
})
class Card extends Vue {
    @Prop({
        required: false,
        type: String,
    })
    public title!: string;

    @Prop({
        required: false,
        type: String,
    })
    public subtitle!: string;

    @Prop({
        required: false,
        type: Boolean,
    })
    public noTitle!: boolean;

    @Prop({
        required: false,
        default: false,
        type: Boolean,
    })
    public danger!: boolean;

    public borderStyle(): {
        border?: string;
    } {
        if (this.danger) {
            return { border: '1px solid #e88080' };
        } else {
            return {};
        }
    }
}

export default toNative(Card);
</script>
