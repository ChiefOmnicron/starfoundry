<template>
    <card title="Webhook">
        <template #action>
            <n-button
                :disabled="testMessage.isSending"
                :loading="testMessage.isSending"
                @click="sendTestMessage"
                type="info"
            >
                Send Test Message
            </n-button>
        </template>

        <div style="margin: 10px">
            <form-item label="Integration">
                <n-select
                    placeholder="Insert integration name"
                    v-model:value="data.target"
                    :options="targetOptions"
                />
            </form-item>

            <form-item label="URL">
                <n-input
                    placeholder="Insert webhook url"
                    @keydown.enter.prevent
                    v-model:value="data.url"
                />
            </form-item>

            <n-alert
                title="Success"
                @close="testMessage.success = undefined"
                closable
                style="margin-bottom: 10px"
                type="success"
                v-if="testMessage.success"
            >
                Sending Test Message Successful. Response:<br />
                {{ testMessage.success }}
            </n-alert>

            <n-alert
                title="Error"
                @close="testMessage.error = undefined"
                closable
                style="margin-bottom: 10px"
                type="error"
                v-if="testMessage.error"
            >
                Error while sending test message. Response:<br />
                {{ testMessage.error }}
            </n-alert>
        </div>
    </card>
</template>

<script lang="ts">
import { NotificationService, type WebhookTarget } from '@/sdk/notification';
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NAlert, NButton, NInput, NSelect, type SelectOption } from 'naive-ui';
import Card from '@/components/Card.vue';
import FormItem from '@/components/FormItem.vue';

@Component({
    components: {
        NAlert,
        NButton,
        NInput,
        NSelect,

        Card,
        FormItem,
    },
})
class NotificationCreate extends Vue {
    @Prop({
        type: Object,
        required: true,
    })
    public data: {
        target: WebhookTarget;
        url: string;
    } = <any>{};

    public testMessage: ITestMessage = {
        isSending: false,
        success: undefined,
        error: undefined,
    };

    public targetOptions: SelectOption[] = [
        {
            label: 'Discord',
            value: 'DISCORD',
        },
        {
            label: 'JSON',
            value: 'JSON',
        },
    ];

    public sendTestMessage() {
        this.testMessage.isSending = true;

        NotificationService.testMessage(this.data.target, this.data.url)
            .then((x) => {
                this.testMessage.success = x || 'Empty Response';
                this.testMessage.error = undefined;
                this.testMessage.isSending = false;
            })
            .catch((x) => {
                this.testMessage.error = x;
                this.testMessage.success = undefined;
                this.testMessage.isSending = false;
            });
    }
}

export default toNative(NotificationCreate);

interface ITestMessage {
    isSending: boolean;
    success: string | undefined;
    error: string | undefined;
}
</script>
