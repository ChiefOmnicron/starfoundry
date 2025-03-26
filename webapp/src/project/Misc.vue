<template>
    <wrapper
        :projectId="$route.params.projectId"
        header="Miscellaneous"
    >
        <card v-if="!busy" no-title>
            <p style="margin-left: 10px">
                Miscellaneous expenses, like reprocessing, BPCs, fuel cost, or whatever you can image.

                <br><br>
                Item and price are required, the rest is optional.
            </p>

            <n-table>
                <thead>
                    <tr>
                        <th width="500px">Item</th>
                        <th width="200px">Quantity</th>
                        <th width="500px">Description</th>
                        <th width="200px">Price</th>
                        <th width="100px"></th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="entry in entries" :key="entry.item">
                        <td>
                            {{ entry.item }}
                        </td>
                        <td>
                            <format-number v-if="entry.quantity" :value="entry.quantity" />
                        </td>
                        <td>
                            {{ entry.description }}
                        </td>
                        <td>
                            <format-number v-if="entry.cost" :value="entry.cost" />
                        </td>
                        <td>
                            <n-button @click="remove(entry.id)" style="width: 90px" type="error" ghost>
                                Remove
                            </n-button>
                        </td>
                    </tr>
                </tbody>
                <tfoot>
                    <tr>
                        <td>
                            <n-input v-model:value="new_entry.item" type="text" />
                        </td>
                        <td>
                            <n-input-number v-model:value="new_entry.quantity" />
                        </td>
                        <td>
                            <n-input v-model:value="new_entry.description" type="text" />
                        </td>
                        <td>
                            <n-input-number v-model:value="new_entry.cost" />
                        </td>
                        <td>
                            <n-button
                                @click="add"
                                :disabled="!new_entry.item || !new_entry.cost"
                                style="width: 90px"
                                type="info" ghost
                            >
                                Add
                            </n-button>
                        </td>
                    </tr>
                </tfoot>
            </n-table>
        </card>
    </wrapper>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { ref } from 'vue';
import { NButton, NInput, NInputNumber, NSelect, NTable, type SelectOption } from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';
import { Service, type IMisc } from '@/project/service';

import Wrapper from '@/project/components/Wrapper.vue';

import Card from '@/components/Card.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import { type Uuid } from '@/sdk/utils';

@Component({
    components: {
        NButton,
        NInput,
        NInputNumber,
        NSelect,
        NTable,

        Card,
        FormatNumber,
        Wrapper,
    }
})
class ProjectsView extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public projectId!: Uuid;

    public busy: boolean = false;

    public buildable_items: SelectOption[] = [];
    public entries: IMisc[] = [];

    public new_entry: IMisc = <any>ref(<any>{});

    public async created() {
        events.$emit(
            PROJECT_ROUTE,
            this.$route.name
        );

        this.busy = true;
        await this.load();
        this.busy = false;
    }

    public async add() {
        await Service.add_misc(
            this.projectId,
            this.new_entry
        )
        .then(_ => this.load());

        this.new_entry.item        = <any>null;
        this.new_entry.cost        = <any>null;
        this.new_entry.quantity    = <any>null;
        this.new_entry.description = <any>null;
    }

    public async remove(id: Uuid) {
        await Service.remove_misc(
            this.projectId,
            id,
        )
        .then(_ => this.load())
    }

    private async load() {
        this.entries = await Service
            .fetch_misc(this.projectId)
            .then(x => { return x; })
            .catch(e => {
                console.error(e);
                return [];
            });
    }
}

export default toNative(ProjectsView);
</script>
