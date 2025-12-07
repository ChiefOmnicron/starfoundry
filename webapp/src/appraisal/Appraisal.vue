<template>
    <n-grid x-gap="10" :cols="12" style="margin-top: 25px">
        <n-grid-item :span="4">
            <appraisal-input v-model:value="rawAppraisal" />

            <div style="margin-top: 10px">
                <appraisal-market-selector v-model:market="market" />

                <n-flex justify="space-between" style="margin-top: 10px">
                    <n-switch
                        v-model:value="viewMode"
                        v-if="appraisal"
                        @update:value="switchViewMode"
                    >
                        <template #checked> Clean input </template>
                        <template #unchecked> Original input </template>
                    </n-switch>

                    <div v-if="!appraisal"></div>

                    <n-button @click="createAppraisal" type="primary">
                        Create Appraisal
                    </n-button>
                </n-flex>
            </div>
        </n-grid-item>
        <n-grid-item :span="8">
            <div v-if="!code">
                <n-steps vertical status="process">
                    <n-step
                        title="Insert your stuff"
                        description="Tip: you can also insert fits"
                    />
                    <n-step
                        title="Select market"
                        description="Select one of the markets, per default Jita 4-4 is selected"
                    />
                    <n-step
                        title="Get an appraisal"
                        description="Click on 'Create Appraisal' and a new appraisal is generted"
                    />
                    <n-step
                        title="Optional: Share"
                        description="Copy the content of your browsers addressbar and share it"
                    />
                    <n-step
                        title="Optional: Reprocessing"
                        description="After creating your appraisal, you can also generate an appraisal of the reprocessed materials"
                    />
                    <n-step
                        title="Optional: Compression"
                        description="You need to know an optimal compressed ore or gas mix of the inserted minerals / gas, generate an appraisal and the tab 'Compression' will provide you with that information"
                    />
                </n-steps>
            </div>

            <div v-if="code && notFound">
                <n-alert title="Invalid Appraisal" type="error">
                    The appraisal couldn't be found, try another one
                </n-alert>
            </div>

            <div v-if="code && !notFound">
                <n-alert
                    title="Non parsable data"
                    type="error"
                    v-if="appraisal && appraisal.invalid.length > 0"
                >
                    The following items couldn't be parsed:<br />
                    <br />
                    <template
                        v-for="(item, index) in appraisal.invalid"
                        :key="index"
                    >
                        <n-text>{{ item }}</n-text>
                        <br />
                    </template>
                </n-alert>

                <n-alert
                    title="Modified Price"
                    type="warning"
                    v-if="appraisal && appraisal.price_modifier !== 100"
                >
                    The price has an additional modifier of
                    {{ appraisal.price_modifier }}%
                </n-alert>

                <n-tabs
                    type="line"
                    @update:value="tabSwitch"
                    :value="selectedTab"
                >
                    <n-tab-pane name="appraisal" tab="Appraisal">
                        <loader :busy="loadingAppraisal" />

                        <div v-if="!loadingAppraisal">
                            <appraisal-header
                                :appraisal="appraisal"
                                v-if="appraisal && appraisal.items.length > 0"
                            />

                            <card-margin
                                v-if="appraisal && appraisal.invalid.length > 0"
                            />

                            <n-data-table
                                :columns="columns()"
                                :data="appraisal.items"
                                :row-class-name="rowClassName"
                                v-if="appraisal && appraisal.items.length > 0"
                                striped
                            />

                            <no-entries
                                description="No entriees"
                                v-if="
                                    (appraisal &&
                                        appraisal.items.length === 0) ||
                                    !appraisal
                                "
                            />
                        </div>
                    </n-tab-pane>
                    <n-tab-pane name="reprocessing" tab="Reprocessing">
                        <loader :busy="loadingAppraisalReprocessing" />

                        <div v-if="!loadingAppraisalReprocessing">
                            <appraisal-header
                                :appraisal="appraisalReprocessing"
                                v-if="
                                    appraisalReprocessing &&
                                    appraisalReprocessing.items.length > 0
                                "
                            />

                            <appraisal-reprocessing
                                v-model:reprocessing="reprocessingOptions"
                            />

                            <n-space reverse>
                                <n-button
                                    @click="fetchReprocessing()"
                                    type="primary"
                                    :disabled="loading"
                                    :loading="loading"
                                >
                                    Recalculate
                                </n-button>
                            </n-space>

                            <card-margin />

                            <n-data-table
                                :columns="columns()"
                                :data="appraisalReprocessing.items"
                                :row-class-name="rowClassName"
                                v-if="
                                    appraisalReprocessing &&
                                    appraisalReprocessing.items.length > 0
                                "
                                striped
                            />

                            <no-entries
                                description="No entriees"
                                v-if="
                                    (appraisalReprocessing &&
                                        appraisalReprocessing.items.length ===
                                            0) ||
                                    !appraisalReprocessing
                                "
                            />
                        </div>
                    </n-tab-pane>
                    <n-tab-pane name="compression" tab="Compression">
                        <loader :busy="loadingAppraisalCompression" />

                        <div v-if="!loadingAppraisalCompression">
                            <n-alert
                                title="No Solution"
                                type="warning"
                                v-if="appraisal && noSolution"
                            >
                                No compression solution found for your input
                                paramteres. Either reduce the amount you need,
                                or allow for example uncompressed ores that you
                                can compress yourself, or allow for raw
                                minerals.
                            </n-alert>

                            <n-tabs type="line" @update:value="tabSwitch">
                                <n-tab-pane name="compressed" tab="Compressed">
                                    <appraisal-header
                                        :appraisal="
                                            appraisalCompression.compression_appraisal
                                        "
                                        v-if="
                                            appraisalCompression &&
                                            appraisalCompression
                                                .compression_appraisal.items
                                                .length > 0
                                        "
                                    />

                                    <appraisal-compression
                                        v-model:compression="compressionOptions"
                                    />

                                    <n-space reverse>
                                        <n-button
                                            @click="fetchCompression()"
                                            type="primary"
                                            :disabled="loading"
                                            :loading="loading"
                                        >
                                            Recalculate
                                        </n-button>
                                    </n-space>

                                    <card-margin />

                                    <n-data-table
                                        :columns="columns()"
                                        :data="
                                            appraisalCompression
                                                .compression_appraisal.items
                                        "
                                        :row-class-name="rowClassName"
                                        v-if="
                                            appraisalCompression &&
                                            appraisalCompression
                                                .compression_appraisal.items
                                                .length > 0
                                        "
                                        striped
                                    />
                                </n-tab-pane>
                                <n-tab-pane
                                    name="compressedOverage"
                                    tab="Overage"
                                >
                                    <appraisal-header
                                        :appraisal="
                                            appraisalCompression.overage_appraisal
                                        "
                                        v-if="
                                            appraisalCompression &&
                                            appraisalCompression.overage_appraisal &&
                                            appraisalCompression
                                                .overage_appraisal.items
                                                .length > 0
                                        "
                                    />

                                    <appraisal-compression
                                        v-model:compression="compressionOptions"
                                    />

                                    <n-space reverse>
                                        <n-button
                                            @click="fetchCompression()"
                                            type="primary"
                                            :disabled="loading"
                                            :loading="loading"
                                        >
                                            Recalculate
                                        </n-button>
                                    </n-space>

                                    <card-margin />

                                    <n-data-table
                                        :columns="columns()"
                                        :data="
                                            appraisalCompression
                                                .overage_appraisal.items
                                        "
                                        :row-class-name="rowClassName"
                                        v-if="
                                            appraisalCompression &&
                                            appraisalCompression.overage_appraisal &&
                                            appraisalCompression
                                                .overage_appraisal.items
                                                .length > 0
                                        "
                                        striped
                                    />
                                </n-tab-pane>
                            </n-tabs>

                            <no-entries
                                description="No entriees"
                                v-if="
                                    (appraisalCompression &&
                                        appraisalCompression
                                            .compression_appraisal.items
                                            .length === 0) ||
                                    !appraisalCompression
                                "
                            />
                        </div>
                    </n-tab-pane>
                    <n-tab-pane name="additiona" tab="Additional">
                        <n-space vertical v-if="appraisal">
                            <h3>Price Modifier</h3>
                            <n-slider
                                v-model:value="appraisal.price_modifier"
                                :step="1"
                                :min="0"
                                :max="200"
                            />
                            <n-input-number
                                v-model:value="appraisal.price_modifier"
                            />

                            <h3>Comment</h3>
                            <n-input
                                type="textarea"
                                maxlength="1024"
                                show-count
                                v-model:value="appraisal.comment"
                            />

                            <n-space justify="end">
                                <n-button
                                    @click="updateAppraisal"
                                    type="primary"
                                >
                                    Update
                                </n-button>
                            </n-space>
                        </n-space>
                    </n-tab-pane>
                </n-tabs>
            </div>
        </n-grid-item>
    </n-grid>
