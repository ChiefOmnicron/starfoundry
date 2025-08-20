<template>
    <span
        style="
            margin: 0px;
            margin-right: 10px;
            display: inline;
            cursor: pointer;
        "
        :style="{ color: colored ? '#70c0e8' : undefined }"
        @click="open"
    >
        <slot />

        &nbsp;
        <n-icon size="12">
            <external-link-alt />
        </n-icon>
    </span>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NIcon } from 'naive-ui';

import { ExternalLinkAlt } from '@vicons/fa';
import Project from '@/components/Project.vue';

@Component({
    components: {
        NButton,
        NIcon,

        ExternalLinkAlt,

        Project,
    },
})
class Reference extends Vue {
    @Prop({
        type: Boolean,
        required: false,
        default: true,
    })
    public newTab!: boolean;

    // route that should be linked to
    @Prop({
        type: String,
        required: true,
    })
    public route!: string;

    // route that should be linked to
    @Prop({
        type: Object,
        required: false,
    })
    public params!: Object;

    @Prop({
        type: Boolean,
        required: false,
        default: true,
    })
    public colored!: boolean;

    public open() {
        let route = this.$router.resolve({
            name: this.route,
            params: <any>this.params,
        });
        window.open(route.href);
    }
}

export default toNative(Reference);
</script>
