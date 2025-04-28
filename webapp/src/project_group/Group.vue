<template>
    <div>
        <page-header :title="group.name" v-if="group && group.name" />

        <common-messages :message="messages" @close="commonMessagesClose" />

        <n-tabs type="line">
            <n-tab-pane name="General">
                <group-general :group-id="groupId" />
            </n-tab-pane>
            <n-tab-pane name="Members">
                <group-member :group-id="groupId" />
            </n-tab-pane>
            <n-tab-pane name="Defaults">
                <group-default :group-id="groupId" />
            </n-tab-pane>
        </n-tabs>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { ProjectGroupService, ProjectGroup } from '@/project_group/service';
import type { Uuid } from '@/sdk/utils';

import { NTabs, NTabPane } from 'naive-ui';
import CommonMessages, {
    DEFAULT_COMMON_MESSAGES,
    type ICommonMessages,
} from '@/components/CommonMessages.vue';
import GroupDefault from '@/project_group/overview/Default.vue';
import GroupGeneral from '@/project_group/overview/General.vue';
import GroupMember from '@/project_group/overview/Member.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NTabs,
        NTabPane,

        CommonMessages,
        GroupDefault,
        GroupGeneral,
        GroupMember,
        PageHeader,
    },
})
class ProjectGroupView extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public groupId!: Uuid;

    public group: ProjectGroup = <ProjectGroup>{};
    public messages: ICommonMessages = DEFAULT_COMMON_MESSAGES();

    public async created() {
        ProjectGroupService.fetch(this.groupId)
            .then((x) => {
                this.group = x;
            })
            .catch((e) => {
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                } else {
                    this.messages.loadingError = true;
                }
            });
    }

    public commonMessagesClose() {
        this.messages = DEFAULT_COMMON_MESSAGES();
    }
}

export default toNative(ProjectGroupView);
</script>
