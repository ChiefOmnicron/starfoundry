<template>
  <n-image
    class="owner"
    :src="getAllianceLogo()"
    width="32"
    height="32"
  />
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NImage } from 'naive-ui';

const BASE_URL_CHAR = 'https://images.evetech.net/alliances';

@Component({
  components: {
    NImage
  }
})
class AllianceImage extends Vue {
    @Prop({
        required: true
    })
    public id!: number;

    public name: string = '';

    public getAllianceLogo(): string {
        return `${BASE_URL_CHAR}/${this.id}/logo?size=1024`;
    }
}

export default toNative(AllianceImage);
</script>
