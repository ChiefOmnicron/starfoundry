<template>
    <card
        title="structures.modals.resolveStructureInfo.title"
        style="width: 1000px"
    >
        <div style="margin: 5px; padding-bottom: 25px">
            <h3>Strucute Info</h3>
            <n-image src="/static/add_structure/structure_info.png" />

            <h3>Drag Structure into chat</h3>
            <n-image src="/static/add_structure/drag_structure.png" />

            <h3>Copy chat message</h3>
            <n-image src="/static/add_structure/copy.png" />

            <h3>Insert into editor</h3>
            <n-image src="/static/add_structure/insert.png" width="985px" />
        </div>
    </card>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';

import { NImage } from 'naive-ui';
import Card from '@/components/Card.vue';

@Component({
    components: {
        NImage,

        Card,
    },
})
class ResolveStructureIdInfo extends Vue {}

export default toNative(ResolveStructureIdInfo);
</script>
