<template>
    <n-select
        :options="availableOptions()"
        :loading="loading"
        :render-label="renderEntry"
        :render-tag="renderSelected"
        @clear="options = []"
        v-model:value="value"
        clearable
        filterable
    />
</template>

<script lang="ts">
import { Component, Vue, toNative, Prop } from 'vue-facing-decorator';
import { h } from 'vue';

import type { StructureId, TypeId } from '@/sdk/utils';
import { Structure, StructureService } from '@/sdk/structure';

import { NAvatar, NText, NSelect, type SelectOption } from 'naive-ui';

@Component({
    components: {
        NAvatar,
        NText,
        NSelect,
    },
})
class StructureSelector extends Vue {
    @Prop({
        type: Array,
        default: [],
    })
    public exclude!: StructureId[];

    // service that the structure needs to have
    @Prop({
        type: Number,
        default: undefined,
        required: false,
    })
    public service!: TypeId;

    @Prop({
        default: [],
        required: false,
        type: Array<String>,
    })
    public selectedStructures!: number[];

    public options: SelectOption[] = [];
    public loading: boolean = false;

    public structures: { [key: string]: Structure } = {};

    // Holds the selected system id
    public value: number | null = null;

    public async created() {
        this.loading = true;
        await StructureService.list({
            service_id: this.service ? this.service : undefined,
        })
            .then((x: Structure[]) => {
                x.map((x: Structure) => {
                    this.options.push({
                        label: x.name,
                        value: x.id,
                    });
                    this.structures[x.id] = x;
                });
            })
            .then((_: any) => (this.value = <any>this.options[0].value))
            .then((_: any) => (this.loading = false))
            .catch((e) => {
                this.loading = false;
                console.error(e);
            });
    }

    public availableOptions(): SelectOption[] {
        return this.options.filter(
            (x) => this.selectedStructures.indexOf(<number>x.value) === -1,
        );
    }

    public renderSelected({ option }: any): string {
        return option.label;
    }

    public renderEntry(option: SelectOption) {
        let structure = this.structures[<string>option.value];

        return h(
            'div',
            {
                style: {
                    display: 'flex',
                    alignItems: 'center',
                },
            },
            [
                h(
                    'div',
                    {
                        style: {
                            marginLeft: '12px',
                            padding: '4px 0',
                        },
                    },
                    [
                        h('div', null, [option.label as string]),
                        h(
                            NText,
                            { depth: 3, tag: 'div' },
                            {
                                default: () => `${structure.structureName}`,
                            },
                        ),
                    ],
                ),
            ],
        );
    }
}

export default toNative(StructureSelector);
</script>
