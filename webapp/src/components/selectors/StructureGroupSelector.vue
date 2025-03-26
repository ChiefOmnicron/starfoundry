<template>
    <n-select
        :options="available_options()"
        :loading="loading"
        @clear="options = []"
        @update:value="selectStructureGroup"
        v-model:value="value"
        placeholder="Select structure group"
        clearable
        filterable
    />
</template>

<script lang="ts">
import { Component, Vue, toNative, Prop } from 'vue-facing-decorator';
import { NText, NSelect, type SelectOption } from 'naive-ui';
import { StructureGroup, StructureGroupService } from '@/sdk/structure_group';
import type { StructureId } from '@/sdk/utils';
import { StructureDynamicGroup, StructureDynamicGroupService } from '@/sdk/structure_dynamic_group';

// Usage:
//
// ```html
// <structure-group-selector
//     v-model:value="data.structure_group"
// ></structure-group-selector>
// ```
//
// Returns a UUID of the structure group.
//
// Optional Parameters:
//
// - exclude: Array of UUIDs to exclude
//
@Component({
    components: {
        NText,
        NSelect,
    },
    emits: [
        'no-structures',
        'update:value',
    ],
})
class StructureGroupSelector extends Vue {
    @Prop({
        type: Array<StructureId>,
        default: [],
    })
    public exclude: StructureId[] = [];

    @Prop({
        type: Boolean,
        default: true,
    })
    public includeDynamicGroups!: boolean;

    public options: SelectOption[] = [];
    public loading: boolean = false;

    // Holds the selected system id
    public value: number | null = null;

    public async created() {
        this.loading = true;
        await StructureGroupService
            .all()
            .then((x: StructureGroup[]) => {
                this.loading = false;

                if (x.length > 0) {
                    this.$emit('no-structures', true);
                }

                x.map((x: StructureGroup) => {
                    this.options.push({
                        label: x.name,
                        value: x.structure_group_id,
                    });
                });
            })
            .catch(e => {
                this.loading = false;
                console.error(e);
            });

        if (!this.includeDynamicGroups) {
            return;
        }

        await StructureDynamicGroupService.all()
            .then((x: StructureDynamicGroup[]) => {
                x.map((x: StructureDynamicGroup) => {
                    this.options.push({
                        label: `${x.name} - Dynamic Group`,
                        value: x.id,
                    });
                });
            })
            .then((_: any) => this.loading = false)
            .catch(e => {
                this.loading = false;
                console.error(e);
            });
    }

    public available_options(): SelectOption[] {
        return this.options.filter(x => this.exclude.indexOf(<StructureId>x.value) === -1);
    }

    public selectStructureGroup(value: string) {
        this.$emit('update:value', value);
    }
}

export default toNative(StructureGroupSelector);
</script>
