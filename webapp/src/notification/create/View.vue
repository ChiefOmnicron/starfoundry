<template>
    <div>
        <page-header title="Create Notification" />

        <alert
            :visible="errors.unexpected"
            @close="errors.unexpected = false"
            description="Unexpected error while creating the notification."
            title="Error creating notification"
        />

        <card title="General Information">
            <div style="margin: 10px">
                <form-item label="Name">
                    <n-input
                        placeholder="Insert notification name"
                        @keydown.enter.prevent
                        v-model:value="notification.name"
                    />
                </form-item>
            </div>
        </card>

        <card-margin />

        <webhook v-model:data="webhook" />

        <card-margin />

        <action-group>
            <n-button @click="$router.back()" quaternary> Cancel </n-button>

            <n-button
                :disabled="!notification.name"
                @click="create"
                type="info"
            >
                Create Notification
            </n-button>
        </action-group>
    </div>
</template>

<script lang="ts">
import {
    type INotification,
    NotificationService,
    type WebhookTarget,
} from '@/sdk/notification';
import { Component, Vue, toNative } from 'vue-facing-decorator';

import { ROUTE_NOTIFICATION } from '../router';

import { NAlert, NButton, NInput, NSelect } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import Alert from '@/components/Alert.vue';
import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import FormItem from '@/components/FormItem.vue';
import PageHeader from '@/components/PageHeader.vue';
import Webhook from '@/notification/components/Webhook.vue';

@Component({
    components: {
        NAlert,
        NButton,
        NInput,
        NSelect,

        ActionGroup,
        Alert,
        Card,
        CardMargin,
        FormItem,
        PageHeader,
        Webhook,
    },
})
class NotificationCreate extends Vue {
    public notification: INotification = <any>{};

    public errors = {
        unexpected: false,
    };

    public webhook: {
        target: WebhookTarget;
        url: string;
    } = <any>{};

    public async create() {
        NotificationService.create({
            ...this.notification,
            ...this.webhook,
        })
            .then((x) => {
                this.$router.push({
                    name: ROUTE_NOTIFICATION,
                    params: {
                        notificationId: x.id,
                    },
                });
            })
            .catch((_) => (this.errors.unexpected = true));
    }
}

export default toNative(NotificationCreate);
</script>
