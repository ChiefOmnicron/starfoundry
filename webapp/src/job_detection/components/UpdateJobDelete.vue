<template>
    <div>
        <n-table>
            <tr>
                <th  style="width: 100px">
                    Old Project
                </th>

                <td style="padding: 10px">
                    <n-switch v-model:value="deleteFromSource">
                        <template #checked>
                            Delete job from old Project
                        </template>
                        <template #unchecked>
                            Reset job in old Project
                        </template>
                    </n-switch>
                </td>
            </tr>
            <tr>
                <th>
                    Ignore
                </th>

                <td style="padding: 10px">
                    <n-switch v-model:value="ignore">
                        <template #checked>
                            Ignore for future job detections
                        </template>
                        <template #unchecked>
                            Allow job detection
                        </template>
                    </n-switch>
                </td>
            </tr>
        </n-table>

        <card-margin />

        <action-group>
            <n-button
                @click="add"
                type="info"
            >
                Delete
            </n-button>
        </action-group>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { ProjectUuid, StructureId, TypeId } from '@/sdk/utils';
import { JobDetectionService } from '@/job_detection/service';

import { NButton, NSwitch, NTable } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import CardMargin from '@/components/CardMargin.vue';

@Component({
    components: {
        NButton,
        NSwitch,
        NTable,

        ActionGroup,
        CardMargin,
    },
    emits: [
        'update:projectId',
    ],
})
class UpdateJobDelete extends Vue {
    @Prop({
        type: Number,
        required: true,
    })
    public jobId!: TypeId;

    public ignore: boolean = false;
    public deleteFromSource: boolean = false;

    public updateProjectId(value: ProjectUuid | undefined) {
        this.$emit('update:projectId', value);
    }

    public async add() {
        await JobDetectionService.updateJobDelete(this.jobId, {
            delete_from_source: this.deleteFromSource,
            ignore: this.ignore,
        });

        this.$emit('jobUpdated', true);
    }
}

export default toNative(UpdateJobDelete);
</script>
