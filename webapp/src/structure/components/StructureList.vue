<template>
    <n-table>
        <thead>
            <tr>
                <th width="400px">Name</th>
                <th width="300px">Location</th>
                <th width="200px">Type</th>
                <th width="400px">Services</th>
                <th width="700px">Rigs</th>
                <th width="100px" v-if="withSelector"></th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="structure_id in structureIds" :key="structure_id">
                <structure-wrapper
                    :structure-id="structure_id"
                    v-slot="{ structure }"
                >
                    <td>
                        <n-button text type="info">
                            <router-link :to="{
                                    name: route_structure,
                                    params: { structureId: structure.id }
                                }
                                " style="color: inherit; text-decoration: none">

                                {{ structure.name }}
                            </router-link>
                        </n-button>
                    </td>
                    <td>
                        <system :system-id="structure.system_id" dotlan></system>
                    </td>
                    <td>
                        {{ structure.type }}
                    </td>
                    <td>
                        <template v-for="service in structure.services">
                            {{ service.name }}<br>
                        </template>
                    </td>
                    <td>
                        <template v-for="rig in structure.rigs">
                            {{ rig.name }}<br>
                        </template>
                    </td>
                    <td v-if="withSelector">
                        <n-button
                            ghost
                            type="error"
                            style="width: 100px"
                            @click="remove(structure_id)"
                        >
                            Remove
                        </n-button>
                    </td>
                </structure-wrapper>
            </tr>
        </tbody>
        <tfoot v-if="withSelector">
            <tr>
                <td colspan="5">
                    <structure-selector
                        v-model:value="selected_structure"
                        :exclude="structureIds"
                    />
                </td>
                <td colspan="5">
                    <n-button
                        style="width: 100px"
                        @click="add"
                    >
                        Add
                    </n-button>
                </td>
            </tr>
        </tfoot>
    </n-table>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NButton, NTable } from 'naive-ui';
import type { StructureId, Uuid } from '@/sdk/utils';
import { ROUTE_STRUCTURE } from '@/structure/router';

import StructureSelector from '@/components/selectors/StructureSelector.vue';
import StructureWrapper from '@/structure/components/Wrapper.vue';
import System from '@/components/System.vue';

@Component({
    components: {
        NButton,
        NTable,

        StructureSelector,
        StructureWrapper,
        System,
    }
})
class StructureList extends Vue {
    @Prop({
        type: Array<StructureId>,
        default: [],
        required: true,
    })
    public structureIds!: StructureId[];

    @Prop({
        type: Boolean,
        default: false,
    })
    public withSelector!: boolean;

    public selected_structure = <any>null;
    public route_structure: string = ROUTE_STRUCTURE;

    public add() {
        this.structureIds.push(this.selected_structure);
        this.selected_structure = null;
    }

    public remove(structure_id: Uuid) {
        let index = this.structureIds.findIndex(x => x === structure_id);
        this.structureIds.splice(index, 1);
    }
}

export default toNative(StructureList);
</script>
