<template>
    <div>
        <n-input-group>
            <n-select
                :disabled="!structureType || readonly"
                :options="options"
                @keydown.enter.prevent
                @update:value="selected"
                filterable
                placeholder="Select Service"
                v-model:value="value"
            />
            <n-button
                :disabled="!value || readonly"
                @click="clear"
                data-cy="clearSelectedService"
                ghost
                type="primary"
            >
                Clear
            </n-button>
        </n-input-group>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative, Prop, Watch } from 'vue-facing-decorator';

import type { TypeId } from '@/sdk/utils';

import {
    NButton,
    NInputGroup,
    NSelect,
    type SelectGroupOption,
} from 'naive-ui';

@Component({
    components: {
        NButton,
        NInputGroup,
        NSelect,
    },
    emits: ['update:value'],
})
class StructureList extends Vue {
    @Prop({
        type: Number,
        required: true,
    })
    public structureType!: any;

    @Prop({
        type: Number,
    })
    public selectedValue!: TypeId;

    @Prop({
        default: true,
        type: Boolean,
    })
    public readonly!: boolean;

    public options: SelectGroupOption[] = [];

    public value: any = undefined;

    public async created() {
        this.options = this.services();

        if (this.selectedValue) {
            this.value = this.selectedValue;
        }
    }

    public clear() {
        this.value = null;
        this.$emit('update:value', null);
    }

    public selected(value: number) {
        this.value = value;
        this.$emit('update:value', this.value);
    }

