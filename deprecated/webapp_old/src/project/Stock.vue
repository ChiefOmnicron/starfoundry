<template>
    <div>
        <page-header
            :title="project.name + ' ' + 'Stock'"
            v-if="!initialLoad && !messages.loadingError && project"
        />

        <common-messages :message="messages" @close="commonMessagesClose" />

        <no-entries
            description="No entries found"
            data-cy="structuresNoEntries"
            v-if="stockEntries.length === 0 && !initialLoad"
        />

        <loader :busy="initialLoad" />

        <action-group v-if="stockEntries.length > 0">
            <n-button :disabled="busy" @click="showExportModal = true">
                Export
            </n-button>

            <appraisal-selector
                :busy="busy"
                @select="selectUpdatePrice"
                v-if="project.canWrite"
            />
        </action-group>

        <card no-title v-if="!initialLoad && stockEntries.length > 0">
            <data-table
                :definitions="tableDefinition()"
                :entries="stockEntries"
                v-if="stockEntries.length > 0"
            />
        </card>

        <export
            :data-fields-csv="['type_id', 'quantity', 'cost']"
            :data-fields-ingame="['type_id', 'item_name']"
            :data-fields="['item_name', 'quantity', 'cost']"
            :data="stockEntries"
            :pid="$route.params.pid"
            @close="showExportModal = false"
            v-model:show="showExportModal"
        />
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { type Uuid } from '@/sdk/utils';
import { type IStock, Project, ProjectService } from '@/sdk/project';

import {
    NButton,
    NDropdown,
    NInput,
    NInputNumber,
    NSelect,
    NTable,
} from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import AppraisalSelector from '@/project/components/AppraisalSelector.vue';
import Card from '@/components/Card.vue';
import CommonMessages, {
    DEFAULT_COMMON_MESSAGES,
    type ICommonMessages,
} from '@/components/CommonMessages.vue';
import CopyText from '@/components/CopyText.vue';
import DataTable, {
    type IDataTableDefinition,
} from '@/components/DataTable.vue';
import EveIcon from '@/components/EveIcon.vue';
import Export from '@/components/ItemExportModal.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import Loader from '@/components/Loader.vue';
import NoEntries from '@/components/NoEntries.vue';
import PageHeader from '@/components/PageHeader.vue';

@Component({
    components: {
        NButton,
        NDropdown,
        NInput,
        NInputNumber,
        NSelect,
        NTable,

        ActionGroup,
        AppraisalSelector,
        Card,
        CommonMessages,
        CopyText,
        DataTable,
        EveIcon,
        Export,
        FormatNumber,
        Loader,
        NoEntries,
        PageHeader,
    },
})
class ProjectStockList extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public projectId!: Uuid;

    public busy: boolean = false;
    public initialLoad: boolean = false;
    public messages: ICommonMessages = DEFAULT_COMMON_MESSAGES();

    public showExportModal: boolean = false;

    public project!: Project;
    public stockEntries: IStock[] = [];

    public async created() {
        this.initialLoad = true;

        await ProjectService.fetch(this.projectId)
            .then((x) => {
                this.project = x;
            })
            .then((_) => this.project.fetchPermissionCanWrite())
            .then((_) => this.project.fetchStock())
            .then((groups) => {
                for (let group of groups) {
                    this.stockEntries.push(...group.entries);
                }

                this.initialLoad = false;
            })
            .catch((e) => {
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                } else if (e.response.status === 403) {
                    this.messages.forbidden = true;
                } else {
                    this.messages.loadingError = true;
                }

                this.initialLoad = false;
            });

        this.initialLoad = false;
    }

    public selectUpdatePrice(option: 'INTERNAL' | 'JANICE') {
        this.busy = true;

        this.project
            .updateStockPrices(option)
            .then((groups) => {
                this.busy = false;

                this.stockEntries = [];
                for (let group of groups) {
                    this.stockEntries.push(...group.entries);
                }
            })
            .catch((e) => {
                if (e.response.status === 404) {
                    this.messages.notFound = true;
                } else if (e.response.status === 403) {
                    this.messages.forbidden = true;
                } else {
                    this.messages.updateError = true;
                }

                this.busy = false;
            });
    }

    public tableDefinition(): IDataTableDefinition[] {
        return [
            {
                header: '',
                key: 'type_id',
                icon: 'icon',
                width: 25,
            },
            {
                header: 'Name',
                key: 'type_id',
                width: 400,
                item: true,
                copy: true,
            },
            {
                header: 'Quantity',
                key: 'quantity',
                width: 200,
                number: true,
                copy: true,
            },
            {
                header: 'Cost',
                key: 'cost',
                width: 200,
                copy: true,
                number: true,
                nullable: true,
            },
        ];
    }

    public commonMessagesClose() {
        this.messages = DEFAULT_COMMON_MESSAGES();
    }
}

export default toNative(ProjectStockList);
</script>
