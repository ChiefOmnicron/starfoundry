<template>
    <n-form
        ref="form_ref"
        style="margin-top: 15px; margin-left: 10px; margin-right: 10px"
        :model="data"
        :rules="rules"
    >
        <n-form-item path="name" label="Name">
            <n-input
                :disabled="readonly"
                placeholder="Name"
                v-model:value="data.name"
            />
        </n-form-item>

        <n-form-item path="project_group_id" label="Project Group">
            <project-group-selector
                :readonly="readonly"
                v-model:value="data.project_group_id"
                project-permission="WRITE"
            />
        </n-form-item>

        <n-form-item path="structure_group_id" label="Structure Group" v-if="!showStatus">
            <structure-group-selector v-model:value="data.structure_group_id" />
        </n-form-item>

        <n-form-item path="orderer" label="Orderer">
            <n-input
                :disabled="readonly"
                v-model:value="data.orderer"
                placeholder="Orderer"
            />
        </n-form-item>

        <n-form-item path="sell_price" label="Sell price">
            <format-number-input
                :readonly="readonly"
                v-model:value="data.finance.sell_price"
                placeholder="Sell price"
                style="width: 100%"
            />
        </n-form-item>

        <n-form-item path="notes" label="Notes">
            <n-input
                :disabled="readonly"
                type="textarea"
                v-model:value="data.notes"
                placeholder="Notes"
            />
        </n-form-item>

        <n-form-item path="status" label="Status" v-if="showStatus">
            <n-select
                :disabled="readonly"
                :options="options"
                v-model:value="data.status"
            />
        </n-form-item>
    </n-form>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { type FormRules, type FormItemRule, NButton, NForm, NFormItem, NInput, NSelect, type SelectOption } from 'naive-ui';
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
})
class ProjectGeneralInfo extends Vue {
    @Prop({
        type: Object,
        required: true,
    })
    public info!: IProject;

    @Prop({
        type:     Boolean,
        required: false,
        default:  true,
    })
    public showStatus!: boolean;

    @Prop({
        type: Boolean,
        required: false,
        default: false,
    })
    public readonly!: boolean;

    public data: any = <any>{};

    public created() {
        this.data = this.info;
    }

    public options: SelectOption[] = [{
        label: 'Preparing',
        value: 'PREPARING',
    }, {
        label: 'In Progress',
        value: 'IN_PROGRESS',
    }, {
        label: 'Paused',
        value: 'PAUSED',
    }, {
        label: 'Done',
        value: 'DONE',
    }];

    public rules: FormRules = {
        name: [{
            required: true,
            validator (_: FormItemRule, value: string) {
                if (!value || value === '') {
                    return new Error('This field is required')
                }
                return true
            },
            trigger: ['input', 'blur']
        }],
        orderer: [{
            required: true,
            validator (_: FormItemRule, value: string) {
                if (!value || value === '') {
                    return new Error('This field is required')
                }
                return true
            },
            trigger: ['input', 'blur']
        }],
        project_group_id: [{
            required: true,
            validator (_: FormItemRule, value: string) {
                if (!value || value === '') {
                    return new Error('This field is required')
                }
                return true
            },
            trigger: ['input', 'blur']
        }],
        structure_group_id: [{
            required: true,
            validator (_: FormItemRule, value: string) {
                if (!value || value === '') {
                    return new Error('This field is required')
                }
                return true
            },
            trigger: ['input', 'blur']
        }],
    }
}

export default toNative(ProjectGeneralInfo);
</script>
