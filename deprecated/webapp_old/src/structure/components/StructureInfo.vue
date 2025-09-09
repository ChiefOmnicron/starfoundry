<template>
    <span>
        <n-spin v-if="busy" />

        <slot v-if="!busy" :busy="busy" :info="structure"></slot>
    </span>
</template>

<script lang="ts">
import { NSpin } from 'naive-ui';
import { Structure, StructureService } from '@/sdk/structure';
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

@Component({
    components: {
        NSpin,
    },
})
class StructureInfo extends Vue {
    /// UUID of the structure
    @Prop({
        type: String,
        required: true,
    })
    public id!: string;

    public busy: boolean = false;
    public structure: Structure = <any>{};

    public async created() {
        this.busy = true;
        this.structure = await StructureService.fetch(<any>this.id).finally(
            () => (this.busy = false),
        );
    }
}

export default toNative(StructureInfo);
</script>
