<template>
    <div style="overflow-x:auto;">
        <n-table
            :striped="striped"
            :bordered="false"
        >
            <thead v-if="!noHeader">
                <tr>
                    <template v-for="definition in definitions">
                        <th
                            v-if="columnIsVisible(definition)"
                            :width="columnWidth(definition)"
                        >
                            {{ definition.header }}
                        </th>
                    </template>
                </tr>
            </thead>
            <tbody>
                <tr v-for="(entry, index) in entries" :key="entry">
                    <template
                        v-for="definition in definitions"
                    >
                        <template
                            v-if="columnIsVisible(definition)"
                        >
                            <td
                                v-if="definition.routing"
                                :width="columnWidth(definition)"
                            >
                                <n-button
                                    type="info"
                                    text
                                >
                                    <router-link :to="{
                                            name: definition.routing.route,
                                            params: routingParams(definition.routing, entry)
                                        }"
                                        style="color: inherit;
                                        text-decoration: none"
                                    >
                                        {{ (definition.transform || transform)(entry[definition.key]) }}
                                    </router-link>
                                </n-button>
                            </td>

                            <td
                                v-else-if="definition.copy"
                                :width="columnWidth(definition)"
                            >
                                <copy-text
                                    :item="definition.item"
                                    :nullable="definition.nullable"
                                    :number="definition.number"
                                    :value="entry[definition.key]"
                                />
                            </td>

                            <td
                                v-else-if="definition.number"
                                :width="columnWidth(definition)"
                            >
                                <format-number :value="entry[definition.key]" />
                            </td>

                            <td
                                v-else-if="definition.icon"
                                :width="columnWidth(definition)"
                            >
                                <eve-icon
                                    :id="entry[definition.key]"
                                    :type="definition.icon"
                                    item
                                />
                            </td>

                            <td
                                v-else-if="definition.item"
                                :width="columnWidth(definition)"
                            >
                                <div v-if="!definition.array">
                                    <item
                                        :type-id="entry[definition.key]"
                                        v-slot="{ item }"
                                    >
                                        {{ item.name }}
                                    </item>
                                </div>
                                <div v-else-if="definition.array">
                                    <template
                                        v-for="arrayEntry in entry[definition.key]"
                                    >
                                        <div>
                                            <item
                                                :type-id="arrayEntry"
                                                v-slot="{ item }"
                                            >
                                                {{ (definition.transform || transform)(item.name) }}
                                            </item>
                                        </div>
                                    </template>
                                </div>
                            </td>

                            <td
                                v-else-if="definition.array"
                                :width="columnWidth(definition)"
                            >
                                <template v-for="arrayEntry in entry[definition.key]">
                                    {{ (definition.transform || transform)(arrayEntry) }}<br>
                                </template>
                            </td>

                            <td
                                v-else-if="definition.render"
                                :width="columnWidth(definition)"
                            >
                                <component :is="definition.render(entry, index)"/>
                            </td>

                            <td
                                v-else
                                :width="columnWidth(definition)"
                            >
                                {{ (definition.transform || transform)(entry[definition.key]) }}
                            </td>
                        </template>
                    </template>
                </tr>
            </tbody>

            <slot name="footer" />
        </n-table>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NTable } from 'naive-ui';
import { type VNode } from 'vue';

import CopyText from '@/components/CopyText.vue';
import EveIcon from './EveIcon.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import Item from '@/components/Item.vue';
import Project from '@/components/Project.vue';

@Component({
    components: {
        NButton,
        NTable,

        CopyText,
        EveIcon,
        FormatNumber,
        Item,
        Project,
    }
})
class DataTable extends Vue {
    @Prop({
        type:     Array<IDataTableDefinition>,
        required: true,
    })
    public definitions!: IDataTableDefinition[];

    @Prop({
        type:     Array<any>,
        required: true
    })
    public entries!: any[];

    @Prop({
        default: true,
        type: Boolean,
        required: false,
    })
    public striped!: boolean;

    @Prop({
        default: false,
        type: Boolean,
        required: false,
    })
    public noHeader!: boolean;

    public routingParams(
        routing: IDefinitionRouting,
        entry:   any[],
    ) {
        let params: { [key: string]: any } = {};
        params[routing.key] = entry[<any>routing.value];
        return params;
    }

    public transform(value: string): string {
        return value;
    }

    public columnIsVisible(definition: IDataTableDefinition): boolean {
        if (definition.visible === undefined) {
            return true;
        } else {
            return definition.visible;
        }
    }

    public columnWidth(definition: IDataTableDefinition): string | undefined {
        if (definition.width) {
            return `${definition.width}px`;
        } else if (definition.widthPercent) {
            return `${definition.widthPercent}%`;
        }

        return undefined;
    }
}

export default toNative(DataTable);

export interface IDataTableDefinition {
    header: string;
    key: string;

    visible?: boolean;
    width?: number;
    widthPercent?: number;
    routing?: IDefinitionRouting;

    array?: boolean;
    // uses the format-number component
    number?: boolean;
    // uses the item component
    item?: boolean;
    // enables copying the text
    copy?: boolean;
    // uses a type_id to display an image
    icon?: 'icon' | 'bp';

    nullable?: boolean;

    render?(row: any, index: number): VNode;
    transform?(value: string | undefined | any): string;
}

export interface IDefinitionRouting {
    route: string;
    key: string;
    value: string;
}
</script>
