<template>
    <card title="Installed Services">
        <n-table>
            <tr
                v-for="index in [
                    ...Array.from(Array(countServiceModules()).keys()),
                ]"
                :key="index"
            >
                <th style="width: 150px">
                    {{ 'Slot ' + index + 1 }}
                </th>
                <td>
                    <structure-service-selector
                        :readonly="readonly"
                        :structure-type="structureType"
                        :selected-value="value[index]"
                        @update:value="
                            (v: any) => selectStructureService(index, v)
                        "
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
import StructureServiceSelector from '@/structure/components/StructureServiceSelector.vue';

@Component({
    components: {
        NTable,

        Card,
        FormItem,
        StructureServiceSelector,
    },
    emits: ['update:value'],
})
class InstalledServices extends Vue {
    @Prop({
        type: Number,
        required: true,
    })
    public structureType!: TypeId;

    @Prop({
        default: [],
        type: Array<TypeId>,
        required: true,
    })
    public services!: TypeId[];

    @Prop({
        default: true,
        type: Boolean,
    })
    public readonly!: boolean;

    public value: TypeId[] = [];

    public created() {
        this.value = this.services;
    }

    public selectStructureService(index: number, value: number) {
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

    public countServiceModules(): number {
        switch (this.structureType) {
            // Azbel
            case 35826:
            // Tatara
            case 35836:
            // Fortizar
            case 35833:
            // 'Prometheus' Fortizar
            case 47516:
            // 'Moreau' Fortizar
            case 47512:
            // 'Marginis' Fortizar
            case 47515:
            // 'Horizon' Fortizar
            case 47514:
            // 'Draccous' Fortizar
            case 47513:
                return 5;
            // Sotiyo
            case 35827:
                return 6;
            // Keepstar
            case 35834:
                return 7;
            // Raitaru
            // Athanor
            // Astrahus
            default:
                return 7;
        }
    }
}

export default toNative(InstalledServices);
</script>
