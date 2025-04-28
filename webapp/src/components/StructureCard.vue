<template>
    <n-grid v-if="structure" :cols="24">
        <n-grid-item>
            <eve-icon :id="structure.structureTypeId" />
        </n-grid-item>
        <n-grid-item span="10">
            <reference
                :params="{ structureId: structure.id }"
                :route="routeStructure"
            >
                {{ structure.name }}
            </reference>
        </n-grid-item>
        <n-grid-item span="6">
            <system :system-id="structure.systemId" dotlan />
        </n-grid-item>
    </n-grid>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { Structure, StructureService } from '@/sdk/structure';

import { ROUTE_STRUCTURE } from '@/structure/router';

import type { Uuid } from '@/sdk/utils';

import { NButton, NGrid, NGridItem } from 'naive-ui';
import { ExternalLinkAlt } from '@vicons/fa';
import EveIcon from '@/components/EveIcon.vue';
import Reference from './Reference.vue';
import System from '@/components/System.vue';

@Component({
    components: {
        NButton,
        NGrid,
        NGridItem,

        ExternalLinkAlt,

        EveIcon,
        Reference,
        System,
    },
})
class StructureCard extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public structureId!: string;

    public routeStructure: string = ROUTE_STRUCTURE;

    public structure!: Structure;

    public async created() {
        await StructureService.fetch(this.structureId).then((x) => {
            this.structure = x;
        });
    }

    public openStructure(structureId: Uuid) {
        let route = this.$router.resolve({
            name: ROUTE_STRUCTURE,
            params: {
                structureId,
            },
        });
        window.open(route.href);
    }
}

export default toNative(StructureCard);
</script>
