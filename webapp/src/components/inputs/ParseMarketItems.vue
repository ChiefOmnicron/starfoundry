<template>
    <n-input
        type="textarea"
        rows="10"
        placeholder="Insert your products you want to build, format: ItemName Runs MaterialEfficiency"
        @input="textChange"
    />
</template>

<script lang="ts">
import type { TypeId } from '@/sdk/utils';
import { toNumber } from '@vue/shared';
import { Component, Vue, toNative } from 'vue-facing-decorator';

import { NInput } from 'naive-ui';

@Component({
    components: {
        NInput,
    },
    emits: [
        'update:value',
    ]
})
class ParseMarketItem extends Vue {
    public value: IParsedMarketItem[] = [];

    public textChange(content: string) {
        this.value = [];
        let lines = content.split('\n');
        let internalValues: IParsedMarketItemInternal[] = [];

        for (let line of lines) {
            if (line.startsWith('Total')) {
                continue;
            }

            // tabs are most likely either copied from eve or from excel
            // spaces are most likely manually typed or copied together
            if (line.indexOf('\t') > 0) {
                internalValues.push(this.parseTabs(line));
            } else {
                internalValues.push(this.parseSpaces(line));
            }
        }

        this.$emit('update:Value', this.value);
    }

    public parseTabs(line: string): IParsedMarketItemInternal {
        let split = line.split('\t');

        // legnth 3: name - quantity - price
        // lenght 4: name - quantity - price per - price
        if (split.length === 3) {
            return {
                name: split[0],
                quantity: toNumber(split[1].replaceAll(',', '')),
                price: toNumber(split[2].replaceAll(',', '')),
            }
        } else if (split.length === 4) {
            return {
                name: split[0],
                quantity: toNumber(split[1].replaceAll(',', '')),
                price: toNumber(split[3].replaceAll(',', '')),
            }
        } else {
            return {
                name: 'Invalid',
                quantity: 0,
                price: 0,
            }
        }
    }

    public parseSpaces(line: string): IParsedMarketItemInternal {
        let split = line.split(' ');

        split.pop

        let price = toNumber((split.pop() || '').replaceAll(',', ''));
        let quantity = toNumber((split.pop() || '').replaceAll(',', ''));
        let name = split.join(' ');

        return {
            name,
            quantity,
            price,
        }
    }
}

export default toNative(ParseMarketItem);

export interface IParsedMarketItem {
    name: string;
    quantity: number;
    price: number;
    type_id: TypeId;
}

interface IParsedMarketItemInternal {
    name: string;
    quantity: number;
    price: number;
}
</script>
