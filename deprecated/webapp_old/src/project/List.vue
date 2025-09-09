<template>
    <div>
        <page-header title="Projects" />

        <div style="margin-bottom: 10px">
            <filter-text
                :filters="filters"
                :load-initial="false"
                :options="filterOptions"
                :search-function="searchFunction"
                v-model:entries="projects"
                style="width: 100%"
                @busy="(s: any) => (busy = s)"
            />

            <filter-element
                style="margin-top: 5px"
                :filters="filters"
                :options="filterOptions"
            />
        </div>

        <loader description="Loading Projects" :busy="busy" />

        <card content-style="padding: 0" v-if="!busy">
            <template #action>
                <n-button
                    @click="$router.push({ name: 'project_create' })"
                    type="info"
                >
                    New project
                </n-button>
            </template>

            <data-table
                :definitions="table_definition()"
                :entries="projects"
                v-if="!busy && projects.length > 0"
            />

            <no-entries
                description="No projects yet"
                v-if="!busy && projects.length === 0"
            />
        </card>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { type VNode, h } from 'vue';
import { events } from '@/main';
import { NAlert, NButton, NCheckbox, NDataTable, NTable, NTag } from 'naive-ui';
import { ProjectService, type IProject } from '@/sdk/project';
import { ROUTE_CHANGE } from '@/event_bus';

import FilterElement from '@/components/FilterElement.vue';
import FilterText, { type IFilterOption } from '@/components/Filter.vue';
import { ROUTE_PROJECT_OVERVIEW } from './router';

import Card from '@/components/Card.vue';
import CharacterInfo from '@/characters/components/CharacterInfo.vue';
import DataTable, {
    type IDataTableDefinition,
} from '@/components/DataTable.vue';
import EveIcon from '@/components/EveIcon.vue';
import Loader from '@/components/Loader.vue';
import NoEntries from '@/components/NoEntries.vue';
import PageHeader from '@/components/PageHeader.vue';
import ProjectProgress from '@/project/components/ProgressBar.vue';
import ProjectStatusTag from '@/project/components/StatusTag.vue';

@Component({
    components: {
        NAlert,
        NButton,
        NCheckbox,
        NDataTable,
        NTable,
        NTag,

        FilterElement,
        FilterText,

        Card,
        CharacterInfo,
        DataTable,
        EveIcon,
        Loader,
        NoEntries,
        PageHeader,
        ProjectProgress,
        ProjectStatusTag,
    },
})
class ProjectsView extends Vue {
    public busy: boolean = false;
    public show_confirm: boolean = false;

    public selected_project: string | undefined = '';

    public projects: IProject[] = [];

    public filters: any = {};
    public filterOptions: { [key: string]: IFilterOption } = this.filter();

    public searchFunction = ProjectService.list;

    public async created() {
        events.$emit(ROUTE_CHANGE, this.$route.name);
    }

    public mounted() {
        this.filters['status'] = ['In Progress'];
    }

    public project_name(): string {
        let info =
            this.projects.find((x) => x.id === this.selected_project) ||
            <any>{ name: '' };
        return info.name;
    }

    private filter(): { [key: string]: IFilterOption } {
        return {
            name: {
                label: 'Name',
            },
            status: {
                label: 'Status',
                multiple: true,
                options: ['Preparing', 'In Progress', 'Done', 'Not Done'],
                preRequest: (_: IFilterOption, val: string[]): string => {
                    const convert_status = (status: string): string => {
                        switch (status) {
                            case 'Preparing':
                                return 'PREPARING';
                            case 'In Progress':
                                return 'IN_PROGRESS';
                            case 'Done':
                                return 'DONE';
                            default:
                                return '';
                        }
                    };

                    return val.map((x) => convert_status(x)).join(',');
                },
            },
        };
    }

    public table_definition(): IDataTableDefinition[] {
        return [
            {
                header: 'Name',
                key: 'name',
                width: 200,
                visible: true,
                routing: {
                    route: ROUTE_PROJECT_OVERVIEW,
                    key: 'projectId',
                    value: 'id',
                },
            },
            {
                header: 'Orderer',
                key: 'orderer',
                width: 200,
                visible: true,
            },
            {
                header: 'Sell Price',
                key: 'price',
                width: 150,
                visible: true,
                number: true,
            },
            {
                header: 'Cost',
                key: 'cost',
                width: 150,
                visible: true,
                number: true,
            },
            {
                header: 'Status',
                key: 'status',
                width: 75,
                visible: true,
                render(row: IProject): VNode {
                    return h(ProjectStatusTag, {
                        status: row.status,
                    });
                },
            },
            {
                header: 'Progress',
                key: 'progress',
                width: 500,
                visible: true,
                render(row: IProject): VNode {
                    return h(ProjectProgress, {
                        projectId: row.id,
                    });
                },
            },
        ];
    }
}

export default toNative(ProjectsView);
</script>
