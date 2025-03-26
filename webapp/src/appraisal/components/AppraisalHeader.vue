<template>
    <n-table style="margin-bottom: 10px">
        <tr>
            <th style="width: 20%">Created at</th>
            <td style="width: 30%; text-align: left;">
                <format-number :value="appraisal?.created_at" :utc="false" date /> (local)
            </td>
            <th style="width: 20%">Buy</th>
            <td style="width: 30%; text-align: left;">
                <copy-text
                    :value="totalBuy()"
                    number
                    with-comma
                /> ISK
            </td>
        </tr>
        <tr>
            <th>Market</th>
            <td style="text-align: left">
                <appraisal-market
                    :market-id="appraisal.market_id"
                />
            </td>
            <th>Split</th>
            <td style="text-align: left">
                <copy-text
                    :value="totalSplit()"
                    number
                    with-comma
                /> ISK
            </td>
        </tr>
        <tr>
            <th>Volume</th>
            <td style="text-align: left">
                <copy-text
                    :value="totalVolume()"
                    number
                    with-comma
                /> m3
            </td>
            <th>Sell</th>
            <td style="text-align: left">
                <copy-text
                    :value="totalSell()"
                    number
                    with-comma
                /> ISK
            </td>
        </tr>
    </n-table>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import type { IAppraisal } from '@/appraisal/service';

import { NTable } from 'naive-ui';
import AppraisalMarket from './AppraisalMarket.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import CopyText from '@/components/CopyText.vue';

@Component({
    components: {
        NTable,

        AppraisalMarket,
        CopyText,
        FormatNumber,
    }
})
class AppraisalHeader extends Vue {
    @Prop({
        required: true
    })
    public appraisal!: IAppraisal;

    public totalBuy(): number {
        if (!this.appraisal) {
            return 0;
        }

        let total = 0;
        for (let item of this.appraisal.items) {
            total += item.buy.max * item.quantity;
        }
        return total;
    }

    public totalSplit(): number {
        return (this.totalBuy() + this.totalSell()) / 2;
    }

    public totalSell(): number {
        if (!this.appraisal) {
            return 0;
        }

        let total = 0;
        for (let item of this.appraisal.items) {
            total += item.sell.min * item.quantity;
        }
        return total;
    }

    public totalVolume(): number {
        if (!this.appraisal) {
            return 0;
        }

        let total = 0;
        for (let item of this.appraisal.items) {
            if (item.meta.repackaged) {
                total += item.meta.repackaged * item.quantity;
            } else {
                total += item.meta.volume * item.quantity;
            }
        }
        return total;
    }
}

export default toNative(AppraisalHeader);
</script>
