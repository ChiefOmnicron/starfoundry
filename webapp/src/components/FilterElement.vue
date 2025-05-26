<template>
    <n-space>
        <template :key="key" v-for="(values, key) in filters">
            <n-tag
                @close="handleClose(key.toString(), entry)"
                closable
                style="height: 25px"
                v-for="entry in options[key].multiple ? values : [values]"
            >
                <n-space class="filter" align="center" size="small">
                    <strong>{{ options[key].label }}: </strong>

                    <item-wrapper
                        :type-id="entry"
                        v-if="options[key].item"
                        v-slot="{ item }"
                    >
                        {{
                            (options[key].transform || transform)(
                                options[key],
                                item.name,
                            )
                        }}
                    </item-wrapper>

                    <div v-else-if="options[key].template">
                        <component v-bind:is="options[key].template(entry)" />
                    </div>

                    <span v-else-if="options[key].transform">
                        {{
                            (options[key].transform || transform)(
                                options[key],
                                entry,
                            )
                        }}
                    </span>

                    <span v-else>{{ entry }}</span>
                </n-space>
            </n-tag>
        </template>

        <template v-if="Object.keys(filters).length > 0">
            <n-button
                quaternary
                size="tiny"
                type="info"
                @click="clearFilters"
            >
                Clear filters
            </n-button>
        </template>
    </n-space>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NSpace, NTag } from 'naive-ui';

import { type IFilterOption } from './Filter.vue';

import ItemWrapper from '@/components/ItemWrapper.vue';

@Component({
    components: {
        NButton,
        NSpace,
        NTag,

        ItemWrapper,
    },
})
class FilterElement extends Vue {
    @Prop({
        type: Object,
        required: true,
    })
    public filters!: any;

    @Prop({
        type: Object,
        required: true,
    })
    public options!: any;

    public handleClose(key: string, entry: any) {
        if (this.filters[key].length > 1) {
            this.filters[key] = this.filters[key].filter((x: any) => x !== entry);
        } else {
            delete this.filters[key];
        }
    }

    public joinValues(values: any): string {
        return values.join(', ');
    }

    public transform(_: IFilterOption, value: string): string {
        return value;
    }

    public clearFilters() {
        for (let key of Object.keys(this.filters)) {
            delete this.filters[key];
        }
    }
}

export default toNative(FilterElement);
</script>
