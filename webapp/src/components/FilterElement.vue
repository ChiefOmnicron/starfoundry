<template>
    <n-space>
        <template
            :key="key"
            v-for="(values, key) in filters"
        >
            <n-tag
                v-for="entry in options[key].multiple ? values : [values]"
                @close="handleClose(key.toString())"
                style="height: 25px"
                closable
            >
                <n-space class="filter" align="center" size="small">
                    <strong>{{ options[key].label }}: </strong>

                    <item
                        :type-id="entry"
                        v-if="options[key].item"
                        v-slot="{ item }"
                    >
                        {{ (options[key].transform || transform)(options[key], item.name) }}
                    </item>

                    <div v-else-if="options[key].template">
                        <component v-bind:is="options[key].template(entry)" />
                    </div>

                    <span v-else-if="options[key].transform">
                        {{ (options[key].transform || transform)(options[key], entry) }}
                    </span>

                    <span v-else>{{ entry }}</span>
                </n-space>
            </n-tag>
        </template>
    </n-space>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NSpace, NTag } from 'naive-ui';

import { type IFilterOption } from './Filter.vue';

import Item from '@/components/Item.vue';

@Component({
    components: {
        NSpace,
        NTag,

        Item,
    }
})
class FilterElement extends Vue {
    @Prop({
        type: Object,
        required: true
    })
    public filters!: any;

    @Prop({
        type: Object,
        required: true
    })
    public options!: any;

    public handleClose(key: string) {
        delete this.filters[key];
    }

    public joinValues(values: any): string {
        return values.join(", ");
    }

    public transform(_: IFilterOption, value: string): string {
        return value;
    }
}

export default toNative(FilterElement);
</script>
