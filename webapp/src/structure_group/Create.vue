<template>
    <div>
        <page-header title="New Structure Group" />

        <card title="General information">
            <general-info :info="group" />
        </card>

        <card-margin />

        <card title="Structures">
            <structure-list
                v-model:structure-ids="group.structure_ids"
                with-selector
            />
        </card>

        <n-space justify="end">
            <n-button @click="$router.back()" quaternary>Cancel</n-button>

            <n-button
                @click="save"
                :disabled="!group.name || !group.structure_ids"
                type="info"
            >
                Create Structure Group
            </n-button>
        </n-space>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NForm, NFormItem, NInput, NSpace } from 'naive-ui';
import { events } from '@/main';
import { type IStructureCreateGroup, StructureGroupService } from '@/sdk/structure_group';

import { ROUTE_CHANGE } from '@/event_bus';
import { ROUTE_STRUCTURE_GROUPS } from './router';

import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import GeneralInfo from '@/structure_group/components/GeneralInfo.vue';
import PageHeader from '@/components/PageHeader.vue';
import StructureList from '@/structure/components/StructureList.vue';

@Component({
    components: {
        NButton,
        NForm,
        NFormItem,
        NInput,
        NSpace,

        Card,
        CardMargin,
        GeneralInfo,
        PageHeader,
        StructureList,
    }
})
class StructureGroupCreate extends Vue {
    public group: IStructureCreateGroup = <any>{
        structure_ids: [],
        tags: [],
    };

    public async created() {
        events.$emit(
            ROUTE_CHANGE,
            this.$route.name
        );
    }

    public async save() {
        await StructureGroupService
            .create(this.group)
            .then(_ => {
                this.$router.push({ name: ROUTE_STRUCTURE_GROUPS })
            })
            .catch(e => {
                console.error(e);
            });
    }
}

export default toNative(StructureGroupCreate);
</script>
