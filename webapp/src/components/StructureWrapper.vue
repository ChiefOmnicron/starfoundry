<template>
    <slot :entry="entry"></slot>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { Structure, StructureService } from '@/sdk/structure';
import type { Uuid } from '@/sdk/utils';

@Component({
    components: {},
    emits: ['loading', 'loading:error'],
})
class StructureWrapper extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public structureId!: Uuid;

    public entry: Structure = <any>{};

    public async created() {
        this.$emit('loading', true);

        await StructureService.fetch(this.structureId)
            .then((x) => {
                this.entry = x;
                this.$emit('loading', false);
            })
            .catch((_: any) => {
                this.$emit('loading:error', 'Error loading item');
            });
    }
}

export default toNative(StructureWrapper);
</script>
