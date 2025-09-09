<template>
    <n-list style="margin-left: 10px">
        <n-list-item>
            <n-thing
                title="Resolved structures"
                description="Lists all structures that are included in the Filter"
            >
                <n-scrollbar style="max-height: 120px">
                    <template
                        v-for="structure in resolved_structures"
                        :key="structure.id"
                    >
                        {{ structure.name }} ({{ structure.type }})<br />
                    </template>
                </n-scrollbar>
            </n-thing>
        </n-list-item>
    </n-list>
</template>

<script lang="ts">
import { Component, Vue, toNative, Watch, Prop } from 'vue-facing-decorator';
import {
    NList,
    NListItem,
    NScrollbar,
    NThing,
    SelectGroupOption,
} from 'naive-ui';
import {
    IStructureCreateGroup,
    StructureGroupService,
} from '@/sdk/structure_group';

import { Structure } from '@/sdk/structure';

@Component({
    components: {
        NList,
        NListItem,
        NScrollbar,
        NThing,
    },
})
class StructureGroupResolve extends Vue {
    @Prop({
        required: true,
    })
    public tags!: string[];

    @Prop({
        required: true,
    })
    public resolver!: (tags: string[]) => Structure[];

    public resolved_structures: Structure[] = [];

    public async created() {
        await this.resolve_structures();
    }

    @Watch('tags')
    public async resolve_structures() {
        this.resolved_structures = await this.resolver(this.tags);
    }
}

export default toNative(StructureGroupResolve);
</script>
