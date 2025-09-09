<template>
    <n-space justify="end" style="margin-bottom: 10px">
        <div>
            <slot />
        </div>
    </n-space>
</template>

<script lang="ts">
// Aligns the given elements to the right of the screen.
//
// Example:
// <action-group>
//     <n-button>Action 1</n-button>
//     <n-button>Action 2</n-button>
// </action-group>
import { NSpace } from 'naive-ui';
import { Component, Vue, toNative } from 'vue-facing-decorator';

@Component({
    components: {
        NSpace,
    },
})
class ActionGroup extends Vue {}

export default toNative(ActionGroup);
</script>
