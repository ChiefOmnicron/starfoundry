<template>
    <n-select
        :loading="busy"
        :options="possibleOptions()"
        @update:value="updateValue"
        data-cy="NotificationSelector"
        filterable
        placeholder="Select notificaton"
        v-model:value="value"
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { Uuid } from '@/sdk/utils';
import { Notification, NotificationService } from '@/sdk/notification';

import { NText, NSelect, type SelectOption } from 'naive-ui';

@Component({
    components: {
        NText,
        NSelect,
    },
    emits: [
        'error',
        'update:value',
    ]
})
class NotificationSelector extends Vue {
    @Prop({
        required: true
    })
    public notifications!: string;

    @Prop({
        default: [],
        required: false,
    })
    public selected!: Uuid[];

    public options: SelectOption[] = [];
    public value!: string | string[] | null;

    public busy: boolean = false;

    public async created() {
        this.busy = true;
        await NotificationService
            .list({})
            .then((x: Notification[]) => {
                this.busy = false;

                x.map((x: Notification) => {
                    this.options.push({
                        label: x.name,
                        value: x.id,
                    });
                });
            })
            .catch(e => {
                this.busy = false;
                this.$emit('error', true)
            });
    }

    public possibleOptions(): SelectOption[] {
        return this.options.filter(x => !this.selected.includes(<any>x.value))
    }

    public updateValue(value: string | string[] | null) {
        this.value = value;
        this.$emit('update:value', this.value);
    }
}

export default toNative(NotificationSelector);
</script>
