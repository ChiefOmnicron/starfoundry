<template>
    <div v-if="structure">
        <slot :structure="structure"></slot>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { type IStructureResolve, StructureService } from '@/sdk/structure';

@Component({
    components: {},
})
class Structure extends Vue {
    // Structure Id of the structure
    @Prop({
        type: Number,
        required: true,
    })
    public sid!: number;

    public structure: IStructureResolve = <any>{};

    public async created() {
        this.structure = await StructureService.resolve(this.sid);
    }
}

export default toNative(Structure);
</script>