    public services(): SelectGroupOption[] {
        if (!this.structureType) {
            return [];
        }

        switch (this.structureType) {
            // Raitaru
            case 35825:
                return [
                    {
                        type: 'group',
                        label: 'Citadel Service Modules',
                        key: 'citadel_service_modules',
                        children: [
                            {
                                label: 'Cloning Center I',
                                value: 35894,
                            },
                        ],
                    },
                    {
                        type: 'group',
                        label: 'Engfineering Service Modules',
                        key: 'engineering_service_modules',
                        children: [
                            {
                                label: 'Research Lab I',
                                value: 35891,
                            },
                            {
                                label: 'Hyasyoda Research Lab',
                                value: 45550,
                            },
                            {
                                label: 'Invention Lab I',
                                value: 35886,
                            },
                            {
                                label: 'Manufacturing Plant I',
                                value: 35878,
                            },
                        ],
                    },
                    {
                        type: 'group',
                        label: 'Resource Processing Service Modules',
                        key: 'resource_reprocessing_service_modules',
                        children: [
                            {
                                label: 'Reprocessing Facility I',
                                value: 35899,
                            },
                        ],
                    },
                ];
            // Azbel
            case 35826:
                return [
                    {
                        type: 'group',
                        label: 'Citadel Service Modules',
                        key: 'citadel_service_modules',
                        children: [
                            {
                                label: 'Cloning Center I',
                                value: 35894,
                            },
                            {
                                label: 'Market Hub I',
                                value: 35892,
                            },
                        ],
                    },
                    {
                        type: 'group',
                        label: 'Engfineering Service Modules',
                        key: 'engineering_service_modules',
                        children: [
                            {
                                label: 'Research Lab I',
                                value: 35891,
                            },
                            {
                                label: 'Hyasyoda Research Lab',
                                value: 45550,
                            },
                            {
                                label: 'Invention Lab I',
                                value: 35886,
                            },
                            {
                                label: 'Manufacturing Plant I',
                                value: 35878,
                            },
                            {
                                label: 'Capital Shipyard I',
                                value: 35881,
                            },
                        ],
                    },
                    {
                        type: 'group',
                        label: 'Resource Processing Service Modules',
                        key: 'resource_reprocessing_service_modules',
                        children: [
                            {
                                label: 'Reprocessing Facility I',
                                value: 35899,
                            },
                        ],
                    },
                ];
            // Sotiyo
            case 35827:
                return [
                    {
                        type: 'group',
                        label: 'Citadel Service Modules',
                        key: 'citadel_service_modules',
                        children: [
                            {
                                label: 'Cloning Center I',
                                value: 35894,
                            },
                            {
                                label: 'Market Hub I',
                                value: 35892,
                            },
                        ],
                    },
                    {
                        type: 'group',
                        label: 'Engfineering Service Modules',
                        key: 'engineering_service_modules',
                        children: [
                            {
                                label: 'Research Lab I',
                                value: 35891,
                            },
                            {
                                label: 'Hyasyoda Research Lab',
                                value: 45550,
                            },
                            {
                                label: 'Invention Lab I',
                                value: 35886,
                            },
                            {
                                label: 'Manufacturing Plant I',
                                value: 35878,
                            },
                            {
                                label: 'Capital Shipyard I',
                                value: 35881,
                            },
                            {
                                label: 'Supercapital Shipyard I',
                                value: 35877,
                            },
                        ],
                    },
                    {
                        type: 'group',
                        label: 'Resource Processing Service Modules',
                        key: 'resource_reprocessing_service_modules',
                        children: [
                            {
                                label: 'Reprocessing Facility I',
                                value: 35899,
                            },
                        ],
                    },
                ];
            // Athanor
            case 35835:
                return [
                    {
                        type: 'group',
                        label: 'Citadel Service Modules',
                        key: 'citadel_service_modules',
                        children: [
                            {
                                label: 'Cloning Center I',
                                value: 35894,
                            },
                        ],
                    },
                    {
                        type: 'group',
                        label: 'Engfineering Service Modules',
                        key: 'engineering_service_modules',
                        children: [
                            {
                                label: 'Research Lab I',
                                value: 35891,
                            },
                            {
                                label: 'Hyasyoda Research Lab',
                                value: 45550,
                            },
                            {
                                label: 'Invention Lab I',
                                value: 35886,
                            },
                            {
                                label: 'Manufacturing Plant I',
                                value: 35878,
                            },
                        ],
                    },
                    {
                        type: 'group',
                        label: 'Resource Processing Service Modules',
                        key: 'resource_reprocessing_service_modules',
                        children: [
                            {
                                label: 'Moon Drill I',
                                value: 45009,
                            },
                            {
                                label: 'Reprocessing Facility I',
                                value: 35899,
                            },
                            {
                                label: 'Biochemical Reactor I',
                                value: 45539,
                            },
                            {
                                label: 'Composite Reactor I',
                                value: 45537,
                            },
                            {
                                label: 'Hybrid Reactor I',
                                value: 45538,
                            },
                        ],
                    },
                ];
            // Tatara
            case 35836:
                return [
                    {
                        type: 'group',
                        label: 'Citadel Service Modules',
                        key: 'citadel_service_modules',
                        children: [
                            {
                                label: 'Cloning Center I',
                                value: 35894,
                            },
                            {
                                label: 'Market Hub I',
                                value: 35892,
                            },
                        ],
                    },
                    {
                        type: 'group',
                        label: 'Engfineering Service Modules',
                        key: 'engineering_service_modules',
                        children: [
                            {
                                label: 'Research Lab I',
                                value: 35891,
                            },
                            {
                                label: 'Hyasyoda Research Lab',
                                value: 45550,
                            },
                            {
                                label: 'Invention Lab I',
                                value: 35886,
                            },
                            {
                                label: 'Manufacturing Plant I',
                                value: 35878,
                            },
                        ],
                    },
                    {
                        type: 'group',
                        label: 'Resource Processing Service Modules',
                        key: 'resource_reprocessing_service_modules',
                        children: [
                            {
                                label: 'Moon Drill I',
                                value: 45009,
                            },
                            {
                                label: 'Reprocessing Facility I',
                                value: 35899,
                            },
                            {
                                label: 'Biochemical Reactor I',
                                value: 45539,
                            },
                            {
                                label: 'Composite Reactor I',
                                value: 45537,
                            },
                            {
                                label: 'Hybrid Reactor I',
                                value: 45538,
                            },
                        ],
                    },
                ];
            // Astrahus
            case 35832:
                return [
                    {
                        type: 'group',
                        label: 'Citadel Service Modules',
                        key: 'citadel_service_modules',
                        children: [
                            {
                                label: 'Cloning Center I',
                                value: 35894,
                            },
                        ],
                    },
                    {
                        type: 'group',
                        label: 'Engfineering Service Modules',
                        key: 'engineering_service_modules',
                        children: [
                            {
                                label: 'Research Lab I',
                                value: 35891,
                            },
                            {
                                label: 'Hyasyoda Research Lab',
                                value: 45550,
                            },
                            {
                                label: 'Invention Lab I',
                                value: 35886,
                            },
                            {
                                label: 'Manufacturing Plant I',
                                value: 35878,
                            },
                            {
                                type: 'group',
                                label: 'Resource Processing Service Modules',
                                key: 'resource_reprocessing_service_modules',
                                children: [
                                    {
                                        label: 'Reprocessing Facility I',
                                        value: 35899,
                                    },
                                ],
                            },
                        ],
                    },
                ];
            // Fortizar
            case 35833:
            // 'Prometheus' Fortizar
            case 47516:
            // 'Moreau' Fortizar
            case 47512:
            // 'Marginis' Fortizar
            case 47515:
            // 'Horizon' Fortizar
            case 47514:
            // 'Draccous' Fortizar
            case 47513:
                return [
                    {
                        type: 'group',
                        label: 'Citadel Service Modules',
                        key: 'citadel_service_modules',
                        children: [
                            {
                                label: 'Cloning Center I',
                                value: 35894,
                            },
                            {
                                label: 'Market Hub I',
                                value: 35892,
                            },
                        ],
                    },
                    {
                        type: 'group',
                        label: 'Engfineering Service Modules',
                        key: 'engineering_service_modules',
                        children: [
                            {
                                label: 'Research Lab I',
                                value: 35891,
                            },
                            {
                                label: 'Hyasyoda Research Lab',
                                value: 45550,
                            },
                            {
                                label: 'Invention Lab I',
                                value: 35886,
                            },
                            {
                                label: 'Manufacturing Plant I',
                                value: 35878,
                            },
                            {
                                label: 'Capital Shipyard I',
                                value: 35881,
                            },
                            {
                                type: 'group',
                                label: 'Resource Processing Service Modules',
                                key: 'resource_reprocessing_service_modules',
                                children: [
                                    {
                                        label: 'Reprocessing Facility I',
                                        value: 35899,
                                    },
                                ],
                            },
                        ],
                    },
                ];
            // Keepstar
            case 35834:
            // Upwell Palatine Keepstar
            case 40340:
                return [
                    {
                        type: 'group',
                        label: 'Citadel Service Modules',
                        key: 'citadel_service_modules',
                        children: [
                            {
                                label: 'Cloning Center I',
                                value: 35894,
                            },
                            {
                                label: 'Market Hub I',
                                value: 35892,
                            },
                        ],
                    },
                    {
                        type: 'group',
                        label: 'Engfineering Service Modules',
                        key: 'engineering_service_modules',
                        children: [
                            {
                                label: 'Research Lab I',
                                value: 35891,
                            },
                            {
                                label: 'Hyasyoda Research Lab',
                                value: 45550,
                            },
                            {
                                label: 'Invention Lab I',
                                value: 35886,
                            },
                            {
                                label: 'Manufacturing Plant I',
                                value: 35878,
                            },
                            {
                                label: 'Capital Shipyard I',
                                value: 35881,
                            },
                            {
                                type: 'group',
                                label: 'Resource Processing Service Modules',
                                key: 'resource_reprocessing_service_modules',
                                children: [
                                    {
                                        label: 'Reprocessing Facility I',
                                        value: 35899,
                                    },
                                ],
                            },
                        ],
                    },
                ];
            default:
                return [];
        }
    }
}

export default toNative(StructureList);
</script>
