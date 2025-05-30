<template>
    <div>
        <n-input type="textarea" v-model:value="list" :rows="10" disabled />

        <action-group style="margin-top: 5px; margin-right: 5px">
            <n-button @click="close()" quaternary> Close </n-button>
            <n-tooltip placement="top" trigger="click">
                <template #trigger>
                    <n-button @click="copyList">Copy</n-button>
                </template>

                <span>Copied!</span>
            </n-tooltip>
        </action-group>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NButton, NInput, NTooltip } from 'naive-ui';
import ActionGroup from './ActionGroup.vue';

@Component({
    components: {
        NButton,
        NInput,
        NTooltip,

        ActionGroup,
    },
    emits: ['close'],
})
class ItemExport extends Vue {
    /// String array of the fields to show
    @Prop({
        required: true,
    })
    public formatIngame!: Array<any>;

    /// String array of the fields to show
    @Prop({
        required: true,
    })
    public formatList!: Array<any>;

    @Prop({
        required: true,
    })
    public data!: Array<any>;

    public list: string = '';

    public created() {
        this.generateList(this.data);
        this.$watch('items', () => this.generateList(this.data), {
            deep: true,
        });
    }

    public copyList() {
        navigator.clipboard.writeText(this.list);
    }

    public copyIngame() {
        let copy = this.data
            .map(
                (x: any) =>
                    `<url=showinfo:${x[<any>this.formatIngame[0]]}>${x[<any>this.formatIngame[1]]}</url>`,
            )
            .join('\n');
        navigator.clipboard.writeText(copy);
    }

    private generateList(data: any[]) {
        this.list = data
            .map((entry) =>
                (<string[]>this.formatList)
                    .map((field: string) => {
                        if (field.indexOf('.') > 0) {
                            let splitted = field.split('.');
                            let currentEntry = entry;

                            for (let split of splitted) {
                                currentEntry = currentEntry[split];
                            }

                            if (!currentEntry) {
                                return '';
                            }

                            return currentEntry;
                        }

                        if (!entry[field]) {
                            return '';
                        }

                        return `${entry[field]}`;
                    })
                    .join('\t'),
            )
            .join('\n');
    }

    public close() {
        this.$emit('close', true);
    }
}

export default toNative(ItemExport);
</script>