</template>

<script lang="ts">
import { Component, Prop, Vue, Watch, toNative } from 'vue-facing-decorator';
import { h } from 'vue';

import {
    createAppraisal,
    defaultCompressionOptions,
    defaultReprocessingOptions,
    fetchAppraisal,
    fetchAppraisalCompression,
    fetchAppraisalReprocessing,
    type IAppraisal,
    type IAppraisalItem,
    type ICompressionOptions,
    type ICompressionResult,
    type IReprocessingOptions,
} from '@/appraisal/service';

import {
    ROUTE_APPRAISAL,
    ROUTE_APPRAISAL_COMPRESSION,
    ROUTE_APPRAISAL_REPROCESSING,
} from '@/appraisal/router';

import { ArrowsAltH } from '@vicons/fa';
import {
    NAlert,
    NButton,
    NDataTable,
    NFlex,
    NGrid,
    NGridItem,
    NTabs,
    NTabPane,
    NText,
    NSpace,
    NSteps,
    NStep,
    NSlider,
    NInputNumber,
    NInput,
    NIcon,
    NSwitch,
} from 'naive-ui';
import AppraisalCompression from './components/AppraisalCompression.vue';
import AppraisalHeader from '@/appraisal/components/AppraisalHeader.vue';
import AppraisalInput from '@/appraisal/components/AppraisalInput.vue';
import AppraisalMarketSelector from './components/AppraisalMarketSelector.vue';
import AppraisalReprocessing from './components/AppraisalReprocessing.vue';
import CardMargin from '@/components/CardMargin.vue';
import CopyText from '@/components/CopyText.vue';
import EveIcon from '@/components/EveIcon.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import Loader from '@/components/Loader.vue';
import NoEntries from '@/components/NoEntries.vue';

