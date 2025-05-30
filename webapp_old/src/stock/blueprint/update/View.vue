<template>
    <div>
        <page-header
            :title="stock.name"
            data-cy="stockBlueprintUpdateTitle"
            v-if="!busy && !messages.notFound && stock && stock.name"
        />

        <common-messages :message="messages" @click="commonMessagesClose" />

        <loader :busy="busy" data-cy="stockBlueprintUpdateLoading" />

        <card
            data-cy="stockBlueprintUpdateGeneralInformation"
            title="General Information"
            v-if="!busy && !messages.notFound && stock"
        >
            <n-table>
                <tr>
                    <th style="width: 150px">Name</th>
                    <td>
                        {{ stock.name }}
                    </td>
                </tr>
                <tr>
                    <th style="width: 150px">Description</th>
                    <td>
                        {{ stock.description }}
                    </td>
                </tr>
            </n-table>
        </card>

        <card-margin />

        <card title="Notification" v-if="!busy">
            <notification v-model:notifications="stock.notifications" />
        </card>

        <card-margin />

        <card title="Thresholds" v-if="!busy">
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
            v-if="!busy && !messages.notFound && stock"
        >
            <n-button @click="$router.back()" quaternary> Back </n-button>

            <n-button
                :disabled="saving"
                :loading="saving"
                @click="save"
                type="info"
            >
                Save
            </n-button>
        </action-group>

        <card-margin />

        <delete-object
            @delete="deleteObject"
            data-cy="stockBlueprintUpdateDeleteObject"
            object-description="Deleting the stock is not recoverable"
            object-title="Delete Blueprint Stock"
            v-if="!busy && !messages.notFound && stock"
        />

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
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { h } from 'vue';

import type { StockBlueprintId, Uuid } from '@/sdk/utils';
import {
    StockBlueprint,
    StockBlueprintService,
    type IStockBlueprintThreshold,
} from '@/sdk/stockBlueprint';

import { ROUTE_STOCK_BLUEPRINTS } from '@/stock/router';

import { NButton, NTable } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import AddThresholdModal from '@/stock/blueprint/modal/AddThreshold.vue';
import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import CommonMessages, {
    DEFAULT_COMMON_MESSAGES,
    type ICommonMessages,
} from '@/components/CommonMessages.vue';
import DataTable, {
    type IDataTableDefinition,
} from '@/components/DataTable.vue';
import DeleteObject from '@/components/DeleteObject.vue';
import EditThresholdModal from '@/stock/blueprint/modal/EditThreshold.vue';
import EveIcon from '@/components/EveIcon.vue';
import Loader from '@/components/Loader.vue';
import Notification from '@/stock/components/Notification.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NButton,
        NTable,

        ActionGroup,
        AddThresholdModal,
        Card,
        CardMargin,
        CommonMessages,
        DataTable,
        DeleteObject,
        EditThresholdModal,
        Loader,
        Notification,
        PageHeader,
    },
})
class StockBlueprintUpdate extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public stockBlueprintId!: StockBlueprintId;

    public busy: boolean = false;
    public saving: boolean = false;
    public messages: ICommonMessages = DEFAULT_COMMON_MESSAGES();

    public stock!: StockBlueprint;
    public thresholds: IStockBlueprintThreshold[] = [];
    public editThresholdEntry: IStockBlueprintThreshold = <any>{};

    public showAddThresholdEntryModal: boolean = false;
    public showEditThresholdEntryModal: boolean = false;

    public async created() {
        this.busy = true;
        await StockBlueprintService.fetch(this.stockBlueprintId)
            .then((x) => {
                this.stock = x;
                return this.stock.loadThresholds();
            })
            .then((x) => {
                this.thresholds = x;
                this.busy = false;
            })
            .catch((e) => {
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                }

                this.busy = false;
            });
    }

    public async save() {
        this.saving = true;
        this.stock.thresholds = this.thresholds;
        await this.stock.saveThresholds().catch((e) => {
            this.messages.updateError = true;
        });
        await this.stock
            .save()
            .then((_) => {
                this.messages.updateSuccess = true;
                this.saving = false;
            })
            .catch((e) => {
                this.messages.updateError = true;
                this.saving = false;
            });
    }

    public async deleteObject() {
        await this.stock
            .remove()
            .then((_) => {
                this.$router.push({
                    name: ROUTE_STOCK_BLUEPRINTS,
                });
            })
            .catch((_) => (this.messages.deleteError = true));
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
        if (this.thresholds.find((x) => x.type_id === threshold.type_id)) {
            this.showAddThresholdEntryModal = false;
            return;
        }

        this.thresholds.push(threshold);
        this.showAddThresholdEntryModal = false;
    }

    public async editThresholdEntryEvent(threshold: IStockBlueprintThreshold) {
        this.showEditThresholdEntryModal = false;

        this.thresholds = this.thresholds.map((x) => {
            if (x.type_id === threshold.type_id) {
                return {
                    ...threshold,
                };
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
        return [
            {
                header: '',
                key: 'icon',
                width: 20,
                render(row) {
                    return h(EveIcon, {
                        id: row.type_id,
                        type: 'bp',
                    });
                },
            },
            {
                header: 'Item',
                key: 'type_id',
                item: true,
                width: 300,
                visible: true,
                copy: true,
            },
            {
                header: 'Want',
                key: 'want',
                number: true,
                width: 100,
                visible: true,
            },
            {
                header: 'Critical',
                key: 'critical',
                number: true,
                width: 100,
                visible: true,
            },
            {
                header: 'Min Runs',
                key: 'min_runs',
                number: true,
                width: 100,
                visible: true,
            },
            {
                header: 'Min ME',
                key: 'min_me',
                number: true,
                width: 100,
                visible: true,
            },
            {
                header: 'Min TE',
                key: 'min_te',
                number: true,
                width: 100,
                visible: true,
            },
            {
                header: '',
                key: 'id',
                visible: true,
                width: 100,
                render: (row: any, index: number) => {
                    return h('div', [
                        h(
                            NButton,
                            {
                                type: 'info',
                                quaternary: true,
                                onClick: () => this.editThreshold(row),
                            },
                            () => 'Edit',
                        ),
                        h(
                            NButton,
                            {
                                type: 'error',
                                quaternary: true,
                                onClick: () => this.removeThreshold(index),
                            },
                            () => 'Remove',
                        ),
                    ]);
                },
            },
        ];
    }

    public commonMessagesClose() {
        this.messages = DEFAULT_COMMON_MESSAGES();
    }
}

export default toNative(StockBlueprintUpdate);
</script>
