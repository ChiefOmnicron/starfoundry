<template>
    <n-select
        :loading="loading"
        :options="selectableOptions()"
        :render-label="renderEntry"
        :render-tag="renderSelected"
        @clear="options = []"
        clearable
        filterable
        v-model:value="value"
    />
</template>

<script lang="ts">
import { NSelect, type SelectOption } from 'naive-ui';
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { h } from 'vue';

import { ItemService, type IItem } from '@/services/item';
import type { TypeId } from '@/sdk/utils';

// Usage:
//
// ```html
// <item-selector
//     v-model:value="type_id"
// ></item-selector>
// ```
//
// Options:
// - buildable=true will filter only items that can be build using Blueprints
//
@Component({
    components: {
        NSelect,
    },
})
class ItemSelector extends Vue {
    @Prop({
        type: Boolean,
        default: false,
    })
    public buildable!: boolean;

    @Prop({
        type: Boolean,
        default: false,
    })
    public blueprints!: boolean;

    // If given, it will ignore already selected entries
    @Prop({
        type: Array,
        default: [],
        required: false,
    })
    public selected!: TypeId[];

    public options: SelectOption[] = [];
    public loading: boolean = false;

    // Holds the selected system id
    public value: number | null = null;

    public created() {
        //this.$watch('default', () => {
        this.options = [];
        this.loading = true;

        if (this.buildable) {
            ItemService.buildable_items()
                .then((x: IItem[]) => {
                    x.map((x: IItem) =>
                        this.options.push({
                            label: `${x.name}`,
                            value: x.type_id,
                        }),
                    );
                })
                .then((_: any) => (this.value = <any>this.options[0].value))
                .then((_: any) => (this.loading = false));
        } else if (this.blueprints) {
            ItemService.blueprints()
                .then((x: IItem[]) => {
                    x.map((x: IItem) =>
                        this.options.push({
                            label: `${x.name}`,
                            value: x.type_id,
                        }),
                    );
                })
                .then((_: any) => (this.value = <any>this.options[0].value))
                .then((_: any) => (this.loading = false));
        } else {
            ItemService.all()
                .then((x: IItem[]) => {
                    x.map((x: IItem) =>
                        this.options.push({
                            label: `${x.name}`,
                            value: x.type_id,
                        }),
                    );
                })
                .then((_: any) => (this.value = <any>this.options[0].value))
                .then((_: any) => (this.loading = false));
        }
        //});
    }

    public selectableOptions() {
        return this.options.filter(
            (x) => this.selected.indexOf(<number>x.value || 0) === -1,
        );
    }

    public renderSelected({ option }: any) {
        return this.renderEntry(option);
    }

    public renderEntry(option: any) {
        return h(
            'div',
            {
                style: {
                    display: 'flex',
                    alignItems: 'center',
                },
            },
            [option.label],
        );
    }
}

export default toNative(ItemSelector);
</script>