@Component({
    components: {
        NAlert,
        NButton,
        NDataTable,
        NFlex,
        NGrid,
        NGridItem,
        NInputNumber,
        NSlider,
        NSpace,
        NStep,
        NSteps,
        NSwitch,
        NTabPane,
        NTabs,
        NText,
        NInput,

        ArrowsAltH,

        AppraisalCompression,
        AppraisalHeader,
        AppraisalInput,
        AppraisalMarketSelector,
        AppraisalReprocessing,
        CardMargin,
        CopyText,
        FormatNumber,
        Loader,
        NoEntries,
    },
})
class AppraisalShow extends Vue {
    @Prop({
        required: false,
        type: String,
    })
    public code!: string;

    public loading: boolean = true;
    public loadingAppraisal: boolean = false;
    public loadingAppraisalCompression: boolean = false;
    public loadingAppraisalReprocessing: boolean = false;

    public currentTab:
        | 'appraisal'
        | 'compression'
        | 'compressionOverage'
        | 'reprocessing' = 'appraisal';

    public notFound: boolean = false;
    public noSolution: boolean = false;

    public rawAppraisal: string = '';
    public appraisal: IAppraisal | null = null;
    public appraisalCompression: ICompressionResult | null = null;
    public appraisalReprocessing: IAppraisal | null = null;

    public compressionOptions: ICompressionOptions =
        defaultCompressionOptions();
    public reprocessingOptions: IReprocessingOptions =
        defaultReprocessingOptions();

    public market: number = 60003760;
    public single: boolean = false;

    public selectedTab: string = 'appraisal';

    public viewMode: boolean = false;

    public async created() {
        if (!this.code) {
            return;
        }

        this.loadingAppraisal = true;
        fetchAppraisal(this.code)
            .then((x) => {
                this.appraisal = x;

                //this.rawAppraisal = '';

                if (x.raw && !this.viewMode) {
                    this.rawAppraisal = x.raw;
                } else {
                    x.items.forEach(
                        (i) =>
                            (this.rawAppraisal += `${i.meta.name}\t${i.quantity}\n`),
                    );
                }

                this.market = x.market_id;
                this.loadingAppraisal = false;

                //if (this.$route.name === ROUTE_APPRAISAL_COMPRESSION) {
                //    this.tabSwitch('compression')
                //} else if (this.$route.name === ROUTE_APPRAISAL_REPROCESSING) {
                //    this.tabSwitch('reprocessing');
                //}

                if (this.$route.path.endsWith('compression')) {
                    this.tabSwitch('compression');
                } else if (this.$route.path.endsWith('reprocessing')) {
                    this.tabSwitch('reprocessing');
                }
            })
            .catch((e) => {
                if (e.status === 404) {
                    this.notFound = true;
                }

                // TODO:
                console.error(e);
                this.loadingAppraisal = false;
            });
    }

