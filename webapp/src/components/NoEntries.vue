<template>
    <n-empty :description="description" size="large" style="margin-top: 5%">
        <template #extra>
            <slot></slot>
        </template>
    </n-empty>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NEmpty } from 'naive-ui';

@Component({
    components: {
        NEmpty,
    },
})
class NoEntries extends Vue {
    @Prop({
        required: false,
        type: String,
    })
    public description!: string;
}

export default toNative(NoEntries);
</script>
