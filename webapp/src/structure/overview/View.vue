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
        >
            <n-data-table
                :columns="tableDefinition()"
                :data="structures"
                :row-key="(row: Structure) => row.id"
                striped
            />
        </card>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { h, type VNode } from 'vue';

import { ROUTE_STRUCTURE, ROUTE_STRUCTURES_CREATE } from '@/structure/router';

import type { TypeId } from '@/sdk/utils';
import { Structure, StructureService } from '@/sdk/structure';
import { ItemService, type IItem } from '@/sdk/item';

import { renderFunction as itemRenderFunction } from '@/components/Item.vue';
import { renderFunction as systemRenderFunction } from '@/components/System.vue';

import { NButton, NDataTable, NEmpty, NSpace, NTable } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import Alert from '@/components/Alert.vue';
import Card from '@/components/Card.vue';
import ConfirmDialog from '@/components/ConfirmDialog.vue';
import FilterElement from '@/components/FilterElement.vue';
import FilterText, { type IFilterOption } from '@/components/FilterV2.vue';
import Loader from '@/components/Loader.vue';
import NoEntries from '@/components/NoEntries.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NButton,
        NDataTable,
        NEmpty,
        NSpace,
        NTable,

        ActionGroup,
        Alert,
        Card,
        ConfirmDialog,
        FilterElement,
        FilterText,
        Loader,
        NoEntries,
        PageHeader,
    },
})
class StructureOverview extends Vue {
    public pageLoadingState: PageLoadingState = new PageLoadingState();

    public structures: Structure[] = [];
    public routeStructure: string = ROUTE_STRUCTURE;
    public routeStructureCreate: string = ROUTE_STRUCTURES_CREATE;

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

    public tableDefinition() {
        return [
            {
                title: 'Name',
                key: 'name',
            },
            {
                title: 'Location',
                key: 'location',
                render: (row: Structure): VNode => {
                    return systemRenderFunction(
                        row.systemId,
                        true,
                    );
                }
            },
            {
                title: 'Type',
                key: 'structureTypeId',
                filter(value: TypeId, row: Structure) {
                    return row.structureTypeId === value;
                },
                render: (row: Structure): VNode => {
                    return itemRenderFunction(
                        row.structureTypeId,
                    )
                }
            },
            {
                title: 'Services',
                key: 'services',
                filter(value: TypeId, row: Structure) {
                    return row.services.indexOf(value) > -1;
                },
                render: (row: Structure): VNode => {
                    return h(
                        'div',
                        {},
                        [
                            row
                                .services
                                .map(x => h(
                                    'div',
                                    {},
                                    itemRenderFunction(x, this.formatService),
                                ))
                        ]
                    )
                }
            },
            {
                title: 'Rigs',
                key: 'rigs',
                filter(value: TypeId, row: Structure) {
                    return row.rigs.indexOf(value) > -1;
                },
                render: (row: Structure): VNode => {
                    return h(
                        'div',
                        {},
                        [
                            row
                                .rigs
                                .map(x => h(
                                    'div',
                                    {},
                                    itemRenderFunction(x, this.formatService),
                                ))
                        ]
                    )
                }
            },
        ]
    }

    private filterDefinition() {
        this.filterOptions = {
            name: {
                label: 'Name',
            },
            rigs: {
                label: 'Rigs',
                key: 'rigs',
                multiple: true,
                options: [...new Set(this.structures.flatMap(x => x.rigs))]
                    .map(x => {
                        return {
                            label: this.formatService(ItemService.getSync(x).name),
                            key: x,
                        }
                    }),
                transform: (
                    _: IFilterOption,
                    value: TypeId,
                ) => {
                    let item = ItemService.getSync(value);
                    return this.formatService(item.name);
                },
            },
            services: {
                label: 'Services',
                key: 'services',
                multiple: true,
                options: [...new Set(this.structures.flatMap(x => x.services))]
                    .map(x => {
                        return {
                            label: this.formatService(ItemService.getSync(x).name),
                            key: x,
                        }
                    }),
                transform: (
                    _: IFilterOption,
                    value: TypeId,
                ) => {
                    let item = ItemService.getSync(value);
                    return this.formatService(item.name);
                },
            },
            /*system_id: {
                label: 'System',
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
                            return 'Unknown system';
                        }
                    } else {
                        return 'Unknown system';
                    }
                },
            },*/
            /*type_id: {
                label: 'Structure Type',
                name: 'name',
                key: 'type_id',
                options: this.structure_options(),
            },*/
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
