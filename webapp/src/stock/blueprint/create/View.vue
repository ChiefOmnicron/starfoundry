<template>
    <div>
        <page-header
            title="Create Blueprint Stock"
            data-cy="stockBlueprintCreateTitle"
        />

        <alert
            :visible="errors.creating"
            @close="errors.creating = false"
            data-cy="stockBlueprintCreateError"
            description="Error creating"
            title="There was an error while creating. Please try again at a later time."
        />

        <loader
            :busy=busy
            data-cy="stockBlueprintCreateLoading"
        />

        <card
            data-cy="stockBlueprintCreateGeneralInformation"
            title="General information"
            v-if="!busy"
        >
            <div style="margin: 10px">
                <form-item
                    label="Name"
                    required
                >
                    <n-input
                        placeholder="Name"
                        @keydown.enter.prevent
                        v-model:value="name"
                    />
                </form-item>

                <form-item
                    label="Description"
                    required
                >
                    <n-input
                        placeholder="Insert Description"
                        @keydown.enter.prevent
                        v-model:value="description"
                    />
                </form-item>
            </div>
        </card>

        <card-margin />

        <card
            title="Notifications"
            v-if="!busy"
        >
            <notification
                :notifications="notifications"
            />
        </card>

        <card-margin />

        <card
            title="Title"
            v-if="!busy"
        >
            <template #action>
                <n-button
                    type="info"
                    @click="showAddThresholdEntryModal = true"
                >
                    Add Threshold
                </n-button>
            </template>

            <data-table
                :definitions="thresholdTableDefinition()"
                :entries="thresholds"
            />
        </card>

        <card-margin />

        <action-group
            data-cy="structureUpdateActions"
        >
            <n-button
                @click="$router.back()"
                quaternary
            >
                Back
            </n-button>

            <n-button
                :disabled="creating"
                :loading="creating"
                @click="create"
                type="info"
            >
                Create
            </n-button>
        </action-group>

        <card-margin />

        <add-threshold-modal
            :show="showAddThresholdEntryModal"
            @close="modalCloseEvent"
            @close:value="addThresholdEntryEvent"
        />

        <edit-threshold-modal
            :entry="editThresholdEntry"
            :show="showEditThresholdEntryModal"
            @close="modalCloseEvent"
            @close:value="editThresholdEntryEvent"
            v-if="!busy && editThresholdEntry"
        />
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { h } from 'vue';

import { type Uuid } from '@/sdk/utils';
import { type IStockBlueprintThreshold, StockBlueprintService } from '@/sdk/stockBlueprint';

import { ROUTE_STOCK_BLUEPRINT, ROUTE_STOCK_BLUEPRINTS } from '@/stock/router';

import { NButton, NInput } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import AddThresholdModal from '@/stock/blueprint/modal/AddThreshold.vue';
import Alert from '@/components/Alert.vue';
import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import DataTable, { type IDataTableDefinition } from '@/components/DataTable.vue';
import DeleteObject from '@/components/DeleteObject.vue';
import EditThresholdModal from '@/stock/blueprint/modal/EditThreshold.vue';
import EveIcon from '@/components/EveIcon.vue';
import FormItem from '@/components/FormItem.vue';
import Loader from '@/components/Loader.vue';
import Notification from '@/stock/components/Notification.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NButton,
        NInput,

        ActionGroup,
        AddThresholdModal,
        Alert,
        Card,
        CardMargin,
        DataTable,
        DeleteObject,
        EditThresholdModal,
        FormItem,
        Loader,
        Notification,
        PageHeader,
    }
})
class StockBlueprintCreate extends Vue {
    public busy: boolean = false;
    public creating: boolean = false;
    public errors = {
        creating: false,
    };

    public description: string = '';
    public name: string = '';
    public thresholds: IStockBlueprintThreshold[] = [];
    public editThresholdEntry: IStockBlueprintThreshold = <any>{};
    public notifications: Uuid[] = [];

    public showAddThresholdEntryModal: boolean = false;
    public showEditThresholdEntryModal: boolean = false;

    public async create() {
        let id = '';
        this.creating = true;

        StockBlueprintService
            .create({
                notifications: this.notifications,
                description: this.description,
                name: this.name
            })
            .then(x => {
                x.thresholds = this.thresholds
                id = x.id;
                return x.saveThresholds()
            })
            .then(_ => {
                this.$router.push({
                    name: ROUTE_STOCK_BLUEPRINT,
                    params: {
                        stockBlueprintId: id,
                    }
                })
            })
            .catch(e => {
                this.errors.creating = true;
                this.creating = false;
            });
    }

    public back() {
        this.$router.push({
            name: ROUTE_STOCK_BLUEPRINTS,
        });
    }

    public modalCloseEvent() {
        this.showAddThresholdEntryModal = false;
        this.showEditThresholdEntryModal = false;
    }

    public async addThresholdEntryEvent(threshold: IStockBlueprintThreshold) {
        if (this.thresholds.find(x => x.type_id === threshold.type_id)) {
            this.showAddThresholdEntryModal = false;
            return;
        }

        this.thresholds.push(threshold);
        this.showAddThresholdEntryModal = false;
    }

    public async editThresholdEntryEvent(threshold: IStockBlueprintThreshold) {
        this.showEditThresholdEntryModal = false;

        this.thresholds = this.thresholds
            .map(x => {
                if (x.type_id === threshold.type_id) {
                    return {
                        ...threshold
                    }
                } else {
                    return x;
                }
            });
    }

    private editThreshold(entry: IStockBlueprintThreshold) {
        this.showEditThresholdEntryModal = true;
        this.editThresholdEntry = JSON.parse(JSON.stringify(entry));
    }

    private removeThreshold(index: number) {
        this.thresholds.splice(index, 1);
    }

    public thresholdTableDefinition(): IDataTableDefinition[] {
        return [{
            header: '',
            key: 'icon',
            width: 20,
            render(row) {
                return h(
                    EveIcon,
                    {
                        id: row.type_id,
                        type: 'bp',
                    }
                )
            },
        }, {
            header: 'Item',
            key: 'type_id',
            item: true,
            width: 300,
            visible: true,
            copy: true,
        }, {
            header: 'Want',
            key: 'want',
            number: true,
            width: 100,
            visible: true,
        }, {
            header: 'Critical',
            key: 'critical',
            number: true,
            width: 100,
            visible: true,
        }, {
            header: 'Min Runs',
            key: 'min_runs',
            number: true,
            width: 100,
            visible: true,
        }, {
            header: 'Min ME',
            key: 'min_me',
            number: true,
            width: 100,
            visible: true,
        }, {
            header: 'Min TE',
            key: 'min_te',
            number: true,
            width: 100,
            visible: true,
        }, {
            header: '',
            key: 'id',
            visible: true,
            width: 100,
            render: (row: any, index: number) => {
                return h(
                    "div",
                    [
                        h(
                            NButton,
                            {
                                type: 'info',
                                quaternary: true,
                                onClick: () => this.editThreshold(row)
                            },
                            () => 'Edit'
                        ),
                        h(
                            NButton,
                            {
                                type: 'error',
                                quaternary: true,
                                onClick: () => this.removeThreshold(index)
                            },
                            () => 'Remove'
                        ),
                    ]
                )
            }
        }];
    }
}

export default toNative(StockBlueprintCreate);
</script>
