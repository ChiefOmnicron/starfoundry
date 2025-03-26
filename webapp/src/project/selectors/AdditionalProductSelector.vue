<template>
    <n-form
        :model="new_product"
        style="margin-left: 10px; margin-right: 10px; margin-top: 10px"
    >
        <div class="width: 100%; overflow: hidden">
            <n-form-item path="additional_products" label="Additional Products">
                <n-input
                    type="textarea"
                    rows="10"
                    placeholder="Insert your additional products you want to track, format: ItemName Quantity"
                    v-model:value="products"
                    @input="$emit('update:products', products)"
                />
            </n-form-item>
        </div>

        <div class="width: 100%; overflow: hidden">
            <n-form-item path="name" label="Name" style="width: 100%">
                <n-select
                    :options="items"
                    v-model:value="new_product.name"
                    placeholder="Select Item"
                    filterable
                />
            </n-form-item>
        </div>

        <div class="width: 100%; overflow: hidden">
            <n-form-item path="count" label="Quantity" style="width: 89%; float: left">
                <n-input-number
                    v-model:value="new_product.count"
                    style="width: 100%"
                />
            </n-form-item>

            <div style="width: 1%; height: 1px; float: left" />

            <n-form-item label="" style="width: 10%; float: left">
                <n-button
                    @click="add_product"
                    :disabled="!new_product.name"
                    style="width: 100%;"
                    ghost
                >
                    Add
                </n-button>
            </n-form-item>
        </div>
    </n-form>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { NButton, NCard, NForm, NFormItem, NInput, NInputNumber, NSelect, NTable, type SelectOption } from 'naive-ui';
import { ItemService } from '@/services/item';

@Component({
    components: {
        NButton,
        NCard,
        NForm,
        NFormItem,
        NInput,
        NInputNumber,
        NSelect,
        NTable
    },
    emits: [
        'update:products',
    ]
})
class AdditionalProductSelector extends Vue {
    public items: SelectOption[] = [];

    public products: string      = '';
    public new_product: IProduct = this.default_product();

    public async created() {
        this.items = (await ItemService.all()).map(x => {
        return {
            label: x.name,
            value: x.name,
        }
        });
    }

    // Adds the current set values
    public add_product() {
        let entry = `${this.new_product.name} ${this.new_product.count}\n`;

        if (this.products && !this.products.endsWith('\n') && !(this.products === '')) {
            this.products += `\n`;
        }

        this.products += entry;
        this.$emit('update:products', this.products);
        this.new_product = this.default_product();
    }

    public default_product(): IProduct {
        return {
            name:  <any>null,
            count: 1,
        };
    }
}

export default toNative(AdditionalProductSelector);

export interface IProduct {
    name:  string;
    count: number;
}
</script>
