<template>
    <div>
        <no-entries
            description="All materials bought"
            v-if="!busy && recommendations.length === 0"
        />

        <card
            v-for="entry in recommendations"
            :key="entry.source"
            style="margin-bottom: 10px"
        >
            <template #title>
                <div>
                    <h2 style="margin: 0px">{{ entry.source }}</h2>
                    <small>
                        Last fetch:
                        <format-number
                            date
                            :utc="false"
                            :value="entry.last_fetch"
                        />
                    </small>
                </div>
            </template>

            <template #action>
                <n-button :disabled="busy" @click="copy(entry.export)">
                    Copy
                </n-button>
                <n-button
                    :disabled="busy"
                    @click="bought(<string>entry.source)"
                    type="info"
                >
                    Bought
                </n-button>
            </template>

            <n-grid :cols="22">
                <n-grid-item span="10">
                    <n-input
                        :value="entry.export"
                        type="textarea"
                        disabled
                    ></n-input>
                </n-grid-item>
                <n-grid-item span="2">
                    <n-space vertical align="center" justify="center">
                        <label></label>
                        <h3>
                            <n-icon size="24">
                                <arrow-right />
                            </n-icon>
                        </h3>
                        <label></label>
                    </n-space>
                </n-grid-item>
                <n-grid-item span="10">
                    <n-input
                        v-model:value="entry.import"
                        type="textarea"
                    ></n-input>
                </n-grid-item>
            </n-grid>

            <div style="margin: 5px">
                <b>Total cost: </b>
                <format-number :value="entry.price_total" /> <b>ISK</b>
                <br />
                <b>Total volume: </b>
                <format-number :value="entry.volume_total" /> <b>m3</b>
            </div>
        </card>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { toNumber } from '@vue/shared';
import {
    NButton,
    NEmpty,
    NGrid,
    NGridItem,
    NIcon,
    NInput,
    NSpace,
} from 'naive-ui';
import { events } from '@/main';
import { PROJECT_ROUTE } from '@/event_bus';

import type { Uuid } from '@/sdk/utils';
import { ProjectService } from '@/sdk/project';
import { ItemService } from '@/services/item';
import { Service, type IUpdateMarketEntry } from '@/project/service';

import { ArrowRight } from '@vicons/fa';
import Card from '@/components/Card.vue';
import FormatNumber from '@/components/FormatNumber.vue';
import NoEntries from '@/components/NoEntries.vue';

@Component({
    components: {
        NButton,
        NEmpty,
        NGrid,
        NGridItem,
        NIcon,
        NInput,
        NSpace,

        ArrowRight,

        Card,
        FormatNumber,
        NoEntries,
    },
})
class ProjectMarketRecommendation extends Vue {
    public busy: boolean = false;

    public manual_source: string = 'Manual';
    public manual_export: string = '';
    public manual_import: string = '';

    public recommendations: IRecommendedMarket[] = [];

    public async created() {
        events.$emit(PROJECT_ROUTE, this.$route.name);

        this.busy = true;
        await this.load();
        this.busy = false;
    }

    public async bought(source: string) {
        let item_names = (
            this.recommendations.find((x) => x.source === source) || {
                import: '',
            }
        ).import
            .split('\n')
            .filter((x: string) => !x.startsWith('Total'))
            .filter((x: string) => x !== '')
            .map((x) => x.split('\t')[0]);
        let bulk_items = await ItemService.resolve_names_bulk(item_names);

        let parsed: IUpdateMarketEntry[] = (
            this.recommendations.find((x) => x.source === source) || {
                import: '',
            }
        ).import
            .split('\n')
            .filter((x: string) => !x.startsWith('Total'))
            .filter((x: string) => x !== '')
            .map((x) => {
                let tab_split = x.split('\t');
                let type_id = bulk_items.find(
                    (x) => x.name === tab_split[0],
                ) || { type_id: 0 };
                return {
                    type_id: type_id.type_id,
                    quantity: toNumber(tab_split[1].replaceAll(',', '')),
                    cost: toNumber(tab_split[3].replaceAll(',', '')),
                    source: source,
                };
            });

        await Service.update_market_general(
            <any>this.$route.params.projectId,
            parsed,
        );
        await this.load();
    }

    public copy(content: string) {
        navigator.clipboard.writeText(content);
    }

    public async lastFetch(structureId: Uuid): Promise<string> {
        return await ProjectService.lastMarketFetch(structureId)
            .then((x: string) => {
                return x;
            })
            .catch((_: any) => {
                return '1970-01-01T00:00:00Z';
            });
    }

    private async load() {
        //this.recommendations = [];
        let recommendations: IRecommendedMarket[] = [];

        let prices = await Service.fetch_market_recommendation_general(
            <any>this.$route.params.projectId,
        );

        let lastFetched = new Map();
        for (let price of prices) {
            if (lastFetched.has(price.structure_id)) {
                continue;
            } else {
                let lastFetch = await this.lastFetch(price.structure_id);
                lastFetched.set(
                    price.structure_id,
                    new Date(lastFetch).getTime(),
                );
            }
        }

        for (let price of prices) {
            let recommendation: IRecommendedMarket | undefined =
                recommendations.find((x) => x.source === price.source);
            if (!recommendation) {
                recommendation = {
                    export: '',
                    import: '',
                    source: price.source,
                    last_fetch: lastFetched.get(price.structure_id),
                    price_total: 0,
                    volume_total: 0,
                };
                recommendations.push(recommendation);
            }

            recommendation.export += `${price.item_name}\t${price.quantity}\n`;
            recommendation.import += `${price.item_name}\t${price.quantity}\t${price.price}\t${price.quantity * price.price}\n`;
            recommendation.price_total += price.quantity * price.price || 0;
            recommendation.volume_total += price.quantity * price.volume || 0;
        }

        recommendations = recommendations.sort(
            (a: IRecommendedMarket, b: IRecommendedMarket) =>
                b.source > a.source ? -1 : 1,
        );
        this.recommendations = recommendations;
    }
}

export default toNative(ProjectMarketRecommendation);

interface IRecommendedMarket {
    export: string;
    import: string;
    source: string;
    last_fetch: string;
    price_total: number;
    volume_total: number;
}
</script>
