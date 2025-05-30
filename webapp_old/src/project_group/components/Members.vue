<template>
    <n-table v-if="members.length > 0">
        <thead>
            <tr>
                <th width="300px">Name</th>
                <th width="500px">Projects</th>
                <th width="500px">Project Group</th>
                <th width="500px">Structures</th>
                <th></th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="member in members" :key="member.character_id">
                <td>
                    {{ member.character_name }}
                </td>
                <td v-if="member.accepted && !member.is_owner">
                    <n-switch
                        checked-value="WRITE"
                        unchecked-value="READ"
                        v-model:value="member.projects"
                        :rail-style="railStyle"
                    >
                        <template #checked> Write </template>
                        <template #unchecked> Read </template>
                    </n-switch>
                </td>
                <td v-if="member.accepted && !member.is_owner">
                    <n-switch
                        checked-value="WRITE"
                        unchecked-value="READ"
                        v-model:value="member.project_group"
                        :rail-style="railStyle"
                    >
                        <template #checked> Write </template>
                        <template #unchecked> Read </template>
                    </n-switch>
                </td>
                <td v-if="member.accepted && !member.is_owner">
                    <n-switch
                        checked-value="WRITE"
                        unchecked-value="READ"
                        v-model:value="member.structures"
                        :rail-style="railStyle"
                    >
                        <template #checked> Write </template>
                        <template #unchecked> Read </template>
                    </n-switch>
                </td>
                <td v-if="member.accepted && !member.is_owner">
                    <n-button-group v-if="!member.is_owner" align="right">
                        <n-button @click="savePermission(member)">
                            Save
                        </n-button>
                        <n-button @click="removeMember(member.character_id)">
                            Remove
                        </n-button>
                    </n-button-group>
                </td>
                <td v-if="member.is_owner" colspan="4">
                    Owners cannot be modified
                </td>
                <td v-if="!member.accepted" colspan="3">
                    Member not yet accepted
                </td>
                <td v-if="!member.accepted" align="right">
                    <n-button
                        @click="acceptMember(member.character_id)"
                        type="primary"
                        ghost
                    >
                        Accept
                    </n-button>
                </td>
            </tr>
        </tbody>
    </n-table>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import type { CSSProperties } from 'vue';

import type { CharacterId } from '@/sdk/utils';
import type { IProjectGroupMember } from '@/project_group/service';

import { NButton, NButtonGroup, NCheckbox, NTable, NSwitch } from 'naive-ui';

@Component({
    components: {
        NButton,
        NButtonGroup,
        NCheckbox,
        NSwitch,
        NTable,
    },
    emits: ['member-accepted', 'save-permission'],
})
class ProjectGroupMembers extends Vue {
    @Prop({
        default: [],
        required: true,
        type: Array<IProjectGroupMember>,
    })
    public members!: IProjectGroupMember[];

    public async acceptMember(characterId: CharacterId) {
        this.$emit('member-accepted', characterId);
    }

    public async removeMember(characterId: CharacterId) {
        this.$emit('member-removed', characterId);
    }

    public async savePermission(member: IProjectGroupMember) {
        this.$emit('save-permission', member);
    }

    public railStyle({
        focused,
        checked,
    }: {
        focused: boolean;
        checked: boolean;
    }) {
        const style: CSSProperties = {};

        if (checked) {
            style.background = '#2A947D';

            if (focused) {
                style.boxShadow = '0 0 0 2px #d0305040';
            }
        } else {
            style.background = '#2080f0';

            if (focused) {
                style.boxShadow = '0 0 0 2px #2080f040';
            }
        }
        return style;
    }
}

export default toNative(ProjectGroupMembers);
</script>
