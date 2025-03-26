<template>
    <slot v-if="!busy" :structure-group="structure_group"></slot>
</template>

<script lang="ts">
import { Component, Vue, Prop, toNative } from 'vue-facing-decorator';

import { type StructureId } from '@/sdk/utils';
import { StructureGroup, StructureGroupService } from '@/sdk/structure_group';

@Component
class StructureWrapper extends Vue {
    @Prop
    public structureGroupId: StructureId = <any>null;
    public busy: boolean            = true;

    public structure_group: StructureGroup = <any>null;

    public async created() {
        await StructureGroupService
            .fetch(this.structureGroupId)
            .then(x => {
                this.structure_group = x;
                this.busy = false;
            })
            .catch(e => {
                this.busy = false;
                console.error(e);
            });
    }
}

export default toNative(StructureWrapper);
</script>
