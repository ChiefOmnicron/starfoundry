<template>
    <character-info
        :character-id="parseInt(<any>$route.params.characterId, 10)"
        v-slot="{ info }"
    >
        <loader description="Loading character" :busy="busy" />

        <page-header :title="info.character" />

        <card title="ESI-Permissions">
            <template #action>
                <n-button @click="add_scope(info.character_id)" type="info">
                    Add scope
                </n-button>
            </template>

            <template #default>
                <n-list style="margin-left: 10px">
                    <n-list-item>
                        <div v-for="p in info.scopes" :key="p">
                            {{ p }}<br />
                        </div>
                    </n-list-item>
                </n-list>
            </template>
        </card>
    </character-info>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import {
    NButton,
    NCheckbox,
    NEmpty,
    NIcon,
    NList,
    NListItem,
    NTable,
    NSpace,
} from 'naive-ui';

import { type CharacterId } from '@/utils';
import { Service, type ICharacter } from '@/characters/service';
import { events } from '@/main';
import { ROUTE_CHANGE } from '@/event_bus';

import Card from '@/components/Card.vue';
import CharacterInfo from '@/characters/components/CharacterInfo.vue';
import EveIcon from '@/components/EveIcon.vue';
import Loader from '@/components/Loader.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NButton,
        NCheckbox,
        NEmpty,
        NIcon,
        NList,
        NListItem,
        NSpace,
        NTable,

        Card,
        CharacterInfo,
        EveIcon,
        Loader,
        PageHeader,
    },
})
class CharacterSettings extends Vue {
    public busy: boolean = false;
    public refresh_active: boolean = false;

    public characters: ICharacter[] = [];
    public tags = {};

    public async created() {
        events.$emit(ROUTE_CHANGE, this.$route.name);

        this.busy = true;
        await this.load();
        this.busy = false;
    }

    public async refresh() {
        this.refresh_active = true;
        await Service.refresh(<any>this.$route.params.characterId);
        this.refresh_active = false;
    }

    public add_scope(characterId: CharacterId) {
        window.location.href = `/api/v1/auth/scopes/${characterId}`;
    }

    private async load() {
        this.characters = [];

        let character = await Service.whoami();
        let character_alts = await Service.alts();

        character_alts.sort((a: ICharacter, b: ICharacter) => {
            return a.character_name.localeCompare(b.character_name);
        });

        this.characters.push(character);
        this.characters = this.characters.concat(character_alts);
    }
}

export default toNative(CharacterSettings);
</script>
