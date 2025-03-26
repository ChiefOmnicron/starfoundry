<template>
    <n-form
        ref="form_ref"
        style="margin-top: 15px; margin-left: 10px; margin-right: 10px"
        :model="info"
        :rules="rules"
    >
        <n-form-item path="name" label="Group name">
            <n-input
                v-model:value="info.name"
                placeholder="Name"
                @keydown.enter.prevent
            />
        </n-form-item>
    </n-form>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator'
import { FormRules, FormItemRule, NForm, NFormItem, NInput} from 'naive-ui';

import { IStructureGroup } from '@/sdk/structure_group';

@Component({
    components: {
        NForm,
        NFormItem,
        NInput,
    }
})
class StructureGroupGeneralInfo extends Vue {
    @Prop
    public info: IStructureGroup = <any>{};

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
    }
}

export default toNative(StructureGroupGeneralInfo);
</script>
