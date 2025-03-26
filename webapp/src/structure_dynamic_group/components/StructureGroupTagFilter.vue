<template>
    <n-tree-select
        multiple
        cascade
        checkable
        :options="filter"
        v-model:value="tags"
        check-strategy="child"
        placeholder="Select tags"
    />
</template>

<script lang="ts">
import { Component, Vue, toNative, Prop, Watch } from 'vue-facing-decorator';
import { NList, NListItem, NScrollbar, NThing, NTreeSelect, SelectGroupOption } from 'naive-ui';
import { IStructureCreateGroup } from '@/sdk/structure_group';

import { Structure } from '@/sdk/structure';

@Component({
    components: {
        NList,
        NListItem,
        NScrollbar,
        NThing,
        NTreeSelect,
    },
    emits: ['update:tags']
})
class StructureGroupTagFilter extends Vue {
    @Prop({
        required: true,
    })
    public filter!: SelectGroupOption[];

    @Prop({
        default: []
    })
    public defaultSelected!: string[];

    public tags: string[]                   = this.defaultSelected;
    public resolved_structures: Structure[] = [];

    public group: IStructureCreateGroup = <any>{
        structure_ids: [],
        tags: [],
    };

    @Watch('tags')
    public created() {
        this.$emit('update:tags', this.tags);
    }
}

export default toNative(StructureGroupTagFilter);
</script>
