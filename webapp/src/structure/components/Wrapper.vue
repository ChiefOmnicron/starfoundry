<template>
    <slot v-if="!busy" :structure="structure"></slot>
</template>

<script lang="ts">
import { Component, Vue, Prop, toNative } from 'vue-facing-decorator';

import type { StructureId } from '@/sdk/utils';
import { Structure, StructureService } from '@/sdk/structure';

@Component
class StructureWrapper extends Vue {
    @Prop({
        type: String,
    })
    public structureId!: StructureId;

    public busy: boolean            = true;

    public structure: Structure = <any>null;

    public async created() {
        await StructureService
            .fetch(this.structureId)
            .then(x => {
                this.structure = x;
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
