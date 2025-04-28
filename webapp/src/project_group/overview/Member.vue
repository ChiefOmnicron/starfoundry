<template>
    <div>
        <common-messages :message="messages" @close="commonMessagesClose" />

        <card no-title>
            <n-form
                style="margin-top: 15px; margin-left: 10px; margin-right: 10px"
            >
                <n-form-item label="Invite link">
                    <n-input-group>
                        <n-input
                            v-model:value="inviteLink"
                            disabled
                            @keydown.enter.prevent
                            style="width: 100%"
                        />
                        <n-button type="primary" ghost @click="copyInviteLink">
                            Copy
                        </n-button>
                    </n-input-group>
                </n-form-item>
            </n-form>

            <group-members
                :members="members"
                @member-accepted="memberAccepted"
                @member-removed="memberRemoved"
                @save-permission="savePermission"
            />
        </card>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import {
    type IProjectGroupMember,
    ProjectGroupMemberService,
} from '@/project_group/service';
import type { CharacterId, Uuid } from '@/sdk/utils';

import { NButton, NForm, NFormItem, NInput, NInputGroup } from 'naive-ui';
import Card from '@/components/Card.vue';
import GroupMembers from '@/project_group/components/Members.vue';
import CommonMessages, {
    DEFAULT_COMMON_MESSAGES,
    type ICommonMessages,
} from '@/components/CommonMessages.vue';

@Component({
    components: {
        NButton,
        NForm,
        NFormItem,
        NInput,
        NInputGroup,

        Card,
        CommonMessages,
        GroupMembers,
    },
    emits: ['refresh'],
})
class ProjectGroupOverviewGeneral extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public groupId!: Uuid;

    public members: IProjectGroupMember[] = [];
    public inviteLink: string = `${window.location}/invite`;

    public messages: ICommonMessages = DEFAULT_COMMON_MESSAGES();

    public async created() {
        this.loadMembers();
    }

    public copyInviteLink() {
        navigator.clipboard.writeText(<string>this.inviteLink);
    }

    public loadMembers() {
        ProjectGroupMemberService.members(this.groupId)
            .then((x) => {
                this.members = x;
            })
            .catch((e) => {
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                } else {
                    this.messages.loadingError = true;
                }
            });
    }

    public memberAccepted(characterId: CharacterId) {
        ProjectGroupMemberService.accept(this.groupId, characterId)
            .then((_) => {
                this.$emit('refresh', characterId);
            })
            .catch((e) => {
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                } else {
                    this.messages.loadingError = true;
                }
            });
    }

    public memberRemoved(characterId: CharacterId) {
        ProjectGroupMemberService.remove(this.groupId, characterId)
            .then((_) => {
                this.$emit('refresh', characterId);

                this.members = this.members.filter(
                    (x) => x.character_id !== characterId,
                );
            })
            .catch((e) => {
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                } else {
                    this.messages.loadingError = true;
                }
            });
    }

    public savePermission(member: IProjectGroupMember) {
        ProjectGroupMemberService.update(this.groupId, member.character_id, {
            projects: member.projects,
            project_group: member.project_group,
            structures: member.structures,
        })
            .then((_) => {
                this.$emit('refresh', null);
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

export default toNative(ProjectGroupOverviewGeneral);
</script>
