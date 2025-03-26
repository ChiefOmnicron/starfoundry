<template>
    <div>
        <page-header title="Overview" />

        <alert
            :visible="pageLoadingState.loadingError"
            @close="pageLoadingState.loadingError = false"
            data-cy="stockBlueprintLoadingError"
            description="There was an error loading the blueprint stocks"
            title="Loadingerror"
        />

        <loader
            :busy="!pageLoadingState.showSpinner"
            data-cy="stockBlueprintLoaderInitial"
        />

        <no-entries
            description="No Entries"
            data-cy="stockBlueprintNoEntries"
            v-if="pageLoadingState.showNoEntries && stocks.length === 0"
        >
            <n-button
                @click="$router.push({ name: routeStockBlueprintCreate })"
                data-cy="stockBlueprintNoEntriesAddButton"
                size="small"
                type="info"
            >
                Add Notification
            </n-button>
        </no-entries>

        <div
            style="margin-bottom: 10px"
            data-cy="stockBlueprintFilter"
            v-if="pageLoadingState.isInitialDataLoaded && (stocks.length > 0 || pageLoadingState.hasFilter)"
        >
            <filter-text
                :filters="filters"
                :load-initial="false"
                :options="filterOptions"
                :search-function="searchFunction"
                @busy="(s: any) => pageLoadingState.loading = s"
                @touched="(s: any) => pageLoadingState.hasFilter = s"
                v-model:entries="stocks"
            />

            <filter-element
                :filters="filters"
                :options="filterOptions"
                style="margin-top: 5px"
            />
        </div>

        <action-group
            data-cy="stockBlueprintActionGroup"
            v-if="pageLoadingState.isInitialDataLoaded && (stocks.length > 0 || pageLoadingState.hasFilter)"
        >
            <n-button
                @click="$router.push({ name: routeStockBlueprintCreate })"
                type="info"
            >
                Add Notification
            </n-button>
        </action-group>

        <loader
            data-cy="stockBlueprintLoaderApi"
            :busy="pageLoadingState.loading"
        />

        <n-empty
            description="Your search did not result yield any results"
            data-cy="stockBlueprintEmptyNoSearchResult"
            size="large"
            style="margin: 5%"
            v-if="pageLoadingState.showNoSearchResult  && stocks.length === 0"
        />

        <card
            content-style="padding: 0"
            data-cy="stockBlueprintDataTable"
            no-title
            v-if="pageLoadingState.showDataTable && stocks.length > 0"
        >
            <data-table
                :definitions="tableDefinition()"
                :entries="stocks"
                v-if="stocks.length > 0"
            />
        </card>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';

import { ROUTE_STOCK_BLUEPRINT, ROUTE_STOCK_BLUEPRINT_CREATE } from '@/stock/router';

import { StockBlueprint, StockBlueprintService } from '@/sdk/stockBlueprint';

import { NButton, NEmpty, NSpace, NTable } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import Alert from '@/components/Alert.vue';
import Card from '@/components/Card.vue';
import ConfirmDialog from '@/components/ConfirmDialog.vue';
import DataTable, { type IDataTableDefinition } from '@/components/DataTable.vue';
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
    }
})
class StockBlueprintList extends Vue {
    public pageLoadingState: PageLoadingState = new PageLoadingState();

    public stocks: StockBlueprint[] = [];
    public routeStockBlueprint: string = ROUTE_STOCK_BLUEPRINT;
    public routeStockBlueprintCreate: string = ROUTE_STOCK_BLUEPRINT_CREATE;

    public filters: any = {};
    public filterOptions: { [key: string]: IFilterOption } = {};
    public searchFunction = StockBlueprintService.list;

    public async created() {
        StockBlueprintService
            .list({})
            .then(x => {
                console.log(x)
                this.stocks = x;
                this.pageLoadingState.isInitialDataLoaded = true;
                this.filterDefinition();
            })
            .catch(_ => {
                this.stocks = [];
                this.pageLoadingState.isInitialDataLoaded = true;
                this.pageLoadingState.loadingError = true;
            });
    }

    public tableDefinition(): IDataTableDefinition[] {
        return [{
            header: 'Name',
            key: 'name',
            width: 400,
            visible: true,
            routing: {
                route: ROUTE_STOCK_BLUEPRINT,
                key: 'stockBlueprintId',
                value: 'id',
            },
        }];
    }

    private filterDefinition() {
        this.filterOptions = {
            name: {
                label: 'Name',
            },
        };
    }
}

export default toNative(StockBlueprintList);

class PageLoadingState {
    private _isInitialDataLoaded: boolean = false;
    private _hasFilter: boolean = false;
    private _loading: boolean = false;
    private _loadingError: boolean = false;

    get showNoEntries(): boolean {
        // the initial data has to be loaded
        return this._isInitialDataLoaded &&
            // don't show it if there was an error
            !this._loadingError &&
            // if a filter is set, don't show
            !this._hasFilter;
    }

    get showNoSearchResult(): boolean {
        // the initial data has to be loaded
        return this._isInitialDataLoaded &&
            // don't show it if there was an error
            !this._loadingError &&
            // only show it when a filter is set
            this._hasFilter;
    }

    get showDataTable(): boolean {
        // the initial data has to be loaded
        return this._isInitialDataLoaded &&
            // don't show it if the application is currently loading
            !this._loading;
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
