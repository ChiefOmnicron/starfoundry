<template>
    <n-form
        ref="form_ref"
        style="margin-top: 15px; margin-left: 10px; margin-right: 10px"
        :model="info"
        :rules="rules"
    >
        <n-form-item path="name" label="Name">
            <n-input
                :disabled="readonly"
                placeholder="Name"
                v-model:value="info.name"
            />
        </n-form-item>

        <n-form-item path="project_group_id" label="Project Group">
            <project-group-selector
                :readonly="readonly"
                v-model:value="info.project_group_id"
                project-permission="WRITE"
                @update:value="selectProjectGroupId"
            />
        </n-form-item>

        <n-form-item path="orderer" label="Orderer">
            <n-input
                :disabled="readonly"
                v-model:value="info.orderer"
                placeholder="Orderer"
            />
        </n-form-item>

        <n-form-item path="sell_price" label="Sell price">
            <format-number-input
                :readonly="readonly"
                v-model:value="info.sell_price"
                placeholder="Sell price"
                style="width: 100%"
            />
        </n-form-item>

        <n-form-item path="note" label="Note">
            <n-input
                :disabled="readonly"
                type="textarea"
                v-model:value="info.note"
                placeholder="Note"
            />
        </n-form-item>

        <n-form-item path="status" label="Status" v-if="showStatus">
            <n-select
                :disabled="readonly"
                :options="options"
                v-model:value="info.status"
            />
        </n-form-item>
    </n-form>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import {
    type FormRules,
    type FormItemRule,
    NButton,
    NForm,
    NFormItem,
    NInput,
    NSelect,
    type SelectOption,
} from 'naive-ui';
import type { IProject } from '@/sdk/project';

import Card from '@/components/Card.vue';
import FormatNumberInput from '@/components/inputs/FormatNumber.vue';
import ProjectGroupSelector from '@/components/selectors/ProjectGroupSelector.vue';
import StructureGroupSelector from '@/components/selectors/StructureGroupSelector.vue';

@Component({
    components: {
        NButton,
        NForm,
        NFormItem,
        NInput,
        NSelect,

        Card,
        FormatNumberInput,
        ProjectGroupSelector,
        StructureGroupSelector,
    },
    emits: ['update:info', 'update:projectGroup'],
})
class ProjectGeneralInfo extends Vue {
    @Prop({
        type: Object,
        required: true,
    })
    public info!: IProject;

    @Prop({
        type: Boolean,
        required: false,
        default: true,
    })
    public showStatus!: boolean;

    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public readonly!: boolean;

    public options: SelectOption[] = [
        {
            label: 'Preparing',
            value: 'PREPARING',
        },
        {
            label: 'In Progress',
            value: 'IN_PROGRESS',
        },
        {
            label: 'Paused',
            value: 'PAUSED',
        },
        {
            label: 'Done',
            value: 'DONE',
        },
    ];

    public selectProjectGroupId(value: string) {
        this.$emit('update:projectGroup', value);
    }

    public rules: FormRules = {
        name: [
            {
                required: true,
                validator(_: FormItemRule, value: string) {
                    if (!value || value === '') {
                        return new Error('This field is required');
                    }
                    return true;
                },
                trigger: ['input', 'blur'],
            },
        ],
        orderer: [
            {
                required: true,
                validator(_: FormItemRule, value: string) {
                    if (!value || value === '') {
                        return new Error('This field is required');
                    }
                    return true;
                },
                trigger: ['input', 'blur'],
            },
        ],
        project_group_id: [
            {
                required: true,
                validator(_: FormItemRule, value: string) {
                    if (!value || value === '') {
                        return new Error('This field is required');
                    }
                    return true;
                },
                trigger: ['input', 'blur'],
            },
        ],
    };
}

export default toNative(ProjectGeneralInfo);
</script>
