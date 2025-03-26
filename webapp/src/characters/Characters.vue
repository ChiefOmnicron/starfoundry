<template>
    <div>
        <page-header title="Characters" />

        <loader description="Loading characters" :busy=busy />

        <card content-style="padding: 0" v-if="!busy">
            <template #action>
                <n-button @click="refresh" :disabled="!selected" :loading="refresh_active">
                    Refresh info
                </n-button>
                <n-button @click="remove" :disabled="!selected">
                    Remove
                </n-button>

                <n-button @click="add_corproation" type="info" ghost>
                    Add corporation
                </n-button>

                <n-button @click="add_character" type="info">
                    Add character
                </n-button>
            </template>

            <n-table v-if="!busy">
                <thead>
                    <tr>
                        <th width="34px"></th>
                        <th width="40px"></th>
                        <th>Name</th>
                        <th>Type</th>
                        <th width="40px"></th>
                        <th>Corporation</th>
                        <th width="40px"></th>
                        <th>Alliance</th>
                    </tr>
                </thead>
                <tbody>
                    <template v-for="character in characters" :key="character.character_id">
                        <tr>
                            <td>
                                <n-checkbox :checked="selected" :checked-value="character.character_id"
                                    @update:checked="handle_select" unchecked-value="undefined" name="selected">
                                </n-checkbox>
                            </td>
                            <td>
                                <eve-icon :id="character.character_id" character  v-if="character.credential_type === 'CHARACTER'" />
                            </td>
                            <td>
                                <n-button text type="info" v-if="character.credential_type === 'CHARACTER'">
                                    <router-link :to="{
                                            name: route_character,
                                            params: { characterId: character.character_id }
                                        }
                                        " style="color: inherit; text-decoration: none">
                                        {{ character.character_name }}
                                    </router-link>
                                </n-button>
                            </td>
                            <td>
                                <p v-if="character.credential_type === 'CORPORATION'">
                                    Corporation
                                </p>
                                <p v-else>
                                    Character
                                </p>
                            </td>
                            <td>
                                <eve-icon :id="character.corporation_id" corporation />
                            </td>
                            <td>
                                {{ character.corporation_name }}
                            </td>
                            <td v-if="character.alliance_id">
                                <alliance :id="character.alliance_id" />
                            </td>
                            <td v-if="!character.alliance_id"></td>
                            <td v-if="character.alliance_id">
                                {{ character.alliance_name }}
                            </td>
                            <td v-if="!character.alliance_id"></td>
                        </tr>
                    </template>
                </tbody>
            </n-table>

            <confirm-dialog v-if="selected" v-model:show="show_confirm" :confirm="confirm_remove">
                Are you sure you want to delete {{ characters.find(x => x.character_id === parseInt(selected)).character
                }}?<br>
                You can add the character back to a later time.<br>
                Please type in 'delete' to confirm.
            </confirm-dialog>
        </card>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NCheckbox, NIcon, NList, NListItem, NTable, NSpace, useMessage } from 'naive-ui';

import { type CharacterId } from '@/utils';
import { Service, type ICharacter } from '@/characters/service';
import { events } from '@/main';
import { ROUTE_CHANGE } from '@/event_bus';
import { ROUTE_CHARACTER } from './router';

import Alliance from '@/characters/components/Alliance.vue';
import Card from '@/components/Card.vue';
import EveIcon from '@/components/EveIcon.vue';
import ConfirmDialog from '@/components/ConfirmDialog.vue';
import Loader from '@/components/Loader.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NButton,
        NCheckbox,
        NIcon,
        NList,
        NListItem,
        NSpace,
        NTable,

        Alliance,
        Card,
        EveIcon,
        ConfirmDialog,
        Loader,
        PageHeader,
    }
})
class CharacterSettings extends Vue {
    public busy: boolean = false;
    public refresh_active: boolean = false;

    public characters: ICharacter[] = [];
    public selected: number = 0;
    public show_confirm: boolean = false;

    public route_character: string = ROUTE_CHARACTER;

    public message = useMessage();

    public async created() {
        events.$emit(
            ROUTE_CHANGE,
            this.$route.name
        );

        this.busy = true;
        await this.load();
        this.busy = false;
    }

    public add_character() {
        Service.add_character();
    }

    public add_corproation() {
        Service.add_corporation();
    }

    public async refresh() {
        if (!this.selected) { return; }

        this.refresh_active = true;

        await Service.refresh(this.selected);
        this.selected = 0;

        this.refresh_active = false;
    }

    public remove() {
        if (!this.selected) { return; }

        if (this.selected === (<any>window).whoami.character_id) {
            this.message.warning('You cannot delete the main character.');
            return;
        }

        this.show_confirm = true;
    }

    public async confirm_remove() {
        if (!this.selected) { return; }

        Service.remove(this.selected);
        this.selected = 0;
        this.show_confirm = false;

        this.busy = true;
        await this.load();
        this.busy = false;
    }

    public handle_select(characterId: number | string) {
        if (characterId === 'undefined') {
            this.selected = 0;
            return;
        }
        this.selected = <number>characterId;
    }

    public refresh_permissions(
        characterId: CharacterId,
    ) {
        window.location.href = `/api/v1/auth/refresh/${characterId}`;
    }

    private async load() {
        this.characters = [];

        let character = await Service.whoami();
        let character_alts = await Service.alts();
        let corporations = await Service.alt_corporations();

        character_alts.sort((a: ICharacter, b: ICharacter) => {
            return a.character_name.localeCompare(b.character_name);
        });

        this.characters.push(character);
        this.characters = this.characters.concat(character_alts);
        this.characters = this.characters.concat(corporations);
    }
}

export default toNative(CharacterSettings);
</script>
