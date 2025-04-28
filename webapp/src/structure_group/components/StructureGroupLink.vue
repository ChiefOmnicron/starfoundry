<template>
    <n-button text type="info" v-if="group">
        <router-link
            :to="{
                name: route_group,
                params: { structureGroupId: groupId },
            }"
            style="color: inherit; text-decoration: none"
        >
            {{ group.name }}
        </router-link>
    </n-button>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NButton } from 'naive-ui';
import { type Uuid } from '@/sdk/utils';
import { ROUTE_STRUCTURE_GROUP } from '@/structure_group/router';
import { StructureGroup, StructureGroupService } from '@/sdk/structure_group';

@Component({
    components: {
        NButton,
    },
})
class StructureList extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public groupId!: Uuid;

    public group: StructureGroup = <any>null;
    public route_group: string = ROUTE_STRUCTURE_GROUP;

    public async created() {
        await StructureGroupService.fetch(this.groupId)
            .then((x) => (this.group = x))
            .catch((e) => {
                console.error(e);
            });
    }
}

export default toNative(StructureList);
</script>
