<template>
    <div>
      <no-entries
        description="All materials bought"
        size="large"
        style="margin-top: 10px"
        v-if="!busy && Object.keys(recommendations).length === 0"
      />
  
      <card
        v-for="(entry, source) in recommendations"
        :key="source"
        :title="source"
        style="margin-bottom: 10px"
      >
        <template #action>
          <n-button
            :disabled="busy"
            @click="copy(entry.export)"
          >
            Copy
          </n-button>
          <n-button
            :disabled="busy"
            @click="bought(<string>source)"
            type="info"
          >
            Bought
          </n-button>
        </template>
  
        <n-grid :cols="22">
          <n-grid-item  span="10">
            <n-input
              :value="entry.export"
              type="textarea"
              rows="10"
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
              rows="10"
            ></n-input>
          </n-grid-item>
        </n-grid>

        <div style="margin: 5px">
            <b>Total cost: </b>
            <format-number :value="entry.price_total" /> <b>ISK</b>
            <br>
            <b>Total volume: </b>
            <format-number :value="entry.volume_total" /> <b>m3</b>
        </div>
      </card>
    </div>
  </template>
  
  <script lang="ts">
  import { Component, Vue, toNative } from 'vue-facing-decorator';
  import { NButton, NGrid, NGridItem, NIcon, NInput, NSpace } from 'naive-ui';
  import { events } from '@/main';
  import { PROJECT_ROUTE } from '@/event_bus';
  import { ArrowRight } from '@vicons/fa'
  
  import { ItemService } from '@/services/item';
  import { Service, type IUpdateMarketEntry } from '@/project/service';
  
  import Card from '@/components/Card.vue';
  import FormatNumber from '@/components/FormatNumber.vue';
  import NoEntries from '@/components/NoEntries.vue';
  import { toNumber } from '@vue/shared';
  
  @Component({
    components: {
      NButton,
      NGrid,
      NGridItem,
      NIcon,
      NInput,
      NSpace,

      ArrowRight,

      Card,
      FormatNumber,
      NoEntries,
    }
  })
  class ProjectMarketRecommendation extends Vue {
    public busy: boolean = false;
  
    public recommendations: {
        [key: string]:
        {
            export: string,
            import: string,
            source: string,
            price_total: number,
            volume_total: number ,
        }
    } = {};
  
    public async created() {
      events.$emit(
        PROJECT_ROUTE,
        this.$route.name
      );
  
      this.busy = true;
      await this.load();
      this.busy = false;
    }
  
    public async bought(source: string) {
      let item_names = this.recommendations[source].import
        .split('\n')
        .filter((x: string) => !x.startsWith('Total'))
        .filter((x: string) => x !== '')
        .map(x => x.split('\t')[0]);
      let bulk_items = await ItemService.resolve_names_bulk(item_names);
  
      let parsed: IUpdateMarketEntry[] = this.recommendations[source].import
        .split('\n')
        .filter((x: string) => !x.startsWith('Total'))
        .filter((x: string) => x !== '')
        .map(x => {
          let tab_split = x.split('\t');
          let type_id = (bulk_items.find(x => x.name === tab_split[0]) || { type_id: 0 })
          return {
            type_id:    type_id.type_id,
            quantity:   toNumber(tab_split[1].replaceAll(',', '')),
            cost:       toNumber(tab_split[3].replaceAll(',', '')),
            source:     source,
          }
        });
  
      this.busy = true;
      await Service.update_market_gas(<any>this.$route.params.projectId, parsed);
      await this.load();
      this.busy = false;
    }
  
    public copy(content: string) {
      navigator.clipboard.writeText(content);
    }
  
    private async load() {
      this.recommendations = {};
  
      let prices = await Service.fetch_market_recommendation_gas(<any>this.$route.params.projectId);
  
      for(let price of prices) {
        if (!this.recommendations[price.source]) {
          this.recommendations[price.source] = {
            export: '',
            import: '',
            source: price.source,
            price_total: 0,
            volume_total: 0,
          };
        }
  
        this.recommendations[price.source].export += `${price.item_name}\t${price.quantity}\n`;
        this.recommendations[price.source].import += `${price.item_name}\t${price.quantity}\t${price.price}\t${price.quantity * price.price}\n`;
        this.recommendations[price.source].price_total += price.quantity * price.price || 0;
        this.recommendations[price.source].volume_total += price.quantity * price.volume || 0;
      }
    }
  }

export default toNative(ProjectMarketRecommendation);
</script>
  