<template>
    <div>
        <page-header data-cy="structureCreateTitle" title="Create Structure" />

        <alert
            :visible="errors.resolveStructure"
            @close="errors.resolveStructure = false"
            data-cy="strucutreCreateResolveStructureError"
            description="Could not resolve structure id. Make sure you have docking access"
            title="Error resolving structure"
        />

        <alert
            :visible="errors.unexpected"
            @close="errors.unexpected = false"
            data-cy="strucutreCreateUnexpectedError"
            description="Unexpected error while adding the structure"
            title="Error adding structure"
        />

        <card
            data-cy="strucutreCreateGeneralInformation"
            description="Based on the given Structure ID the Name, System and Structure Type will be automatically determined. This requires that you have docking access to the strucutre."
            title="General Information"
        >
            <div style="margin: 10px">
                <form-item label="EVE Structure ID" info required>
                    <template #help>
                        <resolve-structure-id-info />
                    </template>

                    <resolve-structure
                        @error="(x: boolean) => (errors.resolveStructure = x)"
                        @resolved="setStructureInfo"
                    />
                </form-item>

                <form-item label="Name of the structure">
                    <n-input
                        placeholder="Structure Name"
                        @keydown.enter.prevent
                        disabled
                        v-model:value="structure.name"
                    />
                </form-item>

                <form-item label="Structure Type">
                    <structure-type-selector
                        v-model:value="structure.structure_type_id"
                    />
                </form-item>

                <form-item label="System">
                    <system-selector
                        :default="structure.system_id"
                        disabled
                        v-model:info="systemInfo"
                        v-model:value="structure.system_id"
                    />
                </form-item>

                <form-item label="Group">
                    <project-group-selector
                        :groups="[]"
                        @error="(x: boolean) => (errors.loadProjectGroup = x)"
                        @update:value="
                            (x: any) => (structure.project_group_ids = x)
                        "
                        multiple
                        skip-default
                        structure-permission="WRITE"
                    />
                </form-item>
            </div>
        </card>

        <card-margin />

        <installed-rigs
            :readonly="false"
            :rigs="structure.rigs"
            :structure-type="structure.structure_type_id"
            v-if="structure.structure_type_id"
            @update:value="(x: any[]) => (structure.rigs = x)"
            data-cy="structureCreateInstalledRigs"
        />

        <card-margin />

        <installed-services
            :readonly="false"
            :services="structure.services"
            :structure-type="structure.structure_type_id"
            @update:value="(x: number[]) => (structure.services = x)"
            v-if="structure.structure_type_id"
            data-cy="structureCreateInstalledServices"
        />

        <card-margin />

        <action-group data-cy="structureCreateActions">
            <n-button @click="$router.back()" quaternary> Cancel </n-button>

            <n-button
                :disabled="!structure.name"
                @click="createStructure"
                data-cy="structureCreateButton"
                type="info"
            >
                Create
            </n-button>
        </action-group>
    </div>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';

import { ROUTE_STRUCTURE } from '@/structure/router';

import type { ISystem } from '@/services/system';
import {
    StructureService,
    type IStructure,
    type IStructureResolve,
} from '@/sdk/structure';

import { NButton, NInput, NInputGroup, NInputNumber, NSpace } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import Alert from '@/components/Alert.vue';
import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import FormItem from '@/components/FormItem.vue';
import InstalledRigs from '@/structure/components/InstalledRigs.vue';
import InstalledServices from '@/structure/components/InstalledServices.vue';
import PageHeader from '@/components/PageHeader.vue';
import ProjectGroupSelector from '@/components/selectors/ProjectGroupSelector.vue';
import ResolveStructure from '@/structure/components/ResolveStructure.vue';
import ResolveStructureIdInfo from '@/structure/modals/ResolveStructureIdInfo.vue';
import StructureRigSelector from '@/structure/components/StructureRigSelector.vue';
import StructureServiceSelector from '@/structure/components/StructureServiceSelector.vue';
import StructureTypeSelector from '@/structure/components/StructureTypeSelector.vue';
import SystemSelector from '@/components/selectors/SystemSelector.vue';

@Component({
    components: {
        NButton,
        NInput,
        NInputGroup,
        NInputNumber,
        NSpace,

        ActionGroup,
        Alert,
        Card,
        CardMargin,
        FormItem,
        InstalledRigs,
        InstalledServices,
        PageHeader,
        ProjectGroupSelector,
        ResolveStructure,
        ResolveStructureIdInfo,
        StructureRigSelector,
        StructureServiceSelector,
        StructureTypeSelector,
        SystemSelector,
    },
})
class StructureCreate extends Vue {
    public structure: IStructure = <any>{
        rigs: [],
        services: [],
        project_group_ids: [],
    };
    // information about the system the structure is in, will be filled out
    // when a system is selected
    public systemInfo: ISystem = <any>{};

    public errors = {
        loadProjectGroup: false,
        resolveStructure: false,
        unexpected: false,
    };

    // form validation
    public structureIdStatus: string = '';
    public selected_rig: { name: string; type_id: number } = <any>{};
    public selected_rigs: { name: string; type_id: number }[] = [];

    public createStructure() {
        if (!this.structure.structure_id) {
            this.structureIdStatus = 'error';
            return;
        }

        if (this.systemInfo.security >= 0 && this.systemInfo.security <= 0.5) {
            this.structure.security = 'LOWSEC';
        } else if (this.systemInfo.security > 0.5) {
            this.structure.security = 'HIGHSEC';
        } else {
            this.structure.security = 'NULLSEC';
        }

        StructureService.create(this.structure)
            .then((x) => {
                this.$router.push({
                    name: ROUTE_STRUCTURE,
                    params: {
                        structureId: x.id,
                    },
                });
            })
            .catch((_) => (this.errors.unexpected = true));
    }

    public selectStructureService(index: number, value: number) {
        if (!value) {
            this.structure.services[index] = <any>null;
            return;
        }

        if (!this.structure.services) {
            this.structure.services = [];
        }

        this.structure.services[index] = value;
    }

    public setStructureInfo(resolve: IStructureResolve) {
        this.structure.structure_id = resolve.structure_id;
        this.structure.system_id = resolve.system_id;
        this.structure.name = resolve.name;
        this.structure.structure_type_id = resolve.type_id;
        this.structure.security = resolve.security;
    }
}

export default toNative(StructureCreate);
</script>
