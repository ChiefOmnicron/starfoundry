<template>
    <n-result
        :description="`You were invited to the project '${group.name}' by '${group.owner_name}'`"
        status="info"
        style="margin-top: 50px"
        title="Project invite"
    >
        <template #footer>
            <n-button type="success" @click="accept_invite">
                Accept invite
            </n-button>
        </template>
    </n-result>
</template>

<script lang="ts">
import { ProjectGroupService, ProjectGroup } from '@/project_group/service';
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { Uuid } from '@/sdk/utils';

import { NButton, NResult } from 'naive-ui';
import { ROUTE_PROJECT_GROUPS } from '@/project_group/router';

@Component({
    components: {
        NButton,
        NResult,
    },
})
class ProjectGroupInvite extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public groupId!: Uuid;

    public group: ProjectGroup = <ProjectGroup>{};

    public async created() {
        ProjectGroupService.fetchInvite(this.groupId)
            .then((x) => (this.group = x))
            .catch((_) => {});
    }

    public accept_invite() {
        ProjectGroupService.acceptInvite(<string>this.$route.params.group_id)
            .then((_) => {
                this.$router.push({
                    name: ROUTE_PROJECT_GROUPS,
                });
            })
            .catch((e) => {
                console.error(e);
            });
    }
}

export default toNative(ProjectGroupInvite);
</script>
