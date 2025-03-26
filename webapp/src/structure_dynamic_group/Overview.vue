<template>
    <div>
        <page-header title="Dynamic Structure Groups" />

        <card content-style="padding: 0" v-if="!busy">
            <template #action>
                <n-button
                    @click="$router.push({ name: new_structure_group })"
                    type="info"
                >
                    New structure group
                </n-button>
            </template>

            <n-table v-if="!busy && structure_groups.length > 0">
                <thead>
                    <tr>
                        <th width="300px">Name</th>
                        <th width="300px">Manufacturing</th>
                        <th width="300px">Reaction</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="group in structure_groups" :key="group.id">
                        <td>
                            {{ group.name }}
                        </td>
                        <td>
                            <template v-for="manufacturing in group.manufacturing">
                                {{ filter_name(manufacturing) }}<br>
                            </template>
                        </td>
                        <td>
                            <template v-for="reaction in group.reaction">
                                {{ filter_name(reaction) }}<br>
                            </template>
                        </td>
                    </tr>
                </tbody>
            </n-table>

            <no-entries
                description="No structures groups yet"
                v-if="!busy && structure_groups.length === 0"
            />

            <loader description="Loading" :busy="busy" />
        </card>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NTable } from 'naive-ui';

import { events } from '@/main';

import { StructureDynamicGroup, StructureDynamicGroupService } from '@/sdk/structure_dynamic_group';

import { ROUTE_CHANGE } from '@/event_bus';
import { ROUTE_STRUCUTRE_GROUP_NEW } from './router';

import Card from '@/components/Card.vue';
import Item from '@/components/Item.vue';
import Loader from '@/components/Loader.vue';
import NoEntries from '@/components/NoEntries.vue';
import PageHeader from '@/components/PageHeader.vue';
import StructureGroupWrapper from '@/structure_group/components/Wrapper.vue';
import System from '@/components/System.vue';

@Component({
    components: {
        NButton,
        NTable,

        Card,
        Item,
        Loader,
        NoEntries,
        PageHeader,
        StructureGroupWrapper,
        System,
    }
})
class StructureDynamicGroupsOverview extends Vue {
    public busy: boolean = false;

    public structure_groups: StructureDynamicGroup[] = [];

    public new_structure_group = ROUTE_STRUCUTRE_GROUP_NEW;

    public async created() {
        events.$emit(
            ROUTE_CHANGE,
            this.$route.name
        );

        this.busy = true;
        await StructureDynamicGroupService
            .all()
            .then(x => {
                this.structure_groups = x
            })
            .catch((e: any) => console.error(e));
        this.busy = false;
    }

    public filter_name(filter: string): string {
        switch(filter) {
            case 'structure_raitaru':
                return 'Raitaru';
            case 'structure_azbel':
                return 'Azbel';
            case 'structure_sotiyo':
                return 'Sotiyo';
            case 'structure_athanor':
                return 'Athanor';
            case 'structure_tatara':
                return 'Tatara';
            default:
                return '';
        }
    }
}

export default toNative(StructureDynamicGroupsOverview);
</script>