    public async createAppraisal() {
        createAppraisal(this.rawAppraisal, this.market, '', 100)
            .then((x) => {
                this.$router.push({
                    name: ROUTE_APPRAISAL,
                    params: {
                        code: x.code,
                    },
                });
            })
            .catch((e) => {
                console.error(e);
            });
    }

    public async updateAppraisal() {
        if (!this.appraisal) {
            return;
        }

        createAppraisal(
            this.rawAppraisal,
            this.appraisal.market_id,
            this.appraisal.comment,
            this.appraisal.price_modifier,
        )
            .then((x) => {
                this.$router.push({
                    name: ROUTE_APPRAISAL,
                    params: {
                        code: x.code,
                    },
                });
            })
            .catch((e) => {
                console.error(e);
            });
    }

    public tabSwitch(tab: string) {
        if (this.selectedTab !== tab) {
            let route = `${window.location.origin}/appraisal/${this.code}`;

            if (tab === 'compression') {
                console.log('a');
                route = `${window.location.origin}/appraisal/${this.code}/compression`;
            } else if (tab === 'reprocessing') {
                console.log('b');
                route = `${window.location.origin}/appraisal/${this.code}/reprocessing`;
            }

            this.selectedTab = tab;
            window.history.replaceState({ ...history.state }, '', route);
        }

        if (tab === 'compression') {
            this.currentTab = 'compression';

            this.loadingAppraisalCompression = true;
            this.fetchCompression();
        } else if (tab === 'compressedOverage') {
            this.currentTab = 'compressionOverage';
        } else if (tab === 'reprocessing') {
            this.currentTab = 'reprocessing';

            this.loadingAppraisalReprocessing = true;
            this.fetchReprocessing();
        } else {
            this.currentTab = 'appraisal';
        }
    }

    public fetchCompression() {
        this.loading = true;

        fetchAppraisalCompression(this.code, this.compressionOptions)
            .then((x) => {
                this.appraisalCompression = x;
                this.loading = false;
                this.loadingAppraisalCompression = false;
            })
            .catch((e) => {
                if (
                    e.status === 400 &&
                    e.response.data.error === 'NO_SOLUTION'
                ) {
                    this.noSolution = true;
                }

                this.loading = false;
                this.loadingAppraisalCompression = false;
            });
    }

    public fetchReprocessing() {
        this.loading = true;

        fetchAppraisalReprocessing(this.code, this.reprocessingOptions)
            .then((x) => {
                this.appraisalReprocessing = x;
                this.loading = false;
                this.loadingAppraisalReprocessing = false;
            })
            .catch((e) => {
                // TODO:
                console.error(e);
                this.loading = false;
                this.loadingAppraisalReprocessing = false;
            });
    }

    public rowClassName(row: IAppraisalItem): string {
        if (row.low_data) {
            return 'low-data';
        }
        return '';
    }

