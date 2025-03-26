<template>
    <div>
        <loader
            :busy="busy"
        />

        <no-entries
            description="No minerals"
            v-if="!busy && minerals.length === 0"
        />

        <!--n-alert
            title="Experimental Feature"
            type="warning"
            v-if="!busy && (compressedOres.compressed || []).length !== 0"
        >
            The feature is currently in an testing phase, the results might not be the best. Validate the data before accepting it!
        </n-alert>

        <card-margin /-->

        <card
            title="Compressed Ore"
            v-if="!busy && minerals.length !== 0"
        >
            <template #action>
                <!--n-tooltip trigger="click">
                    <template #trigger>
                        <n-button
                            @click="copy"
                        >
                            Copy
                        </n-button>
                    </template>
                </n-tooltip-->
                <n-button
                    @click="save"
                    type="info"
                >
                    Save
                </n-button>
            </template>

            <div style="margin: 10px">
                <!--label>Reprocessing Efficiency</label-->

                <!--n-select
                    :options="reprocessingStructureOptions"
                    @update:value="loadCompressedOre"
                    filterable
                    v-model:value="reprocessingStructure"
                /-->
            </div>

            <n-input
                :disabled="busyChangeStructure"
                :loading="busyChangeStructure"
                :rows="10"
                type="textarea"
                v-model:value="compressedOresInput"
            />
        </card>

        <!--card-margin />

        <n-grid
            :cols="3"
            v-if="!busy && (compressedOres.compressed || []).length !== 0"
            x-gap="12"
        >
            <n-grid-item>
                <card
                    style="min-height: 100%"
                    title="project.marketCompressedMineralView.compressedOres"
                >
                    <data-table
                        :definitions="compressedOreTableDefinition()"
                        :entries="compressedOreTableEntries()"
                        no-border
                        style="min-height: 100%"
                    />
                </card>
            </n-grid-item>
            <n-grid-item :span="2">
                <card
                    title="project.marketCompressedMineralView.reprocessedMinerals"
                >
                    <data-table
                        :definitions="mineralsTableDefinition()"
                        :entries="mineralsTableEntries()"
                        no-border
                    />
                </card>
            </n-grid-item>
        </n-grid-->
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { h } from 'vue';

import { ItemService } from '@/sdk/item';
import { ProjectService, type ICompressedOres, type REPROCESSING_EFFICIENCY, Project } from '@/sdk/project';

import type { Uuid } from '@/sdk/utils';

import { NAlert, NButton, NEmpty, NGrid, NGridItem, NInput, NSelect, NTooltip, type SelectGroupOption } from 'naive-ui';

import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import DataTable, { type IDataTableDefinition } from '@/components/DataTable.vue';
import EveIcon from '@/components/EveIcon.vue';
import Loader from '@/components/Loader.vue';
import NoEntries from '@/components/NoEntries.vue';

@Component({
    components: {
        NAlert,
        NButton,
        NEmpty,
        NGrid,
        NGridItem,
        NInput,
        NSelect,
        NTooltip,

        Card,
        CardMargin,
        DataTable,
        Loader,
        NoEntries,
    }
})
class ProjectMarketCompressedMinerals extends Vue {
    public busy: boolean = false;
    public busyChangeStructure: boolean = false;

    public compressedOres: ICompressedOres = <any>{};
    public compressedOresInput: string = '';
    public reprocessingStructure: string = 'NsTataraT2';
    public tableData: any[] = [];
    public minerals: any[] = [];

    private project!: Project;

    public async created() {
        this.busy = true;
        ProjectService
            .fetch(<Uuid>this.$route.params.projectId)
            .then(x => {
                this.project = x;
                this.busy = false;
            })
            .catch(e => {
                console.error(e);
            })
            //.then(_ => this.loadCompressedOre());
            .then(_ => {
                this.minerals = this.project.minerals;
            });
    }

    /*public async loadCompressedOre() {
        console.log(this.project)
        this.busyChangeStructure = true;

        this.project
            .fetchCompressedOre(<REPROCESSING_EFFICIENCY>this.reprocessingStructure)
            .then(x => {
                this.compressedOres = x;

                return ItemService.get_bulk(x.compressed.map(x => x.type_id));
            })
            .then(x => {
                this.compressedOresInput = this.compressedOres
                    .compressed
                    .map(x => `${ItemService.get_sync(x.type_id).name}\t${x.amount * 100}`)
                    .join('\n');

                this.busyChangeStructure = false;
            })
            .catch(_ => {
                loadingError(
                    this.notification,
                    'Load compressed ore error'
                );
            });
    }*/

