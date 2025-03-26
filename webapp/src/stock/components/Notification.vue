<template>
    <div>
        <n-table>
            <thead>
                <tr>
                    <th width="73%">Name</th>
                    <th width="10%">Target</th>
                    <th width="7%"></th>
                </tr>
            </thead>

            <tbody>
                <notification-wrapper
                    :notification-id="notification"
                    v-slot="{ entry }"
                    v-for="notification in notifications" :key="notification"
                >
                    <tr>
                        <td>
                            <div
                                @click="openNotification(notification)"
                                style="cursor: pointer; color: #70c0e8"
                            >
                                {{ entry.name }}

                                <n-icon size="10">
                                    <external-link-alt />
                                </n-icon>
                            </div>
                        </td>
                        <td>{{ entry.target }}</td>
                        <td>
                            <n-button
                                :quaternary="true"
                                @click="deleteNotification(entry.id)"
                                type="error"
                            >
                                Remove
                            </n-button>
                        </td>
                    </tr>
                </notification-wrapper>
            </tbody>

            <tfoot>
                <tr>
                    <td colspan="2">
                        <notification-selector
                            :notifications="[]"
                            :selected="notifications"
                            v-model:value="newNotification"
                        />
                    </td>
                    <td>
                        <n-button
                            :ghost="true"
                            @click="addNotification"
                            style="width: 100%"
                            type="info"
                        >
                            Add
                        </n-button>
                    </td>
                </tr>
            </tfoot>
        </n-table>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { ROUTE_NOTIFICATION } from '@/notification/router';

import type { Uuid } from '@/sdk/utils';

import { NButton, NIcon, NTable } from 'naive-ui';
import { ExternalLinkAlt } from '@vicons/fa';
import NotificationSelector from '@/components/selectors/NotificationSelector.vue';
import NotificationWrapper from '@/components/NotificationWrapper.vue';

@Component({
    components: {
        NButton,
        NIcon,
        NTable,

        ExternalLinkAlt,

        NotificationSelector,
        NotificationWrapper,
    },
    emits: [
        'update:notifications',
    ]
})
class Notification extends Vue {
    @Prop({
        default: [],
        type: Array,
        required: true,
    })
    public notifications!: string[];

    public newNotification: string = '';

    public addNotification() {
        if (this.newNotification.length > 0) {
            this.notifications.push(this.newNotification);
        }

        this.newNotification = '';
    }

    public async deleteNotification(id: Uuid) {
        this.$emit('update:notifications', this.notifications.filter(x => x !== id));
    }

    public openNotification(notification_id: Uuid) {
        let route = this.$router.resolve({
            name: ROUTE_NOTIFICATION,
            params: {
                notificationId: notification_id,
            }
        });
        window.open(route.href);
    }
}

export default toNative(Notification);
</script>
