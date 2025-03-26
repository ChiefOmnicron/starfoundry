<template>
    <div>
        <common-messages
            :message="messages"
            @close="commonMessagesClose"
        />

        <card title="Market">
            <market
                v-model:structures="markets"
            />
        </card>

        <card-margin />

        <card title="Blacklist">
            <blacklist
                v-model:blacklist="blacklist"
            />
        </card>

        <card-margin />

        <action-group
            justify="end"
        >
            <n-button @click="save" type="info">
                Save
            </n-button>
        </action-group>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { TypeId, Uuid } from '@/sdk/utils';
import { ProjectGroupService } from '@/project_group/service';

import { NButton } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import Blacklist from '@/project_group/components/DefaultBlacklist.vue';
import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import CommonMessages, { DEFAULT_COMMON_MESSAGES, type ICommonMessages } from '@/components/CommonMessages.vue';
import Market from '@/project_group/components/DefaultMarket.vue';

@Component({
    components: {
        NButton,

        ActionGroup,
        Blacklist,
        Card,
        CardMargin,
        CommonMessages,
        Market,
    }
})
class ProjectGroupOverviewDefault extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public groupId!: Uuid;

    public messages: ICommonMessages = DEFAULT_COMMON_MESSAGES();

    public markets: Uuid[] = [];
    public blacklist: TypeId[] = [];
    public newMarketStructure: string = '';

    public async created() {
        ProjectGroupService
            .fetch(this.groupId)
            .then(x => x.fetchDefault())
            .then(x => {
                if (x) {
                    this.markets.push(...x.markets || []);
                    this.blacklist.push(...x.blacklist || []);
                } else {
                    this.markets.push('00000000-0000-0000-0000-000000000001');
                }
            })
            .catch(e => {
                if (e.response.status === 404) {
                    this.markets.push('00000000-0000-0000-0000-000000000001');
                } else {
                    this.messages.loadingError = true;
                }
            });
    }

    public async save() {
        ProjectGroupService
            .fetch(this.groupId)
            .then(x => x.updateDefault({
                blacklist: this.blacklist,
                markets: this.markets,
            }))
            .then(_ => {
                this.messages.updateSuccess = true;
            })
            .catch(_ => {
                this.messages.updateError = true;
            });
    }

    public commonMessagesClose() {
        this.messages = DEFAULT_COMMON_MESSAGES();
    }
}

export default toNative(ProjectGroupOverviewDefault);
</script>
