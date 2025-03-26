<template>
    <n-button
        @click="openStructure"
        icon-placement="right"
        quaternary
        type="info"
    >
        <template #icon>
            <n-icon size="10"><external-link-alt /></n-icon>
        </template>

        <slot></slot>
    </n-button>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { ROUTE_STRUCTURE } from '@/structure/router';
import type { Uuid } from '@/sdk/utils';

import { NButton, NIcon } from 'naive-ui';
import { ExternalLinkAlt } from '@vicons/fa';

@Component({
    components: {
        NButton,
        NIcon,

        ExternalLinkAlt,
    }
})
class StructureReference extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public structureId!: Uuid;

    public openStructure() {
        let route = this.$router.resolve({
            name: ROUTE_STRUCTURE,
            params: {
                structureId: this.structureId,
            }
        });
        window.open(route.href);
    }
}

export default toNative(StructureReference);
</script>