    public async save() {
        await ItemService
            .parse(this.compressedOresInput)
            // TODO: fix type
            .then((x: any) => {
                console.log(x)
                this.project.updateCompressedOre(x)
            })
            .then(_ => {
            })
            .catch(e => {
                console.error(e);
            });
    }

    public copy() {
        navigator.clipboard.writeText(this.compressedOresInput);
    }

    public compressedOreTableDefinition(): IDataTableDefinition[] {
        return [{
            header: '',
            key: 'icon',
            visible: true,
            width: 40,
            render(row) {
                return h(
                    EveIcon,
                    {
                        id: row.type_id,
                        type: 'icon',
                    }
                )
            },
        }, {
            header: 'project.marketCompressedMineralView.compressedOreTable.name',
            key: 'type_id',
            width: 300,
            item: true,
            visible: true,
        }, {
            header: 'project.marketCompressedMineralView.compressedOreTable.quantity',
            key: 'amount',
            width: 100,
            number: true,
            visible: true,
        }];
    }

    public compressedOreTableEntries() {
        return this.compressedOres.compressed;
    }

    public mineralsTableDefinition(): IDataTableDefinition[] {
        return [{
            header: '',
            key: 'icon',
            visible: true,
            width: 40,
            render(row) {
                return h(
                    EveIcon,
                    {
                        id: row.type_id,
                        type: 'icon',
                    }
                )
            },
        }, {
            header: 'project.marketCompressedMineralView.mineralsTable.name',
            key: 'type_id',
            width: 150,
            item: true,
            visible: true,
        }, {
            header: 'project.marketCompressedMineralView.mineralsTable.quantity',
            key: 'mineralQuantity',
            width: 100,
            number: true,
            visible: true,
        }, {
            header: 'project.marketCompressedMineralView.mineralsTable.overageQuantity',
            key: 'overageQuantity',
            width: 100,
            number: true,
            visible: true,
        }, {
            header: 'project.marketCompressedMineralView.mineralsTable.overagePrice',
            key: 'overagePrice',
            width: 100,
            number: true,
            visible: true,
        }];
    }

    public mineralsTableEntries() {
        return this.compressedOres
            .reprocessed
            .map(x => {
                let overage = this.compressedOres
                    .overage
                    .find(y => y.type_id === x.type_id) || <any>{};
                console.log()

                return {
                    type_id: x.type_id,
                    mineralQuantity: x.amount,
                    overageQuantity: overage.amount,
                    overagePrice: overage.price,
                }
            });
    }

    public reprocessingStructureOptions: SelectGroupOption[] = [{
        type: 'group',
        key: 'hs',
        label: 'Highsec',
        children: [{
            label: 'HS Athanor - No Rig',
            value: 'HsAthanorNoRig',
        }, {
            label: 'HS Athanor - T1',
            value: 'HsAthanorT1',
        }, {
            label: 'HS Athanor - T2',
            value: 'HsAthanorT2',
        }, {
            label: 'HS Tatara - No Rig',
            value: 'HsTataraNoRig',
        }, {
            label: 'HS Tatara - T1',
            value: 'HsTataraT1',
        }, {
            label: 'HS Tatara - T2',
            value: 'HsTataraT2',
        }]
    }, {
        type: 'group',
        key: 'ls',
        label: 'Lowsec',
        children: [{
            label: 'LS Athanor - No Rig',
            value: 'LsAthanorNoRig',
        }, {
            label: 'LS Athanor - T1',
            value: 'LsAthanorT1',
        }, {
            label: 'LS Athanor - T2',
            value: 'LsAthanorT2',
        }, {
            label: 'LS Tatara - No Rig',
            value: 'LsTataraNoRig',
        }, {
            label: 'LS Tatara - T1',
            value: 'LsTataraT1',
        }, {
            label: 'LS Tatara - T2',
            value: 'LsTataraT2',
        }]
    }, {
        type: 'group',
        key: 'ns',
        label: 'Nullsec',
        children: [{
            label: 'NS Athanor - No Rig',
            value: 'NsAthanorNoRig',
        }, {
            label: 'NS Athanor - T1',
            value: 'NsAthanorT1',
        }, {
            label: 'NS Athanor - T2',
            value: 'NsAthanorT2',
        }, {
            label: 'NS Tatara - No Rig',
            value: 'NsTataraNoRig',
        }, {
            label: 'NS Tatara - T1',
            value: 'NsTataraT1',
        }, {
            label: 'NS Tatara - T2',
            value: 'NsTataraT2',
        }]
    }];
}

export default toNative(ProjectMarketCompressedMinerals);
</script>
