<template>
    <n-select
        :loading="busy"
        :options="options()"
        @update:value="updateValue"
        data-cy="StructureMarketSelector"
        filterable
        placeholder="Select market"
        v-model:value="value"
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NText, NSelect, type SelectOption } from 'naive-ui';
import { Structure, StructureService } from '@/sdk/structure';

@Component({
    components: {
        NText,
        NSelect,
    },
    emits: [
        'error',
        'update:value',
    ]
})
class StructureMarketSelector extends Vue {
    @Prop({
        type: String,
        required: false,
    })
    public structure!: number;

    @Prop({
        default: [],
        required: false,
        type: Array<String>,
    })
    public selectedStructures!: number[];

    @Prop({
        default: false,
        required: false,
        type: Boolean,
    })
    public showNpcMarkets!: boolean;

    public npcStructureOptions: SelectOption[] = [{
        label: 'Jita 4-4',
        value: '00000000-0000-0000-0000-000000000001',
    }, {
        label: 'Amarr',
        value: '00000000-0000-0000-0000-000000000002',
    }];

    public availableOptions: SelectOption[] = [];
    public value: string | null = null;

    public busy: boolean = false;

    public async created() {
        if (this.showNpcMarkets) {
            this.availableOptions.push(...this.npcStructureOptions);
        }

        this.busy = true;
        await StructureService
            .list({
                service_id: 35892
            })
            .then((x: Structure[]) => {
                this.busy = false;

                x.map((x: Structure) => {
                    this.availableOptions.push({
                        label: x.name,
                        value: x.id,
                    });
                });
            })
            .catch(e => {
                this.busy = false;
                this.$emit('error', true)
            });
    }

    public updateValue(value: string) {
        this.value = value;
        this.$emit('update:value', this.value);
    }

    public options(): SelectOption[] {
        return this.availableOptions
            .filter(x => this.selectedStructures.indexOf(<number> x.value) === -1);
    }
}

export default toNative(StructureMarketSelector);
</script>
