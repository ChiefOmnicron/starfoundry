<template>
    <div>
        <page-header
            :title="project.name + ' ' + 'Market'"
            v-if="!initialLoad && !messages.hasError(messages) && project"
        />

        <common-messages :message="messages" @close="commonMessagesClose" />

        <n-tabs type="line">
            <n-tab-pane name="overview" tab="Overview">
                <market-overview :project-uuid="projectId" />
            </n-tab-pane>
            <n-tab-pane name="recommendation_general" tab="Buy Recommendation">
                <market-recommendation />
            </n-tab-pane>
            <n-tab-pane name="replace" tab="Compressed Minerals">
                <compressed-minerals />
            </n-tab-pane>
            <!--n-tab-pane name="recommendation_minerals" tab="Recommendation Minerals">
                <market-recommendation-minerals />
            </n-tab-pane>
            <n-tab-pane name="recommendation_gas" tab="Recommendation Gas">
                <market-recommendation-gas />
            </n-tab-pane-->
        </n-tabs>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NInput, NTabPane, NTabs } from 'naive-ui';

import { type Uuid } from '@/sdk/utils';
import { ProjectService, type Project } from '@/sdk/project';

import CommonMessages, {
    DEFAULT_COMMON_MESSAGES,
    type ICommonMessages,
} from '@/components/CommonMessages.vue';
import CompressedMinerals from '@/project/market/CompressedMinerals.vue';
import MarketOverview from '@/project/market/Overview.vue';
import MarketRecommendation from '@/project/market/Recommendation.vue';
import MarketRecommendationGas from '@/project/market/RecommendationGas.vue';
import MarketRecommendationMinerals from '@/project/market/RecommendationMinerals.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NButton,
        NInput,
        NTabPane,
        NTabs,

        CommonMessages,
        CompressedMinerals,
        MarketOverview,
        MarketRecommendation,
        MarketRecommendationGas,
        MarketRecommendationMinerals,
        PageHeader,
    },
})
class ProjectsMarket extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public projectId!: Uuid;

    public initialLoad: boolean = false;
    public messages: ICommonMessages = DEFAULT_COMMON_MESSAGES();

    public project!: Project;

    public async created() {
        this.initialLoad = true;

        await ProjectService.fetch(this.projectId)
            .then((x) => {
                this.project = x;
                this.initialLoad = false;
            })
            .catch((e) => {
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                } else if (e.response.status === 403) {
                    this.messages.forbidden = true;
                } else {
                    this.messages.loadingError = true;
                }

                this.initialLoad = false;
            });

        this.initialLoad = false;
    }

    public commonMessagesClose() {
        this.messages = DEFAULT_COMMON_MESSAGES();
    }
}

export default toNative(ProjectsMarket);
</script>
