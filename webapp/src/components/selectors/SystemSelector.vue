<template>
    <n-select
        :loading="loading"
        :options="options"
        :render-label="render_entry"
        :render-tag="render_selected"
        @clear="options = []"
        @search="handle_search"
        clearable
        filterable
        placeholder="Select System"
        remote
        @update:value="handleSelect"
        v-model="value"
    />
</template>

<script lang="ts">
import { NSelect, type SelectOption } from 'naive-ui';
import { Component, Prop, Vue, Watch, toNative } from 'vue-facing-decorator';
import { h } from 'vue';

import { type ISystem, SystemService } from '@/services/system';

// Usage:
//
// ```html
// <system-selector
//     v-model:value="system_id"
// ></system-selector>
// ```
//
// The variable `system_id` holds the selected id of the system
//
@Component({
    components: {
        NSelect
    },
    emits: ['update:info']
})
class SystemSelector extends Vue {
    @Prop({
        type: Number,
    })
    public default!: number;

    public options: SelectOption[] = [];
    public loading: boolean        = false;

    // Holds the selected system id
    public value: ISystem | null = null;
    public systems: ISystem[] = [];

    public async created() {
        await SystemService
            .search_by_id(this.default)
            .then((x: ISystem[]) => {
                this.systems = x;
                x.map((x: ISystem) => this.options.push({
                    label: `${x.system_name} (${x.region_name})`,
                    value:  x.system_id
                }));
            })
            //.then((_: any) => this.value = <any>this.options[0].value)
            .then((_: any) => this.loading = false);
    }

    @Watch('default')
    public watchDefault() {
        this.options = [];
        this.loading = true;

        SystemService
            .search_by_id(this.default)
            .then((x: ISystem[]) => {
                this.systems = x;
                x.map((x: ISystem) => this.options.push({
                    label: `${x.system_name} (${x.region_name})`,
                    value:  x.system_id
                }));
            })
            //.then((_: any) => this.value = <any>this.options[0].value)
            .then((_: any) => this.loading = false);
    }

    public async handle_search(query: string) {
        this.options = [];
        this.loading = true;

        SystemService
        .search_by_name(query)
        .then((x: ISystem[]) => {
                this.systems = x;
            x.map((x: ISystem) => this.options.push({
            label: `${x.system_name} (${x.region_name})`,
            value:  x.system_id
            }));
        })
        .then((_: any) => this.loading = false);
    }

    public render_selected({ option }: any) {
        return this.render_entry(option);
    }

    public render_entry(option: any) {
        let label = option.label;
        if (option.label === '[object Object]') {
            label = `${option.value.system_name} (${option.value.region_name})`;
        }

        return h(
                'div',
                {
                    style: {
                        display: 'flex',
                        alignItems: 'center'
                    }
                },
                [
                    label
                ]
            )
    }

    public handleSelect(value: string) {
        this.$emit('update:info', this.systems.find(x => x.system_id === value) || null);
    }
}

export default toNative(SystemSelector);
</script>
