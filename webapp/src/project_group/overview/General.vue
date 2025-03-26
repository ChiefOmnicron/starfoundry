<template>
    <div>
        <common-messages
            :message="messages"
            @close="commonMessagesClose"
        />

        <card
            no-title
            v-if="!busy && !messages.hasError(messages) && group"
        >
            <n-form
                ref="form_ref"
                style="margin-top: 15px; margin-left: 10px; margin-right: 10px"
                :model="group"
                :rules="rules"
            >
                <n-form-item path="name" label="Name">
                    <n-input
                        v-model:value="group.name"
                        @input="(e: any) => group.name = e"
                        placeholder="Name"
                    />
                </n-form-item>

                <n-form-item path="description" label="Description">
                    <n-input
                        v-model:value="group.description"
                        @input="(e: any) => group.description = e"
                        placeholder="Description"
                        type="textarea"
                    />
                </n-form-item>
            </n-form>
        </card>

        <card-margin />

        <action-group
            justify="end"
            v-if="!busy && !messages.hasError(messages) && group"
        >
            <n-button @click="$router.back()" quaternary>Cancel</n-button>

            <n-button @click="update" :disabled="!group.name" type="info">
                Update
            </n-button>
        </action-group>

        <card-margin />

        <delete-object
            @delete="remove"
            object-description=".update.danger.description"
            object-title="project.group.update.danger.danger.title"
            v-if="!busy && !messages.notFound && group && group.isGroupOwner"
        />
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { ROUTE_PROJECT_GROUPS } from '@/project_group/router';

import { type Uuid } from '@/sdk/utils';
import { ProjectGroup, ProjectGroupService } from '@/project_group/service';

import { type FormRules, NButton, NForm, NFormItem, NInput, NSpace } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import CommonMessages, { DEFAULT_COMMON_MESSAGES, type ICommonMessages } from '@/components/CommonMessages.vue';
import DeleteObject from '@/components/DeleteObject.vue';

@Component({
    components: {
        NButton,
        NForm,
        NFormItem,
        NInput,
        NSpace,

        ActionGroup,
        Card,
        CardMargin,
        CommonMessages,
        DeleteObject,
    },
    emits: [
        'refresh',
    ]
})
class ProjectGroupOverviewGeneral extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public groupId!: Uuid;

    public group: ProjectGroup = <ProjectGroup>{};

    public busy: boolean = false;
    public messages: ICommonMessages = DEFAULT_COMMON_MESSAGES();

    public created() {
        this.load();
    }

    public async load() {
        this.busy = true;

        ProjectGroupService
            .fetch(this.groupId)
            .then(x => {
                this.busy = false;
                this.group = x;
            })
            .catch(e => {
                this.busy = false;
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                } else {
                    this.messages.loadingError = true;
                }
            });
    }

    public async update() {
        this.busy = true;

        await this.group
            .update()
            .then(_ => {
                this.busy = false;
                this.$emit('refresh', null);
            })
            .catch(e => {
                this.busy = false;
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                } else if(e.response.status === 403) {
                    this.messages.forbidden = true;
                } else {
                    this.messages.loadingError = true;
                }
            });
    }

    public remove() {
        this.busy = true;

        this.group
            .remove()
            .then(_ => {
                this.busy = false;
                this.$router.push({
                    name: ROUTE_PROJECT_GROUPS,
                });
            })
            .catch(e => {
                this.busy = false;
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                } else if(e.response.status === 403) {
                    this.messages.forbidden = true;
                } else {
                    this.messages.loadingError = true;
                }
            });
    }

    public rules: FormRules = {
        name: [{
            required: true,
            message: 'The field is required'
        }],
    }

    public commonMessagesClose() {
        this.messages = DEFAULT_COMMON_MESSAGES();
    }
}

export default toNative(ProjectGroupOverviewGeneral);
</script>
