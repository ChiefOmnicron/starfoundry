<template>
    <span>
        <n-spin v-if="busy" />

        <slot v-if="!busy" :info="character"></slot>
    </span>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NSpin } from 'naive-ui';

import { Service, type ICharacter } from '@/characters/service';

@Component({
    components: {
        NSpin,
    },
})
class CharacterInfo extends Vue {
    @Prop({
        required: true,
    })
    public characterId!: number;

    public busy: boolean = false;
    public character: ICharacter = <any>{};

    public async created() {
        this.busy = true;

        if (!this.characterId) {
            return;
        }

        this.character = await Service.info(this.characterId).finally(
            () => (this.busy = false),
        );
    }
}

export default toNative(CharacterInfo);
</script>
