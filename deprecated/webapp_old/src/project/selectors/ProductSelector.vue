<template>
    <n-form
        :model="new_product"
        :rules="rules"
        style="margin-left: 10px; margin-right: 10px; margin-top: 10px"
    >
        <div class="width: 100%; overflow: hidden">
            <n-form-item path="products" label="Products to build">
                <n-input
                    type="textarea"
                    rows="10"
                    placeholder="Insert your products you want to build, format: ItemName Runs MaterialEfficiency"
                    v-model:value="products"
                    @input="$emit('update:products', products)"
                />
            </n-form-item>
        </div>

        <div class="width: 100%; overflow: hidden">
            <n-form-item path="name" label="Name" style="width: 100%">
                <n-select
                    :options="buildable_items"
                    v-model:value="new_product.name"
                    placeholder="Select Item"
                    filterable
                />
                <!-- Replace with
                 <item-selector
                    blueprints
                    v-model:value="newEntry.type_id"
                />-->
            </n-form-item>
        </div>

        <div class="width: 100%; overflow: hidden">
            <n-form-item
                path="count"
                label="Quantity"
                style="width: 44%; float: left"
            >
                <n-input-number
                    :min="1"
                    v-model:value="new_product.count"
                    style="width: 100%"
                />
            </n-form-item>

            <div style="width: 1%; height: 1px; float: left" />

            <n-form-item
                path="material_efficiency"
                label="Material Efficiency"
                style="width: 44%; float: left"
            >
                <n-input-number
                    :min="0"
                    :max="10"
                    v-model:value="new_product.material_efficiency"
                    style="width: 100%"
                />
            </n-form-item>

            <div style="width: 1%; height: 1px; float: left" />

            <n-form-item label="" style="width: 10%; float: left">
                <n-button
                    @click="add_product"
                    :disabled="!new_product.name"
                    style="width: 100%"
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
import {
    FormRules,
    NButton,
    NCard,
    NForm,
    NFormItem,
    NInput,
    NInputNumber,
    NSelect,
    NTable,
    SelectOption,
} from 'naive-ui';
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
        NTable,
    },
    emits: ['update:products'],
})
class ProductSelector extends Vue {
    public buildable_items: SelectOption[] = [];

    public products: string = '';
    public new_product: IProduct = this.default_product();

    public async created() {
        this.buildable_items = (await ItemService.buildable_items()).map(
            (x) => {
                return {
                    label: x.name,
                    value: x.name,
                };
            },
        );
    }

    // Adds the current set values
    public add_product() {
        let entry = `${this.new_product.name} ${this.new_product.count} ${this.new_product.material_efficiency}\n`;

        if (
            this.products &&
            !this.products.endsWith('\n') &&
            !(this.products === '')
        ) {
            this.products += `\n`;
        }

        this.products += entry;
        this.$emit('update:products', this.products);
        this.new_product = this.default_product();
    }

    public default_product(): IProduct {
        return {
            name: <any>null,
            count: 1,
            material_efficiency: 0,
        };
    }

    public rules: FormRules = {
        products: [
            {
                required: true,
                message: 'The field is required',
            },
        ],
    };
}

export interface IProduct {
    name: string;
    count: number;
    material_efficiency: number;
}

export default toNative(ProductSelector);
</script>
