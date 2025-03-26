<template>
    <n-input-group>
        <n-input
            placeholder="Structure ID"
            :status="structureIdStatus"
            @keydown.enter.prevent
            @update:value="structureIdChange"
            data-cy="StructureResolveInput"
            style="width: 100%;"
            v-model:value="structureString"
        />
        <n-button
            :loading="busy"
            @click="resolveStructure"
            data-cy="StructureResolveButton"
            ghost
            type="primary"
        >
            Resolve Structure
        </n-button>
    </n-input-group>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';

import { StructureService } from '@/sdk/structure';

import { NButton, NInput, NInputGroup } from 'naive-ui';

@Component({
    components: {
        NButton,
        NInput,
        NInputGroup,
    },
    emits: [
        'error',
        'resolved',
    ]
})
class ResolveStructure extends Vue {
    public busy: boolean = false;

    public structureString: string = '';
    // form validation
    public structureIdStatus: string = '';

    public structureIdChange(value: string) {
        if (!value || value === '') {
            this.structureIdStatus = 'error'
        } else {
            this.structureIdStatus = ''
        }
    }

    public async resolveStructure() {
        let structureId!: number;

        if (this.structureString.indexOf('url=showinfo') >= 0) {
            let regex = /<url=showinfo:[0-9]*\/\/([0-9]*)>/;
            let match = this.structureString.match(regex) || [];
            structureId = parseInt(match[1]);
        } else {
            structureId = parseInt(this.structureString);
        }

        this.busy = true;
        await StructureService
            .resolve(structureId)
            .then(x => {
                this.busy = false;
                this.$emit('resolved', x);
            })
            .catch(_ => {
                this.busy = false;
                this.$emit('error', true)
            });
    }
}

export default toNative(ResolveStructure);
</script>
