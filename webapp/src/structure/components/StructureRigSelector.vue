<template>
    <n-input-group>
        <n-select
            :disabled="!structureType || readonly"
            :options="options"
            @keydown.enter.prevent
            @update:value="selected"
            filterable
            placeholder="Select Rig"
            v-model:value="value"
        />
        <n-button
            :disabled="!value || readonly"
            @click="clear"
            data-cy="clearSelectedRig"
            ghost
            type="primary"
        >
            Clear
        </n-button>
    </n-input-group>
</template>

<script lang="ts">
import { Component, Vue, toNative, Prop, Watch } from 'vue-facing-decorator';

import type { TypeId } from '@/sdk/utils';
import { StructureService } from '@/sdk/structure';

import { NButton, NInputGroup, NSelect, type SelectOption } from 'naive-ui';

@Component({
    components: {
        NButton,
        NInputGroup,
        NSelect,
    },
    emits: [
        'update:value'
    ]
})
class StructureList extends Vue {
    @Prop({
        type: Number,
        required: true,
    })
    public structureType!: TypeId;

    @Prop({
        type: Number
    })
    public selectedValue!: TypeId;

    @Prop({
        default: true,
        type: Boolean,
    })
    public readonly!: boolean;

    public options: SelectOption[] = [];
    public value: any = undefined;

    public async mounted() {
        await this.rigs();

        if (this.selectedValue) {
            this.value = this.selectedValue;
        }
    }

    public clear() {
        this.value = null;
        this.$emit('update:value', null);
    }

    public selected(value: number) {
        this.value = value;
        this.$emit('update:value', this.value);
    }

    @Watch('structureType')
    public watch_structure_type() {
        this.rigs();
    }

    public async rigs() {
        if (!this.structureType) {
            return;
        }

        this.options = (await StructureService.possibleRigs(this.structureType))
            .map(x => {
                return {
                    label: x.name,
                    value: x.type_id
                }
            });
    }
}

export default toNative(StructureList);
</script>
