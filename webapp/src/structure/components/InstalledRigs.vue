<template>
    <card
        title="Installed Rigs"
    >
        <n-table>
            <tr v-for="index in [0, 1, 2]" :key="index">
                <th style="width: 150px">
                    {{ 'Slot ' + index + 1 }}
                </th>
                <td>
                    <structure-rig-selector
                        :readonly="readonly"
                        :structure-type="structureType"
                        :selected-value="value[index]"
                        @update:value="(v: any) => selectStructureRig(index, v)"
                    />
                </td>
            </tr>
        </n-table>
    </card>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { TypeId } from '@/sdk/utils';

import { NTable } from 'naive-ui';
import Card from '@/components/Card.vue';
import FormItem from '@/components/FormItem.vue';
import StructureRigSelector from '@/structure/components/StructureRigSelector.vue';

@Component({
    components: {
        NTable,

        Card,
        FormItem,
        StructureRigSelector,
    },
    emits: [
        'update:value'
    ]
})
class InstalledRigs extends Vue {
    @Prop({
        type: Number,
        required: true,
    })
    public structureType!: TypeId;

    @Prop({
        default: [],
        type: Array<IRigTypeIdName>,
        required: true,
    })
    public rigs!: IRigTypeIdName[];

    @Prop({
        default: true,
        type: Boolean,
    })
    public readonly!: boolean;

    public value: IRigTypeIdName[] = [];

    public created() {
        this.value = this.rigs;
    }

    public selectStructureRig(index: number, value: number) {
        if (!this.value) {
            this.value = [];
        }

        if (!value) {
            this.value[index] = <any>null;
            this.$emit('update:value', this.value);
            return;
        }

        this.value[index] = <any>value;
        this.$emit('update:value', this.value);
    }
}

export default toNative(InstalledRigs);

interface IRigTypeIdName {
    type_id: TypeId;
    name: string;
}
</script>
