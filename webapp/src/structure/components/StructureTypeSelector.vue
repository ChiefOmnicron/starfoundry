<template>
    <n-select
        v-model:value="value"
        :options="options"
        placeholder="Structure Type"
        @keydown.enter.prevent
        disabled
    />
</template>

<script lang="ts">
import { SelectGroupOption } from 'naive-ui';
import { Component, Vue, toNative } from 'vue-facing-decorator';
import { NSelect } from 'naive-ui';

@Component({
    components: {
        NSelect,
    }
})
class StructureList extends Vue {
    public value: any = undefined;

    public options: SelectGroupOption[] = [{
        type: 'group',
        label: 'Engineering Complex',
        key: 'engineering_complex',
        children: [{
            label: 'Raitaru',
            value: 35825
        }, {
            label: 'Azbel',
            value: 35826
        }, {
            label: 'Sotiyo',
            value: 35827
        }]
    }, {
        type: 'group',
        label: 'Refineries',
        key: 'refineries',
        children: [{
            label: 'Athanor',
            value: 35835
        }, {
            label: 'Tatara',
            value: 35836
        }]
    }, {
        type: 'group',
        label: 'Citadels',
        key: 'citadels',
        children: [{
            label: 'Astrahus',
            value: 35832
        }, {
            label: 'Fortizar',
            value: 35833
        }, {
            label: 'Keepstar',
            value: 35834
        },]
    }, {
        type: 'group',
        label: 'Faction Citadels',
        key: 'faction_citadels',
        children: [{
            label: '\'Draccous\' Fortizar',
            value: 47513
        }, {
            label: '\'Horizon\' Fortizar',
            value: 47514
        }, {
            label: '\'Marginis\' Fortizar',
            value: 47515
        }, {
            label: '\'Moreau\' Fortizar',
            value: 47512
        }, {
            label: '\'Prometheus\' Fortizar',
            value: 47516
        }, {
            label: 'Upwell Palatine Keepstar',
            value: 40340
        }]
    }];
}

export default toNative(StructureList);
</script>
