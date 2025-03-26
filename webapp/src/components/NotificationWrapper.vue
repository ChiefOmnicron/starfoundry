<template>
    <slot :entry="entry"></slot>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { Notification, NotificationService } from '@/sdk/notification';
import { Uuid } from '@/sdk/utils';

@Component({
    components: {},
    emits: [
        'loading',
        'loading:error',
    ]
})
class NotificationWrapper extends Vue {
    @Prop({
        type:     String,
        required: true
    })
    public notificationId!: Uuid;

    public entry: Notification = <any>{};

    public async created() {
        this.$emit('loading', true);

        await NotificationService
            .fetch(this.notificationId)
            .then(x => {
                this.entry = x;
                this.$emit('loading', false);
            })
            .catch((_: any) => {
                this.$emit('loading:error', 'Error loading item');
            });
    }
}

export default toNative(NotificationWrapper);
</script>
