<template>
    <n-select
        :options="available_options()"
        :loading="loading"
        :render-label="render_entry"
        :render-tag="render_selected"
        @clear="options = []"
        v-model:value="value"
        clearable
        filterable
    />
</template>

<script lang="ts">
import { Component, Vue, toNative, Prop } from 'vue-facing-decorator';
import { NAvatar, NText, NSelect, type SelectOption } from 'naive-ui';
import { h } from 'vue';
import { Structure, StructureService } from '@/sdk/structure';
import type { StructureId } from '@/sdk/utils';

@Component({
    components: {
        NAvatar,
        NText,
        NSelect,
    }
})
class StructureSelector extends Vue {
    @Prop({
        default: [],
    })
    public exclude: StructureId[] = [];

    public options: SelectOption[] = [];
    public loading: boolean = false;

    public structures: { [key: string]: Structure } = {};

    // Holds the selected system id
    public value: number | null = null;

    public async created() {
        this.loading = true;
        await StructureService
            .list({})
            .then((x: Structure[]) => {
                x.map((x: Structure) => {
                    this.options.push({
                        label: x.name,
                        value: x.id,
                    });
                    this.structures[x.id] = x;
                });
            })
            .then((_: any) => this.value = <any>this.options[0].value)
            .then((_: any) => this.loading = false)
            .catch(e => {
                this.loading = false;
                console.error(e);
            });
    }

    public available_options(): SelectOption[] {
        return this.options.filter(x => this.exclude.indexOf(<StructureId>x.value) === -1);
    }

    public render_selected({ option }: any): string {
        return option.label;
    }

    public render_entry(option: SelectOption) {
        let structure = this.structures[<string>option.value];

        return h(
            'div',
            {
                style: {
                    display: 'flex',
                    alignItems: 'center'
                }
            },
            [
                h(
                    'div',
                    {
                        style: {
                            marginLeft: '12px',
                            padding: '4px 0'
                        }
                    },
                    [
                        h('div', null, [option.label as string]),
                        h(
                            NText,
                            { depth: 3, tag: 'div' },
                            {
                                default: () => `${structure.structureName}`
                            }
                        ),
                    ]
                )
            ]
        )
    }
}

export default toNative(StructureSelector);
</script>
