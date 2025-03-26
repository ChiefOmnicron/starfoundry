<template>
    <n-image
        :src="getImage()"
        :width="width"
    />
</template>

<script lang="ts">
import { NImage } from 'naive-ui';
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

const BASE_URL = 'https://images.evetech.net';

@Component({
  components: {
    NImage
  }
})
class EveIcon extends Vue {
    // either a type_id, character_id, corporation_id or alliance_id
    @Prop({
        type: Number,
        required: true,
    })
    public id!: number;

    // icon, bp, bpc, render
    @Prop({
        type: String,
        default: 'icon',
        required: false
    })
    public type!: string;

    // optional width
    @Prop({
        type: Number,
        default:  32,
        required: false,
    })
    public width!: number;

    @Prop({
        type: Boolean,
        required: false,
    })
    public alliance!: boolean;

    @Prop({
        type: Boolean,
        required: false,
    })
    public character!: boolean;

    @Prop({
        type: Boolean,
        required: false,
    })
    public corporation!: boolean;

    @Prop({
        type: Boolean,
        required: false,
    })
    public blueprint!: boolean;

    public getImage() {
        if (this.alliance) {
            return `${BASE_URL}/alliances/${this.id}/logo?size=1024`;
        } else if (this.character) {
            return `${BASE_URL}/characters/${this.id}/portrait?size=1024`;
        } else if (this.corporation) {
            return `${BASE_URL}/corporations/${this.id}/logo?size=1024`;
        } else {
            return `${BASE_URL}/types/${this.id}/${this.type}?size=1024`;
        }
    }
}

export default toNative(EveIcon);
</script>
