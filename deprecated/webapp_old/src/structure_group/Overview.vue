<template>
    <div>
        <page-header title="Structure Groups" />

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
                        <th width="300px">Systems</th>
                        <th width="300px">Structures</th>
                        <th width="300px">Services</th>
                    </tr>
                </thead>
                <tbody>
                    <tr
                        v-for="group in structure_groups"
                        :key="group.structure_group_id"
                    >
                        <td>
                            {{ group.name }}
                        </td>
                        <td>
                            <template v-for="system in group.structure_systems">
                                <system
                                    :system-id="system"
                                    dotlan
                                    v-if="system"
                                /><br />
                            </template>
                        </td>
                        <td>
                            <template
                                v-for="structure in group.structure_types"
                            >
                                {{ structure }}<br />
                            </template>
                        </td>
                        <td>
                            <template
                                v-for="service in group.structure_services"
                            >
                                <item
                                    :type-id="service"
                                    v-slot="{ item }"
                                    v-if="service"
                                >
                                    {{ service_name(item.name) }}
                                </item>
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
import { StructureGroup, StructureGroupService } from '@/sdk/structure_group';

import { ROUTE_CHANGE } from '@/event_bus';
import {
    ROUTE_STRUCUTRE_GROUP_DYNAMIC_NEW,
    ROUTE_STRUCUTRE_GROUP_NEW,
} from './router';

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
    },
})
class StructureGroupsOverview extends Vue {
    public busy: boolean = false;

    public structure_groups: StructureGroup[] = [];

    public new_structure_group = ROUTE_STRUCUTRE_GROUP_NEW;
    public new_dynamic_structure_group = ROUTE_STRUCUTRE_GROUP_DYNAMIC_NEW;

    public async created() {
        events.$emit(ROUTE_CHANGE, this.$route.name);

        this.busy = true;
        await StructureGroupService.all()
            .then((x) => {
                this.structure_groups = x;
            })
            .catch((e: any) => console.error(e));
        this.busy = false;
    }

    public service_name(service: string): string {
        if (!service) {
            return '';
        }

        return service.replace('Standup ', '');
    }
}

export default toNative(StructureGroupsOverview);
</script>
