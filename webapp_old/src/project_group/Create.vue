<template>
    <div>
        <page-header title="New Project Group" />

        <card title="General">
            <n-form
                ref="form_ref"
                style="margin-top: 15px; margin-left: 10px; margin-right: 10px"
                :model="new_project_group"
                :rules="rules"
            >
                <n-form-item path="name" label="Name">
                    <n-input
                        v-model:value="new_project_group.name"
                        placeholder="Name"
                    />
                </n-form-item>

                <n-form-item path="orderer" label="Description">
                    <n-input
                        v-model:value="new_project_group.description"
                        placeholder="Description"
                        type="textarea"
                    />
                </n-form-item>
            </n-form>
        </card>

        <card-margin />

        <n-space justify="end">
            <n-button @click="$router.back()" quaternary>Cancel</n-button>

            <n-button
                @click="create"
                :disabled="!new_project_group.name"
                type="info"
            >
                Create Group
            </n-button>
        </n-space>
    </div>
</template>

<script lang="ts">
import { events } from '@/main';
import { ROUTE_CHANGE } from '@/event_bus';
import { Component, Vue, toNative } from 'vue-facing-decorator';
import {
    type FormRules,
    NButton,
    NCard,
    NForm,
    NFormItem,
    NInput,
    NInputNumber,
    NSpace,
    NText,
} from 'naive-ui';

import { ROUTE_PROJECT_GROUP } from '@/project_group/router';
import {
    ProjectGroupService,
    type ICreateProjectGroup,
} from '@/project_group/service';

import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NButton,
        NCard,
        NForm,
        NFormItem,
        NInput,
        NInputNumber,
        NSpace,
        NText,

        Card,
        CardMargin,
        PageHeader,
    },
})
class ProjectGroupNew extends Vue {
    public formRef: any = null;
    public products_input: string = '';
    public additional_products_input: string = '';

    public system_id: any = null;

    public stock: string = '';

    public new_project_group: ICreateProjectGroup = <any>{};

    public async created() {
        events.$emit(ROUTE_CHANGE, this.$route.name);
    }

    public async create() {
        let groupId = '';

        ProjectGroupService.create(this.new_project_group)
            .then((x) => {
                console.log(x);
                groupId = x;
                this.$router.push({
                    name: ROUTE_PROJECT_GROUP,
                    params: {
                        groupId,
                    },
                });
            })
            .catch((e) => {
                console.error(e);
            });
    }

    public rules: FormRules = {
        name: [
            {
                required: true,
                message: 'The field is required',
            },
        ],
    };
}

export default toNative(ProjectGroupNew);
</script>
