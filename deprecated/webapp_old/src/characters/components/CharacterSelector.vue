<template>
    <n-select
        v-model:value="value"
        :options="options"
        :render-label="render_entry"
        :render-tag="render_selected"
        filterable
    />
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { NSelect } from 'naive-ui';

import { Service } from '@/characters/service';
import { h } from 'vue';

import EveIcon from '@/components/EveIcon.vue';

@Component({
    components: {
        NSelect,

        EveIcon,
    },
})
class CharacterSelector extends Vue {
    public options: any[] = [];

    public value: number | null = null;

    public async created() {
        let character = (<any>window).whoami;
        this.options.push({
            label: character.character,
            value: character.character_id,
        });

        (await Service.alts()).forEach((x) => {
            this.options.push({
                label: x.character_name,
                value: x.character_id,
            });
        });
    }

    public render_selected({ option }: any) {
        return this.render_entry(option);
    }

    public render_entry(option: any) {
        return h(
            'div',
            {
                style: {
                    display: 'flex',
                    alignItems: 'center',
                },
            },
            [
                h(EveIcon, {
                    id: option.value,
                    character: true,
                    style: {
                        marginTop: '2px',
                        marginRight: '10px',
                    },
                }),
                option.label,
            ],
        );
    }
}

export default toNative(CharacterSelector);
</script>
