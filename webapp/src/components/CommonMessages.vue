<template>
    <n-alert
        :bordered="false"
        :title="messageTitle || ''"
        :type="messageType"
        @close="close()"
        closable
        style="margin-bottom: 10px"
        v-if="messageTitle && messageDescription"
    >
        {{ messageDescription || '' }}
    </n-alert>
</template>

<script lang="ts">
import { Component, Prop, Vue, Watch, toNative } from 'vue-facing-decorator';
import { NAlert } from 'naive-ui';

@Component({
    components: {
        NAlert,
    },
    emits: ['close'],
})
class CommonMessages extends Vue {
    @Prop({
        type: Object,
        required: true,
    })
    public message!: ICommonMessages;

    public messageTitle: string | null = null;
    public messageDescription: string | null = null;
    public messageType: string = 'info';

    @Watch('message', {
        deep: true,
    })
    public watchMessage() {
        if (this.message.createSuccess) {
            (this.messageTitle = 'Create Success'),
                (this.messageDescription =
                    'The entity was successfully created');
            this.messageType = 'success';
        } else if (this.message.updateSuccess) {
            (this.messageTitle = 'Update Success'),
                (this.messageDescription =
                    'The entity was successfully updated');
            this.messageType = 'success';
        } else if (this.message.createError) {
            (this.messageTitle = 'Create Error'),
                (this.messageDescription =
                    'Error while creating the entity. Try again later');
            this.messageType = 'error';
        } else if (this.message.deleteError) {
            (this.messageTitle = 'Delete Error'),
                (this.messageDescription =
                    'Error while deleting the entity. Try again later');
            this.messageType = 'error';
        } else if (this.message.loadingError) {
            (this.messageTitle = 'Loading Error'),
                (this.messageDescription =
                    'Error while loading data. Try again later');
            this.messageType = 'error';
        } else if (this.message.updateError) {
            (this.messageTitle = 'Update Error'),
                (this.messageDescription =
                    'Error while updating. Try again later');
            this.messageType = 'error';
        } else if (this.message.notFound) {
            (this.messageTitle = 'Not found'),
                (this.messageDescription =
                    'Entity not found, make sure the link is correct');
            this.messageType = 'warning';
        } else if (this.message.forbidden) {
            (this.messageTitle = 'Forbidden'),
                (this.messageDescription =
                    'You do not have the necessary permissions');
            this.messageType = 'error';
        }
    }

    public close() {
        this.$emit('close', true);
        this.messageTitle = null;
        this.messageDescription = null;
    }
}

export default toNative(CommonMessages);

export interface ICommonMessages {
    createSuccess: boolean;
    updateSuccess: boolean;

    createError: boolean;
    deleteError: boolean;
    loadingError: boolean;
    updateError: boolean;

    notFound: boolean;
    forbidden: boolean;

    hasError: (self: ICommonMessages) => boolean;
}

export const DEFAULT_COMMON_MESSAGES = (): ICommonMessages => {
    return {
        createSuccess: false,
        updateSuccess: false,

        createError: false,
        deleteError: false,
        loadingError: false,
        updateError: false,

        notFound: false,
        forbidden: false,

        hasError: (self: ICommonMessages) => {
            return (
                self.createError ||
                self.deleteError ||
                self.loadingError ||
                self.updateError ||
                self.notFound ||
                self.forbidden
            );
        },
    };
};
</script>
