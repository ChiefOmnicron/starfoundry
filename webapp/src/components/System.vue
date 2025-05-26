<template>
    <n-text v-if="!dotlan">
        {{ name || systemId }}
    </n-text>

    <n-button
        :href="link"
        tag="a"
        target="_blank"
        text
        type="primary"
        v-if="dotlan"
    >
        {{ name || systemId }}
    </n-button>
</template>

<script lang="ts">
import { h } from 'vue';
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NButton, NText } from 'naive-ui';

import { type ISystem, SystemService } from '@/services/system';
import type { SystemId } from '@/utils';

@Component({
    components: {
        NButton,
        NText,
    },
})
class System extends Vue {
    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public dotlan!: boolean;

    @Prop({
        type: Number,
        required: true,
    })
    public systemId!: number;

    public name: string = '';
    public link: string = '';

    public async created() {
        SystemService.search_by_id(this.systemId).then((x: ISystem[]) => {
            if (x.length === 0) {
                this.name = `${this.systemId}`;
                return;
            }

            this.name = `${x[0].system_name} (${x[0].region_name})`;

            let region_name = x[0].region_name.replace(' ', '_');
            this.link = `https://evemaps.dotlan.net/map/${region_name}/${x[0].system_name}`;
        });
    }
}

export default toNative(System);

export const renderFunction = (
    systemId: SystemId,
    dotlan:   boolean,
) => {
    return h(
        System,
        {
            systemId,
            dotlan,
            // otherwise the component may not re-render when it's changed with
            // filters
            key: systemId,
        }
    )
}
</script>
