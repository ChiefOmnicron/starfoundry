<template>
    <n-table>
        <thead>
            <tr>
                <th>Name</th>
                <th width="100px" v-if="withSelector"></th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="group_id in groupIds" :key="group_id">
                <td>
                    <structure-group-link :group-id="group_id" />
                </td>
                <td v-if="withSelector">
                    <n-button
                        ghost
                        type="error"
                        style="width: 100px"
                        @click="remove(group_id)"
                    >
                        Remove
                    </n-button>
                </td>
            </tr>
        </tbody>
        <tfoot v-if="withSelector">
            <tr>
                <td>
                    <structure-group-selector
                        :includeDynamicGroups="false"
                        :exclude="groupIds"
                        v-model:value="selected_group"
                    />
                </td>
                <td>
                    <n-button style="width: 100px" @click="add"> Add </n-button>
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

import StructureGroupLink from './StructureGroupLink.vue';
import StructureGroupSelector from '@/components/selectors/StructureGroupSelector.vue';

@Component({
    components: {
        NButton,
        NTable,

        StructureGroupLink,
        StructureGroupSelector,
    },
})
class StructureGroupList extends Vue {
    @Prop({
        type: Array<StructureId>,
        default: [],
        required: true,
    })
    public groupIds!: StructureId[];

    @Prop({
        type: Boolean,
        default: false,
    })
    public withSelector!: boolean;

    public selected_group = <any>null;
    public route_structure: string = ROUTE_STRUCTURE;

    public add() {
        this.groupIds.push(this.selected_group);
        this.selected_group = null;
    }

    public remove(group_id: Uuid) {
        let index = this.groupIds.findIndex((x) => x === group_id);
        this.groupIds.splice(index, 1);
    }
}

export default toNative(StructureGroupList);
</script>
