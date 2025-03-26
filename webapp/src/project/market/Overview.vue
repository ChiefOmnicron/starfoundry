<template>
    <div>
        <common-messages
            :message="messages"
            @close="commonMessagesClose"
        />

        <loader
            :busy="busy"
        />

        <action-group>
            <n-button
                type="info"
                @click="showAddMarketEntryModal = true"
            >
                Add Entry
            </n-button>
        </action-group>

        <no-entries
            description="No market entries"
            v-if="!busy && marketEntries.length === 0"
        />

        <card no-title v-if="!busy && marketEntries.length > 0">
            <data-table
                :definitions="tableDefinition()"
                :entries="tableEntries()"
            />
        </card>

        <add-market-entry-modal
            :show="showAddMarketEntryModal"
            @close="modalCloseEvent"
            @close:value="newMarketEntryEvent"
        />

        <edit-market-entry-modal
            v-if="showEditMarketEntryModal"
            :show="showEditMarketEntryModal"
            :old-entry="updateEntry"
            @close="modalCloseEvent"
            @close:value="editMarketEntryEvent"
        />
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { h } from 'vue';

import { Project, ProjectService, type IMarket } from '@/sdk/project';
import { type ProjectUuid } from '@/sdk/utils';

import { NButton } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import AddMarketEntryModal from '@/project/modal/AddMarketEntry.vue';
import Card from '@/components/Card.vue';
import CommonMessages, { DEFAULT_COMMON_MESSAGES, type ICommonMessages } from '@/components/CommonMessages.vue';
import CopyText from '@/components/CopyText.vue';
import DataTable, { type IDataTableDefinition } from '@/components/DataTable.vue';
import EditMarketEntryModal from '@/project/modal/EditMarketEntry.vue';
import EveIcon from '@/components/EveIcon.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import Loader from '@/components/Loader.vue';
import NoEntries from '@/components/NoEntries.vue';

@Component({
    components: {
        NButton,

        ActionGroup,
        AddMarketEntryModal,
        Card,
        CommonMessages,
        CopyText,
        DataTable,
        EditMarketEntryModal,
        EveIcon,
        FormatNumber,
        Loader,
        NoEntries,
    }
})
class ProjectMarketOverview extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public projectUuid!: ProjectUuid;

    public busy: boolean = false;
    public initialLoad: boolean = false;
    public messages: ICommonMessages = DEFAULT_COMMON_MESSAGES();

    public marketEntries: IMarket[] = [];
    public updateEntry: IMarket = <any>{};

    public showAddMarketEntryModal: boolean = false;
    public showEditMarketEntryModal: boolean = false;

    private project!: Project;

    public async created() {
        await this.load();
    }

    public tableEntries(): IMarket[] {
        return this.marketEntries
            .filter(x => x.quantity > 0)
    }

    public tableDefinition(): IDataTableDefinition[] {
        return [{
            header: '',
            key: 'icon',
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
            header: 'Name',
            key: 'type_id',
            width: 500,
            item: true,
            copy: true,
        }, {
            header: 'Quantity',
            key: 'quantity',
            width: 200,
            number: true,
            copy: true,
        }, {
            header: 'Cost',
            key: 'cost',
            width: 200,
            number: true,
            copy: true,
        }, {
            header: 'Source',
            key: 'source',
            width: 200,
            copy: true,
        }, {
            header: '',
            key: 'edit_delete',
            visible: true,
            width: 125,
            render: (row: any, _: number) => {
                return h(
                    "div",
                    [
                        h(
                            NButton,
                            {
                                type: 'info',
                                quaternary: true,
                                onClick: () => {
                                    this.updateEntry = row;
                                    this.showEditMarketEntryModal = true;
                                }
                            },
                            () => 'Edit'
                        ),
                        h(
                            NButton,
                            {
                                type: 'error',
                                quaternary: true,
                                onClick: () => this.deleteMarketEntry(row)
                            },
                            () => 'Remove'
                        ),
                    ]
                )
            }
        }];
    }

    public modalCloseEvent() {
        this.showAddMarketEntryModal = false;
        this.showEditMarketEntryModal = false;
    }

    public async newMarketEntryEvent(entry: IMarket) {
        await this.project
            .addMarket(entry)
            .then(_ => {
                this.modalCloseEvent();
                return this.load();
            })
            .then(_ => {
                this.messages.createSuccess = true;
            })
            .catch(_ => {
                this.messages.createError = true;
            });
    }

    public async editMarketEntryEvent(entry: IMarket) {
        this.modalCloseEvent();

        await this.project
            .updateMarket(entry.id, entry)
            .then(_ => this.load())
            .then(_ => {
                this.messages.updateSuccess = true;
            })
            .catch(_ => {
                this.messages.updateError = true;
            });
    }

    public async deleteMarketEntry(entry: IMarket) {
        await this.project
            .deleteMarket(entry.id)
            .then(_ => {
                this.load();
            })
            .catch(e => {
                this.messages.deleteError = true;
            })
    }

    public commonMessagesClose() {
        this.messages = DEFAULT_COMMON_MESSAGES();
    }

    private async load() {
        this.initialLoad = true;
        await ProjectService
            .fetch(this.projectUuid)
            .then(x => {
                this.initialLoad = false;
                this.project = x;

                return this.project.fetchMarket()
            })
            .then(x => {
                this.marketEntries = [];
                for (let group of x) {
                    this.marketEntries.push(...group.entries);
                }
            })
            .catch(e => {
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                } else if(e.response.status === 403) {
                    this.messages.forbidden = true;
                } else {
                    this.messages.loadingError = true;
                }

                this.initialLoad = false;
            });
    }
}

export default toNative(ProjectMarketOverview);
</script>
