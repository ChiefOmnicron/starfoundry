<template>
  <span>
    <n-spin v-if="busy" />

    <slot
        v-if="!busy"
        :info="corporation"
    ></slot>
  </span>
</template>

<script lang="ts">
import { NSpin } from 'naive-ui';
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { Service, type ICorporationInfo } from '@/characters/service';

@Component({
  components: {
    NSpin,
  }
})
class CorporationInfo extends Vue {
    @Prop({
        required: true,
    })
    public corporationId!: number;

    public busy: boolean                 = false;
    public corporation: ICorporationInfo = <any>{};

    public async created() {
        this.busy = true;

        if (!this.corporationId) {
            return;
        }

        this.corporation = await Service.corporation_info(this.corporationId);
        this.busy = false;
    }
}

export default toNative(CorporationInfo);
</script>
