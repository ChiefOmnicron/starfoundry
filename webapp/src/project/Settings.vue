<template>
    <div>
        <page-header
            :title="project.name + ' ' + 'Settings'"
            v-if="!initialLoad && !messages.hasError(messages) && project"
        />

        <common-messages
            :message="messages"
            @close="commonMessagesClose"
        />

        <card
            title="General"
            v-if="!initialLoad && !messages.hasError(messages) && project"
        >
            <general-info
                v-model:data="project.info"
                v-if="project && project.info"
                :info="project.info"
                :readonly="!project.canWrite"
            />
        </card>

        <card-margin />

        <action-group
            v-if="!initialLoad && !messages.hasError(messages) && project.canWrite"
        >
            <n-button
                @click="$router.back()"
                quaternary
            >
                Cancel
            </n-button>

            <n-button
                @click="save"
                type="info"
            >
                Save
            </n-button>
        </action-group>

        <card-margin />

        <delete-object
            @delete="deleteObject"
            data-cy="structureUpdateDeleteObject"
            object-description="Deleting the project is not reversable."
            object-title="Delete Project"
            v-if="!initialLoad && !messages.hasError(messages) && project.isOwner"
        />
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative, Prop } from 'vue-facing-decorator';
import { NButton, NSpace, NTable } from 'naive-ui';
import { ROUTE_PROJECTS } from '@/project/router';

import { Project, ProjectService } from '@/sdk/project';
import { type ProjectUuid } from '@/sdk/utils';

import ActionGroup from '@/components/ActionGroup.vue';
import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import CommonMessages, { DEFAULT_COMMON_MESSAGES, type ICommonMessages } from '@/components/CommonMessages.vue';
import DeleteObject from '@/components/DeleteObject.vue';
import GeneralInfo from '@/project/components/GeneralInfo.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NButton,
        NSpace,
        NTable,

        ActionGroup,
        Card,
        CardMargin,
        CommonMessages,
        DeleteObject,
        GeneralInfo,
        PageHeader,
    }
})
class ProjectSettings extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public projectId!: ProjectUuid;

    public busy: boolean = false;
    public initialLoad: boolean = false;
    public messages: ICommonMessages = DEFAULT_COMMON_MESSAGES();

    public project!: Project;

    public async created() {
        this.initialLoad = true;

        await ProjectService
            .fetch(this.projectId)
            .then(x => {
                this.project = x;
                return this.project.fetchPermissionIsOwner();
            })
            .then(_ => this.project.fetchPermissionCanWrite())
            .then(_ => {
                this.initialLoad = false;
            })
            .catch(e => {
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                } else if(e.response.status === 403) {
                    this.messages.forbidden = true;
                } else {
                    this.messages.loadingError = true;
                }

                this.initialLoad = false;
            });

        this.initialLoad = false;
    }

    public async save() {
        this.busy = true;
        await this.project
            .saveSettings()
            .then(_ => {
                this.busy = false;
                this.messages.updateSuccess = true;
            })
            .catch(e => {
                if(e.response.status === 403) {
                    this.messages.forbidden = true;
                } else {
                    this.messages.loadingError = true;
                }

                this.busy = false;
            });
    }

    public async deleteObject() {
        this.busy = true;

        await this.project
            .remove()
            .then(_ => {
                this.$router.push({
                    name: ROUTE_PROJECTS,
                });
            })
            .catch(e => {
                if(e.response.status === 403) {
                    this.messages.forbidden = true;
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

export default toNative(ProjectSettings);
</script>
