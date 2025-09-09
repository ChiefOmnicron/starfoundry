<template>
    <div>
        <page-header title="New Dynamic Structure Group" />

        <card title="General information">
            <general-info :info="data" />
        </card>

        <card-margin />

        <card title="Structure Groups">
            <structure-group-list
                v-model:group-ids="data.group_ids"
                with-selector
            />
        </card>

        <card-margin />

        <n-space justify="end">
            <n-button @click="$router.back()" quaternary>Cancel</n-button>

            <n-button @click="save" :disabled="!data.name" type="info">
                Create Structure Group
            </n-button>
        </n-space>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import {
    NButton,
    NForm,
    NFormItem,
    NInput,
    NList,
    NListItem,
    NScrollbar,
    NSpace,
    NTag,
    NThing,
    NTreeSelect,
} from 'naive-ui';
import { events } from '@/main';
import {
    type IStructureDynamicGroup,
    StructureDynamicGroupService,
} from '@/sdk/structure_dynamic_group';

import { ROUTE_CHANGE } from '@/event_bus';
import { ROUTE_STRUCTURE_DYNAMIC_GROUPS } from './router';

import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import GeneralInfo from '@/structure_group/components/GeneralInfo.vue';
import PageHeader from '@/components/PageHeader.vue';
import StructureGroupList from '@/structure_group/components/StructureGroupList.vue';

@Component({
    components: {
        NButton,
        NForm,
        NFormItem,
        NInput,
        NList,
        NListItem,
        NScrollbar,
        NSpace,
        NTag,
        NThing,
        NTreeSelect,

        Card,
        CardMargin,
        GeneralInfo,
        PageHeader,
        StructureGroupList,
    },
})
class StructureGroupCreate extends Vue {
    public new_group: any = null;

    public data: IStructureDynamicGroup = <any>{
        group_ids: [],
        tags: [],
    };

    public async created() {
        events.$emit(ROUTE_CHANGE, this.$route.name);
    }

    public async save() {
        await StructureDynamicGroupService.create(this.data)
            .then((_) => {
                this.$router.push({ name: ROUTE_STRUCTURE_DYNAMIC_GROUPS });
            })
            .catch((e) => {
                console.error(e);
            });
    }
}

export default toNative(StructureGroupCreate);
</script>
