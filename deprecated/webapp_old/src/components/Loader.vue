<template>
    <n-empty
        :description="description"
        size="large"
        style="margin-top: 5%"
        v-if="busy"
    >
        <template #icon>
            <n-spin size="large" />
        </template>
    </n-empty>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NEmpty, NSpin } from 'naive-ui';

@Component({
    components: {
        NEmpty,
        NSpin,
    },
})
class Loader extends Vue {
    @Prop({
        type: Boolean,
        required: true,
    })
    public busy!: boolean;

    @Prop({
        type: String,
        default: 'Loading',
    })
    public description!: string;
}

export default toNative(Loader);
</script>
