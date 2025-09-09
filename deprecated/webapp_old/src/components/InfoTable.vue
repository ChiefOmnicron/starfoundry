<template>
    <n-table>
        <tr v-for="row in data" :key="row.field">
            <th
                :style="{
                    width: fieldWidth + 'px',
                }"
            >
                {{ row.field }}
            </th>

            <td v-if="row.table" style="padding: 0">
                <data-table
                    :definitions="row.table"
                    :entries="row.value"
                    :striped="false"
                />
            </td>
            <td v-else-if="(row.value || row.value === '') && !row.is_number">
                <copy-text :value="row.value" />
            </td>
            <td
                v-else-if="
                    (<number>row.value >= 0 || <number>row.value <= 0) &&
                    row.is_number
                "
            >
                <copy-text :value="row.value" number />
            </td>
            <td v-else>Not set</td>
        </tr>
    </n-table>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NTable } from 'naive-ui';

import CopyText from '@/components/CopyText.vue';
import DataTable, {
    type IDataTableDefinition,
} from '@/components/DataTable.vue';

@Component({
    components: {
        NTable,

        CopyText,
        DataTable,
    },
})
class InfoTable extends Vue {
    @Prop({
        type: Array,
        required: true,
    })
    public data!: IInfoTable[];

    @Prop({
        type: Number,
        default: 150,
    })
    public fieldWidth!: number;
}

export default toNative(InfoTable);

export interface IInfoTable {
    field: string;
    value: any;
    is_number: boolean;
    table?: IDataTableDefinition[];
}
</script>
