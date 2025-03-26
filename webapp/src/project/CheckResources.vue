<template>
    <div>
        <div>
            <p style="margin-left: 10px">
                Insert all materials that you plan on using for the selected jobs.
                It will check if you have enough resources, and if not how much is missing.
                <br><br>
                The format looks like:
                <code>ItemName Quantity</code>
            </p>

            <n-input
                v-model:value="available_resources"
                type="textarea"
                placeholder="Insert available materials"
                rows="10"
            />
        </div>

        <div v-if="missing_materials.length > 0">
            <n-table>
                <thead>
                    <tr>
                        <th width="25px"></th>
                        <th width="500px">Name</th>
                        <th width="200px">Missing</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="entry in missing_materials" :key="entry.type_id">
                        <td v-if="entry.quantity > 0">
                            <eve-icon :id="entry.type_id" type="icon" item />
                        </td>
                        <td v-if="entry.quantity > 0">
                            <copy-text :value="entry.type_id" item />
                        </td>
                        <td v-if="entry.quantity > 0">
                            <copy-text :value="entry.quantity" number />
                        </td>
                    </tr>
                </tbody>
            </n-table>
        </div>

        <div v-if="missing_materials.length === 0 && check_performed">
            <n-result
                status="success"
                title="All good"
                description="There are enough resources to start all jobs"
            />
        </div>

        <action-group>
            <n-button
                @click="close()"
                quaternary
            >
                Close
            </n-button>

            <n-button
                @click="check_resources"
                type="info"
            >
                Check
            </n-button>
        </action-group>
    </div>
</template>

<script lang="ts">
import { Component, Vue, Prop, toNative } from 'vue-facing-decorator';
import { NButton, NInput, NResult, NTable } from 'naive-ui';

import { ProjectService } from '@/sdk/project';
import { ItemService } from '@/sdk/item';
import type { TypeId, Uuid } from '@/sdk/utils';

import ActionGroup from '@/components/ActionGroup.vue';
import CopyText from '@/components/CopyText.vue';
import EveIcon from '@/components/EveIcon.vue';

@Component({
    components: {
        NButton,
        NInput,
        NResult,
        NTable,

        ActionGroup,
        CopyText,
        EveIcon,
    },
    emits: ['close']
})
class CheckResources extends Vue {
    @Prop({
        required: true,
        type:     Array<Uuid>
    })
    public projectJobIds!: Uuid[];

    public check_performed: boolean = false;

    public available_resources: string = '';
    public missing_materials: { type_id: TypeId, quantity: number }[] = [];

    public async check_resources() {
        this.check_performed = false;

        const resources = await ItemService.parse(this.available_resources);

        this.missing_materials = await ProjectService
            .check_resources(
                {
                    job_ids: this.projectJobIds,
                    resources: <any>resources
                }
            )
            .then(x => {
                this.check_performed = true;
                return x
                    .filter(y => y.quantity > 0);
            });
    }

    public close() {
        this.$emit('close', true);
    }
}

export default toNative(CheckResources);
</script>
