<template>
    <div>
        <page-header title="Project Groups" />

        <common-messages
            :message="messages"
            @close="commonMessagesClose"
        />

        <loader description="Loading Groups" :busy="busy" />

        <action-group>
            <n-button @click="$router.push({ name: ROUTE_PROJECT_GROUP_CREATE })" type="info">
                New group
            </n-button>
        </action-group>

        <card no-title content-style="padding: 0" v-if="!busy">
            <n-table v-if="!busy && groups.length > 0">
                <thead>
                    <tr>
                        <th width="300px">Name</th>
                        <th width="750px">Description</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="group in groups" :key="group.id">
                        <td v-if="group.id !== '00000000-0000-0000-0000-000000000000'">
                            <n-button text type="info">
                                <router-link :to="{
                                        name: ROUTE_PROJECT_GROUP,
                                        params: { groupId: group.id }
                                    }
                                    "
                                    style="color: inherit;
                                    text-decoration: none"
                                >
                                    {{ group.name }}
                                </router-link>
                            </n-button>
                        </td>
                        <td v-if="group.id === '00000000-0000-0000-0000-000000000000'">
                            {{ group.name }}
                        </td>
                        <td>
                            {{ group.description }}
                        </td>
                    </tr>
                </tbody>
            </n-table>

            <no-entries
                description="No groups yet"
                size="large"
                style="margin: 5%"
                v-if="!busy && groups.length === 0"
            />
        </card>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';

import { ROUTE_PROJECT_GROUP, ROUTE_PROJECT_GROUP_CREATE } from '@/project_group/router';
import { ProjectGroup, ProjectGroupService } from '@/project_group/service';

import { NButton, NTable } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import Card from '@/components/Card.vue';
import CommonMessages, { DEFAULT_COMMON_MESSAGES, type ICommonMessages } from '@/components/CommonMessages.vue';
import Loader from '@/components/Loader.vue';
import NoEntries from '@/components/NoEntries.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NButton,
        NTable,

        ActionGroup,
        Card,
        CommonMessages,
        Loader,
        NoEntries,
        PageHeader,
    }
})
class ProjectGroupList extends Vue {
    public ROUTE_PROJECT_GROUP = ROUTE_PROJECT_GROUP;
    public ROUTE_PROJECT_GROUP_CREATE = ROUTE_PROJECT_GROUP_CREATE;

    public busy: boolean = false;

    public groups: ProjectGroup[] = [];
    public messages: ICommonMessages = DEFAULT_COMMON_MESSAGES();

    public async created() {
        this.busy = true;

        this.groups
            .push(<any>{
                id: '00000000-0000-0000-0000-000000000000',
                name: 'Default',
                description: 'The default group. It cannot be edited.'
            });

        ProjectGroupService
            .list({})
            .then(entries => {
                this.groups.push(...entries);
                this.busy = false;
            })
            .catch(e => {
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                } else {
                    this.messages.loadingError = true;
                }

                this.busy = false;
            });
    }

    public commonMessagesClose() {
        this.messages = DEFAULT_COMMON_MESSAGES();
    }
}

export default toNative(ProjectGroupList);
</script>
