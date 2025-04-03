<template>
    <div>
        <n-table>
            <tbody>
                <structure-wrapper
                    :structure-id="structure"
                    v-slot="{ entry }"
                    v-for="structure in structures"
                    :key="structure"
                >
                    <tr>
                        <td>
                            <structure-reference
                                :structure-id="entry.id"
                                v-if="entry.id"
                            >
                                {{ entry.name }}
                            </structure-reference>
                        </td>
                        <td>
                            <n-button
                                :quaternary="true"
                                @click="deleteStructure(entry.id)"
                                type="error"
                            >
                                Remove
                            </n-button>
                        </td>
                    </tr>
                </structure-wrapper>
            </tbody>

            <tfoot>
                <tr>
                    <td colspan="1" width="93%">
                        <structure-selector
                            :selected-structures="structures"
                            :service="35892"
                            v-model:value="newStructure"
                        />
                    </td>
                    <td width="7%">
                        <n-button
                            :ghost="true"
                            @click="addStructure"
                            style="width: 100%"
                            type="info"
                        >
                            Add
                        </n-button>
                    </td>
                </tr>
            </tfoot>
        </n-table>
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { NButton, NIcon, NTable } from 'naive-ui';
import { ExternalLinkAlt } from '@vicons/fa';
import StructureReference from '@/components/StructureReference.vue';
import StructureSelector from '@/components/selectors/StructureSelector.vue';
import StructureWrapper from '@/components/StructureWrapper.vue';

@Component({
    components: {
        NButton,
        NIcon,
        NTable,

        ExternalLinkAlt,

        StructureSelector,
        StructureReference,
        StructureWrapper,
    },
    emits: [
        'update:structures',
    ]
})
class ProjectGroupDefaultPlayerMarket extends Vue {
    @Prop({
        default: [],
        type: Array,
        required: true,
    })
    public structures!: string[];

    public newStructure: string | null = null;

    public addStructure() {
        if (this.newStructure) {
            this.structures.push(this.newStructure);
        }

        this.newStructure = null;
        this.$emit('update:structures', this.structures);
    }

    public deleteStructure(id: string) {
        let updatedStructures = this.structures.filter(x => x !== id);
        this.newStructure = null;
        this.$emit('update:structures', updatedStructures);
    }
}

export default toNative(ProjectGroupDefaultPlayerMarket);
</script>
