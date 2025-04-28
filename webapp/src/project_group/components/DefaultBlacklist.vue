<template>
    <data-table
        :definitions="tableDefinition()"
        :entries="tableEntries()"
        no-header
    >
        <template #footer>
            <tfoot>
                <tr>
                    <td colspan="2" width="93%">
                        <item-selector
                            :selected="blacklist"
                            buildable
                            v-model:value="newBlacklist"
                        />
                    </td>
                    <td width="7%">
                        <n-button
                            :ghost="true"
                            @click="addBlacklist"
                            style="width: 100%"
                            type="info"
                        >
                            Add
                        </n-button>
                    </td>
                </tr>
            </tfoot>
        </template>
    </data-table>
</template>

<script lang="ts">
import { h } from 'vue';
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { TypeId } from '@/sdk/utils';

import { NButton, NTable } from 'naive-ui';
import DataTable, {
    type IDataTableDefinition,
} from '@/components/DataTable.vue';
import EveIcon from '@/components/EveIcon.vue';
import Item from '@/components/Item.vue';
import ItemSelector from '@/components/selectors/ItemSelector.vue';

@Component({
    components: {
        NButton,
        NTable,

        DataTable,
        EveIcon,
        Item,
        ItemSelector,
    },
    emits: ['update:blacklist'],
})
class ProjectGroupDefaultBlacklist extends Vue {
    @Prop({
        default: [],
        type: Array,
        required: true,
    })
    public blacklist!: TypeId[];

    public newBlacklist: TypeId | null = null;

    public addBlacklist() {
        if (this.newBlacklist && this.newBlacklist > 0) {
            this.blacklist.push(this.newBlacklist);
        }

        this.newBlacklist = null;
        this.$emit('update:blacklist', this.blacklist);
    }

    public async deleteBlacklist(id: TypeId, index: number) {
        this.blacklist.splice(index, 1);
        //this.blacklist = this.blacklist.filter(x => x !== id);
        this.$emit('delete', id);
    }

    public tableEntries() {
        return this.blacklist.map((x) => {
            return {
                type_id: x,
            };
        });
    }

    public tableDefinition(): IDataTableDefinition[] {
        return [
            {
                header: '',
                key: 'type_id',
                icon: 'icon',
                widthPercent: 3,
                visible: true,
            },
            {
                header: '',
                key: 'type_id',
                widthPercent: 90,
                visible: true,
                item: true,
                copy: true,
            },
            {
                header: '',
                key: 'id',
                visible: true,
                widthPercent: 7,
                render: (row: any, index: number) => {
                    return h('div', [
                        h(
                            NButton,
                            {
                                type: 'error',
                                quaternary: true,
                                onClick: () =>
                                    this.deleteBlacklist(row.type_id, index),
                            },
                            () => 'Remove',
                        ),
                    ]);
                },
            },
        ];
    }
}

export default toNative(ProjectGroupDefaultBlacklist);
</script>
