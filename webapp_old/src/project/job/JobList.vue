<template>
    <n-table>
        <thead>
            <tr>
                <th width="1%">
                    <n-checkbox
                        v-model:checked="all_selected"
                        @update:checked="handle_select_all"
                        :indeterminate="partially_all_selected"
                    >
                    </n-checkbox>
                </th>
                <th width="1%"></th>
                <th width="25%">Name</th>
                <th width="20%">Runs</th>
                <th width="30%">Structure</th>
                <th></th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="(job, i) in jobs" :key="job.type_id">
                <td>
                    <n-checkbox
                        v-model:checked="selected[i]"
                        @update:checked="handle_select"
                    >
                    </n-checkbox>
                </td>
                <td>
                    <eve-icon :id="job.type_id" type="icon" item />
                </td>
                <td>
                    <copy-text :value="job.item_name" />
                </td>
                <td>
                    <copy-text :value="job.runs" number />
                </td>
                <td>
                    <structure-wrapper
                        :structure-id="job.structure_uuid"
                        v-slot="{ structure }"
                    >
                        {{ structure.name }}
                    </structure-wrapper>
                </td>
                <td>
                    <div style="display: flex; justify-content: flex-end">
                        <n-button @click="started(i)"> Hide </n-button>
                    </div>
                </td>
            </tr>
        </tbody>
    </n-table>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NCheckbox, NTable } from 'naive-ui';
import type { IJob } from '@/project/service';
import type { Uuid } from '@/sdk/utils';

import CopyText from '@/components/CopyText.vue';
import EveIcon from '@/components/EveIcon.vue';
import StructureWrapper from '@/structure/components/Wrapper.vue';

@Component({
    components: {
        NButton,
        NCheckbox,
        NTable,

        CopyText,
        EveIcon,
        StructureWrapper,
    },
    emits: ['update:selected_ids'],
})
class ProjectJobOverview extends Vue {
    @Prop({
        type: Array,
        required: true,
    })
    public jobs!: IJob[];

    public selected: boolean[] = [];
    public selected_ids: Uuid[] = [];

    public all_selected: boolean = false;
    public partially_all_selected: boolean = false;

    public started(index: number) {
        this.jobs.splice(index, 1);
    }

    public handle_select() {
        this.selected_ids = this.selected
            .map((value, index) => {
                if (value) {
                    return this.jobs[index].id;
                } else {
                    return <any>null;
                }
            })
            .filter((x) => x != null);
        this.$emit('update:selected_ids', this.selected_ids);

        if (this.selected_ids.length === 0) {
            return;
        } else if (this.selected_ids.length === this.jobs.length) {
            this.all_selected = true;
            this.partially_all_selected = false;
        } else {
            this.all_selected = true;
            this.partially_all_selected = true;
        }
    }

    public handle_select_all() {
        if (this.all_selected) {
            this.all_selected = true;
            this.selected = this.jobs.map((_) => true);
        } else {
            this.all_selected = false;
            this.partially_all_selected = false;
            this.selected = this.jobs.map((_) => false);
        }

        this.handle_select();
    }
}

export default toNative(ProjectJobOverview);
</script>
