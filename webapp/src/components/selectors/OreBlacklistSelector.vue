<template>
    <n-tree-select
        placeholder="Blacklist Ores"
        @update:value="handleBlacklist"
        :options="oreBlacklist()"
        check-strategy="child"
        cascade
        checkable
        clearable
        filterable
        multiple
    />
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NSwitch, NTable, NTreeSelect, type SelectOption } from 'naive-ui';
import FormatNumber from '@/components/FormatNumber.vue';
import CopyText from '@/components/CopyText.vue';
import GasDecompressionSelector from '@/components/selectors/GasDecompressionSelector.vue';
import OreEfficiencySelector from '@/components/selectors/OreEfficiencySelector.vue';

@Component({
    components: {
        NSwitch,
        NTable,
        NTreeSelect,

        CopyText,
        FormatNumber,
        GasDecompressionSelector,
        OreEfficiencySelector,
    },
    emits: ['update:blacklist'],
})
class OreBlacklistSelector extends Vue {
    @Prop({
        type: Array<Number>,
        required: true,
    })
    public blacklist!: number[];

    public handleBlacklist(value: number[]) {
        this.$emit('update:blacklist', value);
    }

    public oreBlacklist(): SelectOption[] {
        return [
            {
                label: 'Highsec',
                key: 'highsec',
                children: [
                    {
                        label: 'Plagioclase',
                        key: 'plagioclase',
                        children: [
                            {
                                label: 'Plagioclase',
                                key: 62528,
                            },
                            {
                                label: 'Plagioclase II-Grade',
                                key: 62529,
                            },
                            {
                                label: 'Plagioclase III-Grade',
                                key: 62530,
                            },
                            {
                                label: 'Plagioclase IV-Grade',
                                key: 62531,
                            },
                        ],
                    },
                    {
                        label: 'Pyroxeres',
                        key: 'pyroxeres',
                        children: [
                            {
                                label: 'Pyroxeres',
                                key: 62524,
                            },
                            {
                                label: 'Pyroxeres II-Grade',
                                key: 62525,
                            },
                            {
                                label: 'Pyroxeres III-Grade',
                                key: 62526,
                            },
                            {
                                label: 'Pyroxeres IV-Grade',
                                key: 62527,
                            },
                        ],
                    },
                    {
                        label: 'Scordite',
                        key: 'scordite',
                        children: [
                            {
                                label: 'Scordite',
                                key: 62520,
                            },
                            {
                                label: 'Scordite II-Grade',
                                key: 62521,
                            },
                            {
                                label: 'Scordite III-Grade',
                                key: 62522,
                            },
                            {
                                label: 'Scordite IV-Grade',
                                key: 62523,
                            },
                        ],
                    },
                    {
                        label: 'Veldspar',
                        key: 'veldspar',
                        children: [
                            {
                                label: 'Veldspar',
                                key: 62516,
                            },
                            {
                                label: 'Veldspar II-Grade',
                                key: 62517,
                            },
                            {
                                label: 'Veldspar III-Grade',
                                key: 62518,
                            },
                            {
                                label: 'Veldspar IV-Grade',
                                key: 62519,
                            },
                        ],
                    },
                ],
            },
            {
                label: 'A0 - 0.5',
                key: 'a0_05',
                children: [
                    {
                        label: 'Ducinium',
                        key: 'ducinium',
                        children: [
                            {
                                label: 'Ducinium',
                                key: 75287,
                            },
                            {
                                label: 'Ducinium II-Grade',
                                key: 75288,
                            },
                            {
                                label: 'Ducinium III-Grade',
                                key: 75289,
                            },
                            {
                                label: 'Ducinium IV-Grade',
                                key: 75290,
                            },
                        ],
                    },
                    {
                        label: 'Eifyrium',
                        key: 'eifyrium',
                        children: [
                            {
                                label: 'Eifyrium',
                                key: 75283,
                            },
                            {
                                label: 'Eifyrium II-Grade',
                                key: 75284,
                            },
                            {
                                label: 'Eifyrium III-Grade',
                                key: 75285,
                            },
                            {
                                label: 'Eifyrium IV-Grade',
                                key: 75286,
                            },
                        ],
                    },
                    {
                        label: 'Mordunium',
                        key: 'mordunium',
                        children: [
                            {
                                label: 'Mordunium',
                                key: 75275,
                            },
                            {
                                label: 'Mordunium II-Grade',
                                key: 75276,
                            },
                            {
                                label: 'Mordunium III-Grade',
                                key: 75277,
                            },
                            {
                                label: 'Mordunium IV-Grade',
                                key: 75278,
                            },
                        ],
                    },
                    {
                        label: 'Ytirium',
                        key: 'ytirium',
                        children: [
                            {
                                label: 'Ytirium',
                                key: 75279,
                            },
                            {
                                label: 'Ytirium II-Grade',
                                key: 75280,
                            },
                            {
                                label: 'Ytirium III-Grade',
                                key: 75281,
                            },
                            {
                                label: 'Ytirium IV-Grade',
                                key: 75282,
                            },
                        ],
                    },
                ],
            },
            {
                label: 'Lowsec',
                key: 'lowsec',
                children: [
                    {
                        label: 'Crokite',
                        key: 'crokite',
                        children: [
                            {
                                label: 'Crokite',
                                key: 62560,
                            },
                            {
                                label: 'Crokite II-Grade',
                                key: 62561,
                            },
                            {
                                label: 'Crokite III-Grade',
                                key: 62562,
                            },
                            {
                                label: 'Crokite IV-Grade',
                                key: 62563,
                            },
                        ],
                    },
                    {
                        label: 'Dark Ochre',
                        key: 'darkochre',
                        children: [
                            {
                                label: 'Dark Ochre',
                                key: 62556,
                            },
                            {
                                label: 'Ochre II-Grade',
                                key: 62557,
                            },
                            {
                                label: 'Ochre III-Grade',
                                key: 62558,
                            },
                            {
                                label: 'Ochre IV-Grade',
                                key: 62559,
                            },
                        ],
                    },
                    {
                        label: 'Gneiss',
                        key: 'gneiss',
                        children: [
                            {
                                label: 'Gneiss',
                                key: 62552,
                            },
                            {
                                label: 'Gneiss II-Grade',
                                key: 62553,
                            },
                            {
                                label: 'Gneiss III-Grade',
                                key: 62554,
                            },
                            {
                                label: 'Gneiss IV-Grade',
                                key: 62555,
                            },
                        ],
                    },
                    {
                        label: 'Hedbergite',
                        key: 'hedbergite',
                        children: [
                            {
                                label: 'Hedbergite',
                                key: 62548,
                            },
                            {
                                label: 'Hedbergite II-Grade',
                                key: 62549,
                            },
                            {
                                label: 'Hedbergite III-Grade',
                                key: 62550,
                            },
                            {
                                label: 'Hedbergite IV-Grade',
                                key: 62551,
                            },
                        ],
                    },
                    {
                        label: 'Hemorphite',
                        key: 'hemorphite',
                        children: [
                            {
                                label: 'Hemorphite',
                                key: 62544,
                            },
                            {
                                label: 'Hemorphite II-Grade',
                                key: 62545,
                            },
                            {
                                label: 'Hemorphite III-Grade',
                                key: 62546,
                            },
                            {
                                label: 'Hemorphite IV-Grade',
                                key: 62547,
                            },
                        ],
                    },
                    {
                        label: 'Jaspet',
                        key: 'jaspet',
                        children: [
                            {
                                label: 'Jaspet',
                                key: 62540,
                            },
                            {
                                label: 'Jaspet II-Grade',
                                key: 62541,
                            },
                            {
                                label: 'Jaspet III-Grade',
                                key: 62542,
                            },
                            {
                                label: 'Jaspet IV-Grade',
                                key: 62543,
                            },
                        ],
                    },
                    {
                        label: 'Kernite',
                        key: 'kernite',
                        children: [
                            {
                                label: 'Kernite',
                                key: 62536,
                            },
                            {
                                label: 'Kernite II-Grade',
                                key: 62537,
                            },
                            {
                                label: 'Kernite III-Grade',
                                key: 62538,
                            },
                            {
                                label: 'Kernite IV-Grade',
                                key: 62539,
                            },
                        ],
                    },
                    {
                        label: 'Omber',
                        key: 'omber',
                        children: [
                            {
                                label: 'Omber',
                                key: 62532,
                            },
                            {
                                label: 'Omber II-Grade',
                                key: 62533,
                            },
                            {
                                label: 'Omber III-Grade',
                                key: 62534,
                            },
                            {
                                label: 'Omber IV-Grade',
                                key: 62535,
                            },
                        ],
                    },
                ],
            },
            {
                label: 'Nullsec',
                key: 'nullsec',
                children: [
                    {
                        label: 'Arkonor',
                        key: 'arkonor',
                        children: [
                            {
                                label: 'Arkonor',
                                key: 62568,
                            },
                            {
                                label: 'Arkonor II-Grade',
                                key: 62569,
                            },
                            {
                                label: 'Arkonor III-Grade',
                                key: 62570,
                            },
                            {
                                label: 'Arkonor IV-Grade',
                                key: 62571,
                            },
                        ],
                    },
                    {
                        label: 'Bistot',
                        key: 'bistot',
                        children: [
                            {
                                label: 'Bistot',
                                key: 62564,
                            },
                            {
                                label: 'Bistot II-Grade',
                                key: 62565,
                            },
                            {
                                label: 'Bistot III-Grade',
                                key: 62566,
                            },
                            {
                                label: 'Bistot IV-Grade',
                                key: 62567,
                            },
                        ],
                    },
                    {
                        label: 'Mercoxit',
                        key: 'mercoxit',
                        children: [
                            {
                                label: 'Mercoxit',
                                key: 62586,
                            },
                            {
                                label: 'Mercoxit II-Grade',
                                key: 62587,
                            },
                            {
                                label: 'Mercoxit III-Grade',
                                key: 62588,
                            },
                        ],
                    },
                ],
            },
            {
                label: 'Sov-Hub',
                key: 'sov_hub',
                children: [
                    {
                        label: 'Griemeer',
                        key: 'griemeer',
                        children: [
                            {
                                label: 'Griemeer',
                                key: 82316,
                            },
                            {
                                label: 'Griemeer II-Grade',
                                key: 82317,
                            },
                            {
                                label: 'Griemeer III-Grade',
                                key: 82318,
                            },
                            {
                                label: 'Griemeer IV-Grade',
                                key: 82319,
                            },
                        ],
                    },
                    {
                        label: 'Hezorime',
                        key: 'hezorime',
                        children: [
                            {
                                label: 'Hezorime',
                                key: 82312,
                            },
                            {
                                label: 'Hezorime II-Grade',
                                key: 82313,
                            },
                            {
                                label: 'Hezorime III-Grade',
                                key: 82314,
                            },
                            {
                                label: 'Hezorime IV-Grade',
                                key: 82315,
                            },
                        ],
                    },
                    {
                        label: 'Kylixium',
                        key: 'kylixium',
                        children: [
                            {
                                label: 'Kylixium',
                                key: 82300,
                            },
                            {
                                label: 'Kylixium II-Grade',
                                key: 82301,
                            },
                            {
                                label: 'Kylixium III-Grade',
                                key: 82302,
                            },
                            {
                                label: 'Kylixium IV-Grade',
                                key: 82303,
                            },
                        ],
                    },
                    {
                        label: 'Nocxite',
                        key: 'nocxite',
                        children: [
                            {
                                label: 'Nocxite',
                                key: 82304,
                            },
                            {
                                label: 'Nocxite II-Grade',
                                key: 82305,
                            },
                            {
                                label: 'Nocxite III-Grade',
                                key: 82306,
                            },
                            {
                                label: 'Nocxite IV-Grade',
                                key: 82307,
                            },
                        ],
                    },
                    {
                        label: 'Ueganite',
                        key: 'ueganite',
                        children: [
                            {
                                label: 'Ueganite',
                                key: 82308,
                            },
                            {
                                label: 'Ueganite II-Grade',
                                key: 82309,
                            },
                            {
                                label: 'Ueganite III-Grade',
                                key: 82310,
                            },
                            {
                                label: 'Ueganite IV-Grade',
                                key: 82311,
                            },
                        ],
                    },
                ],
            },
            {
                label: 'Triglavian',
                key: 'triglavian',
                children: [
                    {
                        label: 'Bezdnacine',
                        key: 'bezdnacine',
                        children: [
                            {
                                label: 'Bezdnacine',
                                key: 62576,
                            },
                            {
                                label: 'Bezdnacine II-Grade',
                                key: 62577,
                            },
                            {
                                label: 'Bezdnacine III-Grade',
                                key: 62578,
                            },
                        ],
                    },
                    {
                        label: 'Rakovene',
                        key: 'rakovene',
                        children: [
                            {
                                label: 'Rakovene',
                                key: 62579,
                            },
                            {
                                label: 'Rakovene II-Grade',
                                key: 62580,
                            },
                            {
                                label: 'Rakovene III-Grade',
                                key: 62581,
                            },
                        ],
                    },
                    {
                        label: 'Spodumain',
                        key: 'spodumain',
                        children: [
                            {
                                label: 'Spodumain',
                                key: 62572,
                            },
                            {
                                label: 'Spodumain II-Grade',
                                key: 62573,
                            },
                            {
                                label: 'Spodumain III-Grade',
                                key: 62574,
                            },
                            {
                                label: 'Spodumain IV-Grade',
                                key: 62575,
                            },
                        ],
                    },
                    {
                        label: 'Talassonite',
                        key: 'talassonite',
                        children: [
                            {
                                label: 'Talassonite',
                                key: 62582,
                            },
                            {
                                label: 'Talassonite II-Grade',
                                key: 62583,
                            },
                            {
                                label: 'Talassonite III-Grade',
                                key: 62584,
                            },
                        ],
                    },
                ],
            },
            {
                label: 'Moon R4',
                key: 'r4',
                children: [
                    {
                        label: 'Bitumens',
                        key: 'bitumens',
                        children: [
                            {
                                label: 'Bitumens',
                                key: 62454,
                            },
                            {
                                label: 'Brimful Bitumens',
                                key: 62455,
                            },
                            {
                                label: 'Glistering Bitumens',
                                key: 62456,
                            },
                        ],
                    },
                    {
                        label: 'Coesite',
                        key: 'coesite',
                        children: [
                            {
                                label: 'Coesite',
                                key: 62457,
                            },
                            {
                                label: 'Brimful Coesite',
                                key: 62458,
                            },
                            {
                                label: 'Glistering Coesite',
                                key: 62459,
                            },
                        ],
                    },
                    {
                        label: 'Sylvite',
                        key: 'sylvite',
                        children: [
                            {
                                label: 'Sylvite',
                                key: 62460,
                            },
                            {
                                label: 'Brimful Sylvite',
                                key: 62461,
                            },
                            {
                                label: 'Glistering Sylvite',
                                key: 62466,
                            },
                        ],
                    },
                    {
                        label: 'Zeolites',
                        key: 'zeolites',
                        children: [
                            {
                                label: 'Zeolites',
                                key: 62463,
                            },
                            {
                                label: 'Brimful Zeolites',
                                key: 62464,
                            },
                            {
                                label: 'Glistering Zeolites',
                                key: 62467,
                            },
                        ],
                    },
                ],
            },
            {
                label: 'Moon R8',
                key: 'r8',
                children: [
                    {
                        label: 'Cobaltite',
                        key: 'cobaltite',
                        children: [
                            {
                                label: 'Cobaltite',
                                key: 62474,
                            },
                            {
                                label: 'Copious Cobaltite',
                                key: 62475,
                            },
                            {
                                label: 'Twinkling Cobaltite',
                                key: 62476,
                            },
                        ],
                    },
                    {
                        label: 'Euxenite',
                        key: 'euxenite',
                        children: [
                            {
                                label: 'Euxenite',
                                key: 62471,
                            },
                            {
                                label: 'Copious Euxenite',
                                key: 62472,
                            },
                            {
                                label: 'Twinkling Euxenite',
                                key: 62473,
                            },
                        ],
                    },
                    {
                        label: 'Scheelite',
                        key: 'scheelite',
                        children: [
                            {
                                label: 'Scheelite',
                                key: 62468,
                            },
                            {
                                label: 'Copious Scheelite',
                                key: 62469,
                            },
                            {
                                label: 'Twinkling Scheelite',
                                key: 62470,
                            },
                        ],
                    },
                    {
                        label: 'Titanite',
                        key: 'titanite',
                        children: [
                            {
                                label: 'Titanite',
                                key: 62477,
                            },
                            {
                                label: 'Copious Titanite',
                                key: 62478,
                            },
                            {
                                label: 'Twinkling Titanite',
                                key: 62479,
                            },
                        ],
                    },
                ],
            },
            {
                label: 'Moon R16',
                key: 'r16',
                children: [
                    {
                        label: 'Chromite',
                        key: 'chromite',
                        children: [
                            {
                                label: 'Chromite',
                                key: 62480,
                            },
                            {
                                label: 'Lavish Chromite',
                                key: 62481,
                            },
                            {
                                label: 'Shimmering Chromite',
                                key: 62482,
                            },
                        ],
                    },
                    {
                        label: 'Otavite',
                        key: 'otavite',
                        children: [
                            {
                                label: 'Otavite',
                                key: 62483,
                            },
                            {
                                label: 'Lavish Otavite',
                                key: 62484,
                            },
                            {
                                label: 'Shimmering Otavite',
                                key: 62485,
                            },
                        ],
                    },
                    {
                        label: 'Sperrylite',
                        key: 'sperrylite',
                        children: [
                            {
                                label: 'Sperrylite',
                                key: 62486,
                            },
                            {
                                label: 'Lavish Sperrylite',
                                key: 62487,
                            },
                            {
                                label: 'Shimmering Sperrylite',
                                key: 62488,
                            },
                        ],
                    },
                    {
                        label: 'Vanadinite',
                        key: 'vanadinite',
                        children: [
                            {
                                label: 'Vanadinite',
                                key: 62489,
                            },
                            {
                                label: 'Lavish Vanadinite',
                                key: 62490,
                            },
                            {
                                label: 'Shimmering Vanadinite',
                                key: 62491,
                            },
                        ],
                    },
                ],
            },
            {
                label: 'Moon R32',
                key: 'r32',
                children: [
                    {
                        label: 'Carnotite',
                        key: 'carnotite',
                        children: [
                            {
                                label: 'Carnotite',
                                key: 62492,
                            },
                            {
                                label: 'Replete Carnotite',
                                key: 62493,
                            },
                            {
                                label: 'Glowing Carnotite',
                                key: 62494,
                            },
                        ],
                    },
                    {
                        label: 'Cinnabar',
                        key: 'cinnabar',
                        children: [
                            {
                                label: 'Cinnabar',
                                key: 62495,
                            },
                            {
                                label: 'Replete Cinnabar',
                                key: 62496,
                            },
                            {
                                label: 'Glowing Cinnabar',
                                key: 62497,
                            },
                        ],
                    },
                    {
                        label: 'Pollucite',
                        key: 'pollucite',
                        children: [
                            {
                                label: 'Pollucite',
                                key: 62498,
                            },
                            {
                                label: 'Replete Pollucite',
                                key: 62499,
                            },
                            {
                                label: 'Glowing Pollucite',
                                key: 62500,
                            },
                        ],
                    },
                    {
                        label: 'Zircon',
                        key: 'zircon',
                        children: [
                            {
                                label: 'Zircon',
                                key: 62501,
                            },
                            {
                                label: 'Replete Zircon',
                                key: 62502,
                            },
                            {
                                label: 'Glowing Zircon',
                                key: 62503,
                            },
                        ],
                    },
                ],
            },
            {
                label: 'Moon R64',
                key: 'r64',
                children: [
                    {
                        label: 'Loparite',
                        key: 'loparite',
                        children: [
                            {
                                label: 'Loparite',
                                key: 62504,
                            },
                            {
                                label: 'Bountiful Loparite',
                                key: 62505,
                            },
                            {
                                label: 'Shining Loparite',
                                key: 62506,
                            },
                        ],
                    },
                    {
                        label: 'Monazite',
                        key: 'monazite',
                        children: [
                            {
                                label: 'Monazite',
                                key: 62507,
                            },
                            {
                                label: 'Bountiful Monazite',
                                key: 62508,
                            },
                            {
                                label: 'Shining Monazite',
                                key: 62509,
                            },
                        ],
                    },
                    {
                        label: 'Xenotime',
                        key: 'xenotime',
                        children: [
                            {
                                label: 'Xenotime',
                                key: 62510,
                            },
                            {
                                label: 'Bountiful Xenotime',
                                key: 62511,
                            },
                            {
                                label: 'Shining Xenotime',
                                key: 62512,
                            },
                        ],
                    },
                    {
                        label: 'Ytterbite',
                        key: 'ytterbite',
                        children: [
                            {
                                label: 'Ytterbite',
                                key: 62513,
                            },
                            {
                                label: 'Bountiful Ytterbite',
                                key: 62514,
                            },
                            {
                                label: 'Shining Ytterbite',
                                key: 62515,
                            },
                        ],
                    },
                ],
            },
        ];
    }
}

export default toNative(OreBlacklistSelector);
</script>
