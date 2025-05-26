<template>
    <span v-if="item">
        <slot :item="item"></slot>
    </span>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { ItemService, type IItem } from '@/sdk/item';

@Component({
    components: {},
    emits: ['loading'],
})
class ItemWrapper extends Vue {
    // TypeId of the item to resolve
    @Prop({
        type: Number,
        required: true,
    })
    public typeId!: number;

    public item: IItem = <any>{};

    public async created() {
        this.$emit('loading', true);

        await ItemService.get(this.typeId)
            .then((x) => {
                console.log(x)
                this.item = x;
                this.$emit('loading', false);
            })
            .catch((e) => {
                console.error(e);
            });
    }
}

export default toNative(ItemWrapper);
</script>
