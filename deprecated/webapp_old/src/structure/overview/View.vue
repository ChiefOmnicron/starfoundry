<template>
    <div>
        <page-header title="Structures" />

        <alert
            :visible="pageLoadingState.loadingError"
            @close="pageLoadingState.loadingError = false"
            data-cy="structureLoadingError"
            description="structure.overview.errors.loadInitial.description"
            title="structure.overview.errors.loadInitial.title"
        />

        <loader
            :busy="!pageLoadingState.showSpinner"
            data-cy="structuresLoaderInitial"
        />

        <no-entries
            description="No Structures"
            data-cy="structuresNoEntries"
            v-if="pageLoadingState.showNoEntries && structures.length === 0"
        >
            <n-button
                @click="$router.push({ name: routeStructureCreate })"
                data-cy="structuresNoEntriesAddButton"
                size="small"
                type="info"
            >
                Add Structure
            </n-button>
        </no-entries>

        <div
            style="margin-bottom: 10px"
            data-cy="structuresFilter"
            v-if="
                pageLoadingState.isInitialDataLoaded &&
                (structures.length > 0 || pageLoadingState.hasFilter)
            "
        >
            <filter-text
                :filters="filters"
                :load-initial="false"
                :options="filterOptions"
                :search-function="searchFunction"
                @busy="(s: any) => (pageLoadingState.loading = s)"
                @touched="(s: any) => (pageLoadingState.hasFilter = s)"
                v-model:entries="structures"
            />

            <filter-element
                :filters="filters"
                :options="filterOptions"
                style="margin-top: 5px"
            />
        </div>

        <action-group
            data-cy="structuresActionGroup"
            v-if="
                pageLoadingState.isInitialDataLoaded &&
                (structures.length > 0 || pageLoadingState.hasFilter)
            "
        >
            <n-button
                @click="$router.push({ name: routeStructureCreate })"
                type="info"
            >
                Add Structure
            </n-button>
        </action-group>

        <loader
            data-cy="structuresLoaderApi"
            :busy="pageLoadingState.loading"
        />

        <n-empty
            description="Your search did not result yield any results"
            data-cy="structuresEmptyNoSearchResult"
            size="large"
            style="margin: 5%"
            v-if="
                pageLoadingState.showNoSearchResult && structures.length === 0
            "
        />

        <card
            content-style="padding: 0"
            data-cy="structuresDataTable"
            no-title
            v-if="pageLoadingState.showDataTable && structures.length > 0"
        >
            <data-table
                :definitions="tableDefinition()"
                :entries="structures"
                v-if="structures.length > 0"
            />
        </card>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { h, type VNode } from 'vue';

import { ROUTE_STRUCTURE, ROUTE_STRUCUTRES_CREATE } from '@/structure/router';

import type { TypeId } from '@/sdk/utils';
import { Structure, StructureService } from '@/sdk/structure';

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
import Item from '@/components/Item.vue';
import Loader from '@/components/Loader.vue';
import NoEntries from '@/components/NoEntries.vue';
import PageHeader from '@/components/PageHeader.vue';
import System from '@/components/System.vue';

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
        Item,
        Loader,
        NoEntries,
        PageHeader,
        System,
    },
})
class StructureOverview extends Vue {
    public pageLoadingState: PageLoadingState = new PageLoadingState();

    public structures: Structure[] = [];
    public routeStructure: string = ROUTE_STRUCTURE;
    public routeStructureCreate: string = ROUTE_STRUCUTRES_CREATE;

    public filters: any = {};
    public filterOptions: { [key: string]: IFilterOption } = {};
    public searchFunction = StructureService.list;

    public async created() {
        StructureService.list({})
            .then((x) => {
                this.structures = x;
                this.pageLoadingState.isInitialDataLoaded = true;
                this.filterDefinition();
            })
            .catch((_) => {
                this.structures = [];
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
                    route: ROUTE_STRUCTURE,
                    key: 'structureId',
                    value: 'id',
                },
            },
            {
                header: 'Location',
                key: 'location',
                width: 300,
                visible: true,
                render(row: Structure): VNode {
                    return h(System, {
                        systemId: row.systemId,
                        dotlan: true,
                    });
                },
            },
            {
                header: 'Type',
                key: 'structureTypeId',
                item: true,
                width: 200,
                visible: true,
            },
            {
                header: 'Services',
                key: 'services',
                width: 400,
                visible: true,
                array: true,
                item: true,
                transform: this.formatService,
            },
            {
                header: 'Rigs',
                key: 'rigs',
                width: 700,
                visible: true,
                array: true,
                item: true,
                transform: this.formatService,
            },
        ];
    }

    private filterDefinition() {
        this.filterOptions = {
            name: {
                label: 'structure.overview.filters.name',
            },
            /*rigs: {
                label: 'structure.overview.filters.rigs',
                name: 'name',
                key: 'type_id',
                multiple: true,
                item: true,
                options: [...new Set(
                    (this.structures || [])
                        .map(x => x.rigs)
                        .flat()
                )],
                transform: (
                    _: IFilterOption,
                    value: string,
                ) => this.formatService(value),
            },
            services: {
                label: 'structure.overview.filters.services',
                name: 'name',
                key: 'type_id',
                multiple: true,
                item: true,
                options: this.service_options(),
                transform: (
                    _: IFilterOption,
                    value: string,
                ) => this.formatService(value),
            },*/
            system_id: {
                label: 'structure.overview.filters.systemId',
                name: 'name',
                key: 'system_id',
                options: this.systemOptions(),
                transform: (
                    filter: IFilterOption,
                    system_id: number,
                ): string => {
                    if (filter.options) {
                        const value: {
                            name: string;
                            system_id: number;
                        } = filter.options.find(
                            (x) => x.system_id === system_id,
                        );
                        if (value) {
                            return value.name;
                        } else {
                            return 'Unkown system';
                        }
                    } else {
                        return 'Unkown system';
                    }
                },
            },
            type_id: {
                label: 'structure.overview.filters.typeId',
                name: 'name',
                key: 'type_id',
                item: true,
                options: this.structure_options(),
            },
        };
    }

    private formatService(name: string): string {
        if (!name) {
            return '';
        }
        return name
            .replace('Standup ', '')
            .replace('XL-Set ', '')
            .replace('L-Set ', '')
            .replace('M-Set ', '');
    }

    /*private serviceOptions(): {
        name: string,
        type_id: TypeId,
    }[] {
        let services = (this.structures || [])
            .map(x => x.services)
            .flat()
            .map(x => {
                return {
                    type_id: x,
                    name: this.formatService(ItemService.get_sync(x).name),
                }
            });
        return [...new Set(services)];
    }*/

    private systemOptions(): {
        name: string;
        system_id: TypeId;
    }[] {
        let systems = (this.structures || []).map((x) => {
            return {
                name: x.systemName,
                system_id: x.systemId,
            };
        });
        return [...new Set(systems)];
    }

    private structure_options(): {
        name: string;
        type_id: TypeId;
    }[] {
        let systems = (this.structures || []).map((x) => {
            return {
                name: x.structureName,
                type_id: x.structureTypeId,
            };
        });
        return [...new Set(systems)];
    }
}

export default toNative(StructureOverview);

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