    public clipboardData() {
        let content = [];

        switch (this.currentTab) {
            case 'appraisal':
                if (this.appraisal) {
                    for (let item of this.appraisal.items) {
                        let volume = 0;
                        if (item.meta.repackaged) {
                            volume = item.meta.repackaged;
                        } else {
                            volume = item.meta.volume;
                        }

                        if (!this.single) {
                            volume *= item.quantity;
                        }

                        let buy = item.buy.max;
                        let sell = item.sell.min;
                        let split = 0;

                        if (this.single) {
                            buy = item.buy.per_item.max;
                            sell = item.sell.per_item.min;
                        }

                        buy = Math.floor(buy * 100) / 100;
                        split = Math.floor((((Math.floor(buy * 100) / 100) + Math.floor(sell * 100) / 100) / 2) * 100) / 100;
                        sell = Math.floor(sell * 100) / 100;

                        content.push(
                            `${item.meta.name}\t${item.quantity}\t${volume}\t${buy}\t${split}\t${sell}`,
                        );
                    }
                }
                return content.join('\n');
            case 'compression':
                if (this.appraisalCompression) {
                    for (let item of this.appraisalCompression
                        .compression_appraisal.items) {
                        let volume = 0;
                        if (item.meta.repackaged) {
                            volume = item.meta.repackaged;
                        } else {
                            volume = item.meta.volume;
                        }

                        if (!this.single) {
                            volume *= item.quantity;
                        }

                        let buy = item.buy.max;
                        let sell = item.sell.min;
                        let split = 0;

                        if (this.single) {
                            buy = item.buy.per_item.max;
                            sell = item.sell.per_item.min;
                        }

                        buy = Math.floor(buy * 100) / 100;
                        split = Math.floor((((Math.floor(buy * 100) / 100) + Math.floor(sell * 100) / 100) / 2) * 100) / 100;
                        sell = Math.floor(sell * 100) / 100;

                        content.push(
                            `${item.meta.name}\t${item.quantity}\t${volume}\t${buy}\t${split}\t${sell}`,
                        );
                    }
                }
                return content.join('\n');
            case 'compressionOverage':
                if (
                    this.appraisalCompression &&
                    this.appraisalCompression.overage_appraisal
                ) {
                    for (let item of this.appraisalCompression.overage_appraisal
                        .items) {
                        let volume = 0;
                        if (item.meta.repackaged) {
                            volume = item.meta.repackaged;
                        } else {
                            volume = item.meta.volume;
                        }

                        if (!this.single) {
                            volume *= item.quantity;
                        }

                        let buy = item.buy.max;
                        let sell = item.sell.min;
                        let split = 0;

                        if (this.single) {
                            buy = item.buy.per_item.max;
                            sell = item.sell.per_item.min;
                        }

                        buy = Math.floor(buy * 100) / 100;
                        split = Math.floor((((Math.floor(buy * 100) / 100) + Math.floor(sell * 100) / 100) / 2) * 100) / 100;
                        sell = Math.floor(sell * 100) / 100;

                        content.push(
                            `${item.meta.name}\t${item.quantity}\t${volume}\t${buy}\t${split}\t${sell}`,
                        );
                    }
                }
                return content.join('\n');
            case 'reprocessing':
                if (this.appraisalReprocessing) {
                    for (let item of this.appraisalReprocessing.items) {
                        let volume = 0;
                        if (item.meta.repackaged) {
                            volume = item.meta.repackaged;
                        } else {
                            volume = item.meta.volume;
                        }

                        if (!this.single) {
                            volume *= item.quantity;
                        }

                        let buy = item.buy.max;
                        let sell = item.sell.min;
                        let split = 0;

                        if (this.single) {
                            buy = item.buy.per_item.max;
                            sell = item.sell.per_item.min;
                        }

                        buy = Math.floor(buy * 100) / 100;
                        split = Math.floor((((Math.floor(buy * 100) / 100) + Math.floor(sell * 100) / 100) / 2) * 100) / 100;
                        sell = Math.floor(sell * 100) / 100;

                        content.push(
                            `${item.meta.name}\t${item.quantity}\t${volume}\t${buy}\t${split}\t${sell}`,
                        );
                    }
                }
                return content.join('\n');
            default:
                return '';
        }
    }

