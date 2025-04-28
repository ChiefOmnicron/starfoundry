<template>
    <div>
        <page-header title="notification.update.title" />

        <alert-save :visible="saved" @close="saved = false" />

        <alert
            :visible="errors.notFound"
            @close="errors.notFound = false"
            description="Could not find the notification you are searching for. Try another one."
            title="Notification not found"
        />

        <alert
            :visible="errors.updating"
            @close="errors.updating = false"
            description="There was an error while updating. Please try again at a later time"
            title="Error updating"
        />

        <alert
            :visible="errors.deleting"
            @close="errors.deleting = false"
            description="There was an error while deleting. Please try again at a later time"
            title="Error deleting"
        />

        <loader :busy="busy" />

        <card
            title="General Information"
            v-if="!busy && !errors.notFound && notification"
        >
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

        <webhook
            v-model:data="webhook"
            v-if="notification.target && notification.url"
        />

        <card-margin />

        <action-group v-if="!busy && !errors.notFound && notification">
            <n-button @click="$router.back()" quaternary> Back </n-button>

            <n-button @click="save" type="info"> Save </n-button>
        </action-group>

        <card-margin />

        <delete-object
            @delete="deleteObject"
            object-description="Deleting the Notification will remove it from all configured locations. This cannot be reversed."
            object-title="Delete Notification"
            v-if="!busy && !errors.notFound && notification"
        />
    </div>
</template>

<script lang="ts">
import { Component, Vue, Prop, toNative } from 'vue-facing-decorator';

import { ROUTE_NOTIFICATIONS } from '../router';

import { type Uuid } from '@/sdk/utils';
import {
    type INotification,
    Notification,
    NotificationService,
    type WebhookTarget,
} from '@/sdk/notification';

import { NButton, NInput, NSelect, type SelectOption } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import Alert from '@/components/Alert.vue';
import AlertSave from '@/components/AlertSave.vue';
import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import DeleteObject from '@/components/DeleteObject.vue';
import FormItem from '@/components/FormItem.vue';
import Loader from '@/components/Loader.vue';
import PageHeader from '@/components/PageHeader.vue';
import Webhook from '@/notification/components/Webhook.vue';

@Component({
    components: {
        NButton,
        NInput,
        NSelect,

        ActionGroup,
        Alert,
        AlertSave,
        Card,
        CardMargin,
        DeleteObject,
        FormItem,
        Loader,
        PageHeader,
        Webhook,
    },
})
class NotificationUpdate extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public notificationId!: Uuid;

    public notification: INotification = <any>{};
    public webhook: { target: WebhookTarget; url: string } = <any>{};
    public notification_obj!: Notification;

    public busy: boolean = false;
    public saved: boolean = false;
    public errors = {
        notFound: false,
        updating: false,
        deleting: false,
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

    public async created() {
        this.busy = true;
        await NotificationService.fetch(this.notificationId)
            .then((x) => {
                this.notification = {
                    url: x.url,
                    name: x.name,
                    target: x.target,
                };
                this.webhook = {
                    target: x.target,
                    url: x.url,
                };

                this.notification_obj = x;

                this.busy = false;
            })
            .catch((e) => {
                if (e.response.status === 404) {
                    this.errors.notFound = true;
                }

                this.busy = false;
            });
    }

    public async save() {
        this.notification_obj.name = this.notification.name;
        this.notification_obj.target =
            this.webhook.target || this.notification.target;
        this.notification_obj.url = this.webhook.url || this.notification.url;

        await this.notification_obj
            .save()
            .then((_) => {
                this.errors = {
                    notFound: false,
                    updating: false,
                    deleting: false,
                };
                this.saved = true;
            })
            .catch((_) => {
                this.errors = {
                    notFound: false,
                    updating: true,
                    deleting: false,
                };
                this.saved = false;
            });
    }

    public async deleteObject() {
        await this.notification_obj
            .remove()
            .then((_) => {
                this.$router.push({
                    name: ROUTE_NOTIFICATIONS,
                });
            })
            .catch((_) => {
                this.errors = {
                    notFound: false,
                    updating: false,
                    deleting: true,
                };
                this.saved = false;
            });
    }
}

export default toNative(NotificationUpdate);
</script>
