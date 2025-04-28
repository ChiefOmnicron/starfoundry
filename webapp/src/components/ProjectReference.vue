<template>
    <project :project-uuid="projectId" v-slot="{ project }">
        <h2
            v-if="header"
            style="
                margin: 0px;
                margin-right: 10px;
                display: inline;
                cursor: pointer;
            "
            @click="openProject"
        >
            {{ project.name }}

            <n-icon size="12">
                <external-link-alt />
            </n-icon>
        </h2>
        <template v-else>
            <n-button
                @click="openProject"
                icon-placement="right"
                quaternary
                type="info"
            >
                <template #icon>
                    <n-icon size="10"><external-link-alt /></n-icon>
                </template>

                <slot>
                    {{ project.name }}
                </slot>
            </n-button>
        </template>
    </project>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NIcon } from 'naive-ui';

import { ROUTE_PROJECT_OVERVIEW } from '@/project/router';
import type { Uuid } from '@/sdk/utils';

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
class ProjectReference extends Vue {
    @Prop({
        type: String,
        required: true,
    })
    public projectId!: Uuid;

    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public header!: boolean;

    public openProject() {
        let route = this.$router.resolve({
            name: ROUTE_PROJECT_OVERVIEW,
            params: {
                projectId: this.projectId,
            },
        });
        window.open(route.href);
    }
}

export default toNative(ProjectReference);
</script>