    public columns() {
        return [
            {
                key: 'icon',
                width: 32,
                render: (row: IAppraisalItem) => {
                    return h(EveIcon, {
                        id: row.type_id,
                    });
                },
                title: () => {
                    return h(CopyText, {
                        icon: true,
                        value: this.clipboardData(),
                    });
                },
            },
            {
                title: 'Name',
                key: 'name',
                width: '25%',
                defaultSortOrder: 'ascend',
                sorter: (row1: IAppraisalItem, row2: IAppraisalItem) =>
                    row1.meta.name.localeCompare(row2.meta.name),
                render: (row: IAppraisalItem) => {
                    return h(CopyText, {
                        value: row.meta.name,
                    });
                },
            },
            {
                align: 'right',
                title: 'Quantity',
                key: 'quantity',
                width: '10%',
                sorter: (row1: IAppraisalItem, row2: IAppraisalItem) =>
                    row1.quantity - row2.quantity,
                render: (row: IAppraisalItem) => {
                    return h(CopyText, {
                        value: row.quantity,
                        number: true,
                    });
                },
            },
            {
                key: 'attrs',
                title: () => {
                    return h(
                        NButton,
                        {
                            style: 'width: 100%;',
                            quaternary: true,
                            onClick: () => (this.single = !this.single),
                        },
                        () => [
                            h(
                                'span',
                                {
                                    style: `font-weight: ${!this.single ? 'bold' : 'normal'}; margin-right: 10px`,
                                },
                                'Stack\t',
                            ),
                            h(NIcon, {}, () => [h(ArrowsAltH)]),
                            h(
                                'span',
                                {
                                    style: `font-weight: ${this.single ? 'bold' : 'normal'}; margin-left: 10px`,
                                },
                                '\tSingle',
                            ),
                        ],
                    );
                },
                children: [
                    {
                        align: 'right',
                        title: 'Volume (m3)',
                        key: 'volume',
                        width: '15%',
                        sorter: (
                            row1: IAppraisalItem,
                            row2: IAppraisalItem,
                        ) => {
                            let quantityRow1 = this.single ? 1 : row1.quantity;
                            let quantityRow2 = this.single ? 1 : row2.quantity;

                            if (row1.meta.repackaged && row2.meta.repackaged) {
                                return (
                                    row1.meta.repackaged * quantityRow1 -
                                    row2.meta.repackaged * quantityRow2
                                );
                            } else if (row1.meta.repackaged) {
                                return (
                                    row1.meta.repackaged * quantityRow1 -
                                    row2.meta.volume * quantityRow2
                                );
                            } else if (row2.meta.repackaged) {
                                return (
                                    row1.meta.volume * quantityRow1 -
                                    row2.meta.repackaged * quantityRow2
                                );
                            } else {
                                return (
                                    row1.meta.volume * quantityRow1 -
                                    row2.meta.volume * quantityRow2
                                );
                            }
                        },
                        render: (row: IAppraisalItem) => {
                            let volume = 0;
                            if (row.meta.repackaged) {
                                volume = row.meta.repackaged;
                            } else {
                                volume = row.meta.volume;
                            }

                            if (!this.single) {
                                volume *= row.quantity;
                            }

                            return h(CopyText, {
                                value: volume,
                                number: true,
                                withComma: true,
                            });
                        },
                    },
                    {
                        align: 'right',
                        title: 'Buy (ISK)',
                        key: 'buy',
                        width: '17.5%',
                        sorter: (row1: IAppraisalItem, row2: IAppraisalItem) =>
                            row1.buy.max - row2.buy.max,
                        render: (row: IAppraisalItem) => {
                            let value = row.buy.max;

                            if (this.single) {
                                value = row.buy.per_item.max;
                            }

                            return h(CopyText, {
                                value: value,
                                number: true,
                                withComma: true,
                            });
                        },
                    },
                    {
                        align: 'right',
                        title: 'Split (ISK)',
                        key: 'split',
                        width: '17.5%',
                        sorter: (row1: IAppraisalItem, row2: IAppraisalItem) =>
                            row1.buy.max - row2.buy.max,
                        render: (row: IAppraisalItem) => {
                            let value = (
                                row.buy.max +
                                row.sell.min
                            ) / 2;

                            if (this.single) {
                                value = (
                                    row.buy.per_item.max +
                                    row.sell.per_item.min
                                ) / 2;
                            }

                            return h(CopyText, {
                                value: value,
                                number: true,
                                withComma: true,
                            });
                        },
                    },
                    {
                        align: 'right',
                        title: 'Sell (ISK)',
                        key: 'sell',
                        width: '17.5%',
                        sorter: (row1: IAppraisalItem, row2: IAppraisalItem) =>
                            row1.sell.min - row2.sell.min,
                        render: (row: IAppraisalItem) => {
                            let value = row.sell.min;

                            if (this.single) {
                                value = row.sell.per_item.min;
                            }

                            return h(CopyText, {
                                value: value,
                                number: true,
                                withComma: true,
                            });
                        },
                    },
                ],
            },
        ];
    }

    public switchViewMode() {
        if (!this.appraisal) {
            return;
        }

        this.rawAppraisal = '';
        if (this.appraisal.raw && !this.viewMode) {
            this.rawAppraisal = this.appraisal.raw;
        } else {
            this.appraisal.items.forEach(
                (i) => (this.rawAppraisal += `${i.meta.name}\t${i.quantity}\n`),
            );
        }
    }
}

export default toNative(AppraisalShow);
</script>

<style scoped>
:deep(.low-data td) {
    color: rgb(255, 179, 71) !important;
}
</style>
