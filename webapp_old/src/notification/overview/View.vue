<template>
    <div>
        <page-header title="Notifications" />

        <alert
            :visible="pageLoadingState.loadingError"
            @close="pageLoadingState.loadingError = false"
            data-cy="loadingError"
            description="There was an error loading the notifications"
            title="Loading Error"
        />

        <loader :busy="!pageLoadingState.showSpinner" data-cy="loaderInitial" />

        <no-entries
            description="No notification configured yet"
            data-cy="noEntries"
            v-if="pageLoadingState.showNoEntries && notifications.length === 0"
        >
            <n-button
                @click="$router.push({ name: routeNotificationCreate })"
                data-cy="noEntriesAddButton"
                size="small"
                type="info"
            >
                Add Notification
            </n-button>
        </no-entries>

        <div
            style="margin-bottom: 10px"
            data-cy="filter"
            v-if="
                pageLoadingState.isInitialDataLoaded &&
                (notifications.length > 0 || pageLoadingState.hasFilter)
            "
        >
            <filter-text
                :filters="filters"
                :load-initial="false"
                :options="filterOptions"
                :search-function="searchFunction"
                @busy="(s: any) => (pageLoadingState.loading = s)"
                @touched="(s: any) => (pageLoadingState.hasFilter = s)"
                v-model:entries="notifications"
            />

            <filter-element
                :filters="filters"
                :options="filterOptions"
                style="margin-top: 5px"
            />
        </div>

        <action-group
            data-cy="actionGroup"
            v-if="
                pageLoadingState.isInitialDataLoaded &&
                (notifications.length > 0 || pageLoadingState.hasFilter)
            "
        >
            <n-button
                @click="$router.push({ name: routeNotificationCreate })"
                type="info"
            >
                Add Notification
            </n-button>
        </action-group>

        <loader data-cy="loaderApi" :busy="pageLoadingState.loading" />

        <n-empty
            description="Your search did not result yield any results"
            data-cy="noSearchResult"
            size="large"
            style="margin: 5%"
            v-if="
                pageLoadingState.showNoSearchResult &&
                notifications.length === 0
            "
        />

        <card
            content-style="padding: 0"
            data-cy="dataTable"
            no-title
            v-if="pageLoadingState.showDataTable && notifications.length > 0"
        >
            <data-table
                :definitions="tableDefinition()"
                :entries="notifications"
                v-if="notifications.length > 0"
            />
        </card>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';

import {
    ROUTE_NOTIFICATION,
    ROUTE_NOTIFICATION_CREATE,
} from '@/notification/router';

import { Notification, NotificationService } from '@/sdk/notification';

import { NButton, NEmpty, NSpace, NTable } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import Alert from '@/components/Alert.vue';
import Card from '@/components/Card.vue';
import ConfirmDialog from '@/components/ConfirmDialog.vue';
import DataTable, {
    type IDataTableDefinition,
} from '@/components/DataTable.vue';
import FilterElement from '@/components/FilterElement.vue';
import FilterText, { type IFilterOption } from '@/components/Filter.vue';
import Loader from '@/components/Loader.vue';
import NoEntries from '@/components/NoEntries.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NButton,
        NEmpty,
        NSpace,
        NTable,

        ActionGroup,
        Alert,
        Card,
        ConfirmDialog,
        DataTable,
        FilterElement,
        FilterText,
        Loader,
        NoEntries,
        PageHeader,
    },
})
class NotificationOverview extends Vue {
    public pageLoadingState: PageLoadingState = new PageLoadingState();

    public notifications: Notification[] = [];
    public routeStockBlueprint: string = ROUTE_NOTIFICATION;
    public routeNotificationCreate: string = ROUTE_NOTIFICATION_CREATE;

    public filters: any = {};
    public filterOptions: { [key: string]: IFilterOption } = {};
    public searchFunction = NotificationService.list;

    public async created() {
        NotificationService.list({})
            .then((x) => {
                this.notifications = x;
                this.pageLoadingState.isInitialDataLoaded = true;
                this.filterDefinition();
            })
            .catch((_) => {
                this.notifications = [];
                this.pageLoadingState.isInitialDataLoaded = true;
                this.pageLoadingState.loadingError = true;
            });
    }

    public tableDefinition(): IDataTableDefinition[] {
        return [
            {
                header: 'Name',
                key: 'name',
                width: 400,
                visible: true,
                routing: {
                    route: ROUTE_NOTIFICATION,
                    key: 'notificationId',
                    value: 'id',
                },
            },
            {
                header: 'Target',
                key: 'target',
                visible: true,
            },
            {
                header: 'URL',
                key: 'url',
                visible: true,
            },
        ];
    }

    private filterDefinition() {
        this.filterOptions = {
            name: {
                label: 'Name',
            },
            target: {
                label: 'Target',
                options: ['DISCORD', 'JSON'],
            },
        };
    }
}

export default toNative(NotificationOverview);

class PageLoadingState {
    private _isInitialDataLoaded: boolean = false;
    private _hasFilter: boolean = false;
    private _loading: boolean = false;
    private _loadingError: boolean = false;

    get showNoEntries(): boolean {
        // the initial data has to be loaded
        return (
            this._isInitialDataLoaded &&
            // don't show it if there was an error
            !this._loadingError &&
            // if a filter is set, don't show
            !this._hasFilter
        );
    }

    get showNoSearchResult(): boolean {
        // the initial data has to be loaded
        return (
            this._isInitialDataLoaded &&
            // don't show it if there was an error
            !this._loadingError &&
            // only show it when a filter is set
            this._hasFilter
        );
    }

    get showDataTable(): boolean {
        // the initial data has to be loaded
        return (
            this._isInitialDataLoaded &&
            // don't show it if the application is currently loading
            !this._loading
        );
    }

    get showSpinner(): boolean {
        return this._isInitialDataLoaded;
    }

    get isInitialDataLoaded(): boolean {
        return this._isInitialDataLoaded;
    }

    set isInitialDataLoaded(state: boolean) {
        this._isInitialDataLoaded = state;
    }

    get hasFilter(): boolean {
        return this._hasFilter;
    }

    set hasFilter(filter: boolean) {
        this._hasFilter = filter;
    }

    get loading(): boolean {
        return this._loading;
    }

    set loading(filter: boolean) {
        this._loading = filter;
    }

    get loadingError(): boolean {
        return this._loadingError;
    }

    set loadingError(error: boolean) {
        this._loadingError = error;
    }
}
</script>
