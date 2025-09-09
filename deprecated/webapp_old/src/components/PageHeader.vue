<template>
    <n-page-header>
        <template #title>
            <h1>
                {{ title }}
            </h1>
        </template>
    </n-page-header>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NPageHeader } from 'naive-ui';

// Example:
//
// <title title="Title" />
//
@Component({
    components: {
        NPageHeader,
    },
})
class Title extends Vue {
    // translation key that should be used as title
    @Prop({
        type: String,
        required: true,
    })
    public title!: string;
}

export default toNative(Title);
</script>
