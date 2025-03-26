<template>
  <n-input
    type="textarea"
    placeholder="Fitting or item list"
    v-model:value="items"
    @input="debounce(() => resolve())"
  />
</template>

<script lang="ts">
import { NInput } from 'naive-ui';
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { ItemService } from '@/services/item';

@Component({
  components: {
    NInput
  }
})
class Resolve extends Vue {
    @Prop({
        default: [],
        required: true,
    })
    public modelValue!: Array<any>;

    @Prop({
        default: false
    })
    public buildable!: boolean;

  public items: string         = '';
  public debounce_timeout: any = null;

  public value: IResolve[] = [];

  // Debounces the user input for 500 milliseconds
  // After the debounce the given function is executed
  public debounce(fnc: () => void): void {
    clearTimeout(this.debounce_timeout);
    this.debounce_timeout = setTimeout(() => { fnc() }, 500)
  }

  public async resolve() {
    if (!this.items) { return; }

    let val_idx: Map<string, number>= new Map();
    let splitted = this.items
      .split('\n')
      .filter(x => x !== '');

    for (let split of splitted) {
      let quantity = 1;
      let name = split;

      let header_rgx_match = split.match(/\[([a-bA-Z].*),/);
      if (header_rgx_match) {
        name = header_rgx_match[1];
      }

      let rgx_match = split.match(/ ?x?([0-9]+)/);
      if (rgx_match) {
        quantity = Number(rgx_match[1]);
        name  = name.replace(/ ?x?([0-9]+)/, '');
      }
      name = (name.match(/([a-zA-Z]+ ?)/g) || ['Unknown']).join('');

      if (val_idx.get(name)) {
        let idx = val_idx.get(name) || 0;
        this.value[idx].quantity += quantity;
      } else {
        let idx = this.value.length;
        val_idx.set(name, idx);
        this.value[idx] = {
          name,
          quantity,
          type_id: 0
        }
      }
    }

    let ids: any[] = await (<any>ItemService).resolve_name_bulk(
      this.value.map(x => x.name),
      //{ is_buildable: this.buildable }
    );

    for (let id of ids) {
      let idx = val_idx.get(id.name) || 0;
      let entry = this.value[idx];
      this.value[idx] = {
        quantity:   entry.quantity,
        name:    entry.name,
        type_id: id.type_id
      };
    }

    this.value = this.value.filter(x => x.type_id !== 0);
    this.$emit('update:modelValue', this.value);
    this.items = '';
  }
}

export interface IResolve {
  name:     string;
  quantity: number;
  type_id:  number;
}

export default toNative(Resolve);
</script>
