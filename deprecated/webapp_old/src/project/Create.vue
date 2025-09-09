<template>
    <div>
        <page-header title="New Project Assistent" />

        <card title="General">
            <p style="margin-left: 10px">
                General information about the project.
            </p>

            <general-info
                v-model:info="newProject"
                :show-status="false"
                @update:projectGroup="selectProjectGroupId"
            />
        </card>

        <card-margin />

        <card title="Structure">
            <p style="margin-left: 10px">
                Structure group that should be used

                <structure-group-selector
                    @update:value="selectStructureGroup"
                    v-model:value="newProject.structure_group_id"
                />

                <div
                    v-for="structure in structures"
                    :key="structure.id"
                    style="margin-top: 10px"
                >
                    <structure-card :structure-id="structure.id" />
                </div>
            </p>
        </card>

        <card-margin />

        <card title="Markets" v-if="projectGroupId">
            <p style="margin-left: 10px">
                NPC Markets that should be used for market recommendation.
                <br>
                <template v-if="projectGroupId !== '00000000-0000-0000-0000-000000000000'">
                    The default NPC markets are set from the configured defaults of the
                    <reference
                        :params="{ groupId: projectGroupId }"
                        :route="routeProjectGroupDetail"
                    >
                        Project Group
                    </reference>
                </template>
                <template v-else>
                    Ypu can set the default values by creating a
                    <reference
                        :route="routeProjectGroup"
                    >
                        Project Group
                    </reference>
                </template>
            </p>

            <market
                v-model:structures="newProject.markets"
            />
        </card>

        <card-margin />

        <card title="Blacklist" v-if="projectGroupId">
            <p style="margin-left: 10px">
                Blacklist of items that should not be build.
                <br>
                <template v-if="projectGroupId !== '00000000-0000-0000-0000-000000000000'">
                    The default blacklist are set from the configured defaults of the selected
                    <reference
                        :params="{ groupId: projectGroupId }"
                        :route="routeProjectGroupDetail"
                    >
                        Project Group
                    </reference>
                </template>
                <template v-else>
                    Ypu can set the default values by creating a
                    <n-button
                        quaternary
                        type="primary"
                        @click="$router.push({ name: routeProjectGroup })"
                    >
                        Project Group
                    </n-button>
                </template>
            </p>

            <blacklist
                v-model:blacklist="newProject.blacklist"
            />
        </card>

        <card-margin />

        <card title="Stock">
            <p style="margin-left: 10px">
                Stock that you already have.
                Everything in stock will be prefered over buying or building it.
                If it's not required by the project, it will be ignored.
                <br>
                The format looks like:
                <code>ItemName Quantity</code>
            </p>

            <n-input
                v-model:value="stock"
                type="textarea"
                placeholder="Insert your Stock"
                rows="10"
            />
        </card>

        <card-margin />

        <card title="Products">
            <p style="margin-left: 10px; margin-right: 10px">
                There are two types of products, the ones you build and the ones that you can´t build, or can´t be bothered to build but still need.
                Put everything you want to build into the first text box.
                Everything else what you need for the project, for example faction modules when you build a fitting, or anything else that you need for the project, but either cannot build or cannot be bothered to build.

                <br><br>
                The format looks like this for things to build:
                <code>ItemName Runs MaterialEfficiency</code>

                <br>
                and the format is like this for additional products:
                <code>ItemName Quantity</code>
            </p>

            <div style="width: 100%; overflow: hidden">
                <div style="width: 49%; float: left;">
                    <product-selector
                        v-model:products="products"
                    />
                </div>

                <div style="width: 2%; height: 1px; float: left"></div>

                <div style="width: 49%; float: left;">
                    <additional-product-selector
                        v-model:products="additionalProducts"
                    />
                </div>
            </div>
        </card>

        <card-margin />

        <action-group>
            <n-button @click="$router.back()" quaternary>Cancel</n-button>

            <n-button
                @click="createProject"
                :disabled="!newProject.name || !products"
                type="info"
            >
                Create project
            </n-button>
        </action-group>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { ROUTE_PROJECT_GROUP, ROUTE_PROJECT_GROUPS } from '@/project_group/router';

import { ProjectService, type ICreateProject, type IProduct } from '@/sdk/project';
import { ItemService } from '@/services/item';
import { Structure } from '@/sdk/structure';
import { StructureGroupService } from '@/sdk/structure_group';
import { ProjectGroupService } from '@/project_group/service';

import {
    NButton, NCard, NForm, NFormItem, NInput, NInputNumber,
  NSpace, NSwitch, NText,
} from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import AdditionalProductSelector from '@/project/selectors/AdditionalProductSelector.vue';
import Blacklist from '@/project_group/components/DefaultBlacklist.vue';
import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import FormatNumberInput from '@/components/inputs/FormatNumber.vue';
import GeneralInfo from '@/project/components/GeneralInfo.vue';
import Market from '@/project_group/components/DefaultMarket.vue';
import PageHeader from '@/components/PageHeader.vue';
import ProductSelector from '@/project/selectors/ProductSelector.vue';
import Reference from '@/components/Reference.vue';
import StructureCard from '@/components/StructureCard.vue';
import StructureGroupSelector from '@/components/selectors/StructureGroupSelector.vue';

@Component({
    components: {
        NButton,
        NCard,
        NForm,
        NFormItem,
        NInput,
        NInputNumber,
        NSpace,
        NSwitch,
        NText,

        ActionGroup,
        AdditionalProductSelector,
        Blacklist,
        Card,
        CardMargin,
        FormatNumberInput,
        GeneralInfo,
        Market,
        PageHeader,
        ProductSelector,
        Reference,
        StructureCard,
        StructureGroupSelector,
    }
})
class CreateProjectV2 extends Vue {
    public products: string = '';
    public additionalProducts: string = '';
    public stock: string = '';

    public newProject: ICreateProject = <any>{
        blacklist: [],
        markets: [],
    };

    public projectGroupId: string | null = null;

    public structures: Structure[] = [];

    public routeProjectGroup = ROUTE_PROJECT_GROUPS;
    public routeProjectGroupDetail = ROUTE_PROJECT_GROUP;

    public async createProject() {
        this.newProject.stocks = await ItemService.parse(this.stock);
        this.newProject.products = (await ItemService.parse<IProduct>(this.products, true))
            .map((x) => {
                if (!x.material_efficiency) {
                    x.material_efficiency = 0;
                }
                return x;
            });
        this.newProject.additional_products = await ItemService.parse(this.additionalProducts);

        await ProjectService
            .create(this.newProject)
            .then(projectId => {
                this.$router.push({
                    name: 'project_overview',
                    params: {
                        projectId
                    }
                });
            })
            .catch(e => {
                console.error(e);
            });
    }

    public async selectProjectGroupId(value: string) {
        if (value === '00000000-0000-0000-0000-000000000000') {
            this.projectGroupId = '00000000-0000-0000-0000-000000000000';
            return;
        }

        await ProjectGroupService
            .fetch(value)
            .then(x => x.fetchDefault())
            .then(x => {
                if (x && x.markets) {
                    this.newProject.markets = x.markets;
                }
                if (x && x.blacklist) {
                    this.newProject.blacklist = x.blacklist
                }

                this.projectGroupId = value;
            })
            .catch(_ => {
            });
    }

    public async selectStructureGroup(value: string) {
        await StructureGroupService
            .fetch(value)
            .then(x => {
                this.structures = x.structures;
            })
            .catch(_ => {
            })
    }
}

export default toNative(CreateProjectV2);
</script>

