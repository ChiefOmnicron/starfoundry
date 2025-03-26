<template>
    <n-dropdown
        :options="appraisalOptions"
        @select="selectAppraisal"
        trigger="hover"
        v-if="appraisalOptions.length > 1"
    >
        <n-button
            type="info"
            :disabled="busy"
        >
            Update price with ...
        </n-button>
    </n-dropdown>


    <n-button
        @click="selectAppraisal('INTERNAL')"
        type="info"
        v-if="appraisalOptions.length === 1"
    >
        Update price based on Jita sell
    </n-button>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NButton, NDropdown, type SelectOption } from 'naive-ui';
import { FeatureFlagService } from '@/sdk/featureFlags';

@Component({
    components: {
        NButton,
        NDropdown,
    },
    emits: [
        'select'
    ]
})
class AppraisalSelector extends Vue {
    public appraisalOptions: SelectOption[] = [];

    @Prop({
        type: Boolean,
        required: true
    })
    public busy!: boolean

    public async created() {
        this.appraisalOptions.push({
            label: 'Internal Appraisal based on Jita sell',
            key: 'INTERNAL',
        });

        const featureFlags = await FeatureFlagService.fetch();

        if (featureFlags.indexOf('JANICE') > -1) {
            this.appraisalOptions.push({
                label: 'Janice Appraisal based on Jita sell',
                key: 'JANICE',
            });
        }
    }

    public selectAppraisal(selected: string) {
        this.$emit('select', selected);
    }
}

export default toNative(AppraisalSelector);
</script>
