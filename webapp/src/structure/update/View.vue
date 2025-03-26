<template>
    <div>
        <page-header
            :title="structure.name"
            data-cy="structureUpdateTitle"
            v-if="!busy && !errors.notFound && structure && structure.name"
        />

        <alert
            :visible="errors.notFound"
            @close="errors.notFound = false"
            data-cy="structureUpdateNotFound"
            description="Couldn't find the structure. Try again later."
            title="Structure Not Found"
        />

        <alert
            :visible="errors.updating"
            @close="errors.updating = false"
            data-cy="structureUpdateError"
            description="Error while updating. Try again later."
            title="Update Error"
        />

        <alert
            :visible="errors.deleting"
            @close="errors.deleting = false"
            data-cy="structureDeleteError"
            description="Error while deleting. Try again later."
            title="Delete Error"
        />

        <loader
            :busy=busy
            data-cy="structureUpdateLoading"
        />

        <card
            data-cy="structureUpdateGeneralInformation"
            title="General Information"
            v-if="!busy && !errors.notFound && structure"
        >
            <template #action>
                <n-button @click="resolveStructureName()">
                    Update name
                </n-button>
            </template>

            <n-table>
                <tr>
                    <th style="width: 150px">Nmae</th>
                    <td>
                        {{ structure.name }}
                    </td>
                </tr>
                <tr>
                    <th style="width: 150px">Structure Type</th>
                    <td>
                        <item :type-id="structure.structureTypeId" v-slot="{ item }">
                            {{ item.name }}
                        </item>
                    </td>
                </tr>
                <tr>
                    <th>Location</th>
                    <td>
                        <system v-if="structure.systemId" :dotlan="true" :system-id="structure.systemId" />
                    </td>
                </tr>
            </n-table>
        </card>

        <card-margin />

        <installed-rigs
            :readonly="isReadonly"
            :rigs="structure.rigs"
            :structure-type="structure.structureTypeId"
            @update:value="(x: number[]) => structure.rigs = x"
            data-cy="structureUpdateInstalledRigs"
            v-if="!busy && !errors.notFound && structure && structure.structureTypeId"
        />

        <card-margin />

        <installed-services
            :readonly="isReadonly"
            :services="structure.services"
            :structure-type="structure.structureTypeId"
            @update:value="(x: number[]) => structure.services = x"
            data-cy="structureUpdateInstalledServices"
            v-if="!busy && !errors.notFound && structure && structure.structureTypeId"
        />

        <card-margin />

        <action-group
            data-cy="structureUpdateActions"
            v-if="!busy && !errors.notFound && structure"
        >
            <n-button
                @click="$router.back()"
                quaternary
            >
                Back
            </n-button>

            <n-button
                @click="save"
                data-cy="structureUpdateSave"
                type="info"
            >
                Save
            </n-button>
        </action-group>

        <card-margin />

        <delete-object
            @delete="deleteObject"
            data-cy="structureUpdateDeleteObject"
            object-description="Deleting the structure is not recoverable"
            object-title="Delete Structure"
            v-if="!busy && !errors.notFound && structure && structure.isOwner"
        />
    </div>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';

import { Structure, StructureService } from '@/sdk/structure';
import { type Uuid } from '@/sdk/utils';

import { ROUTE_STRUCTURES } from '@/structure/router';

import { NButton, NList, NListItem, NSpace, NTable } from 'naive-ui';
import ActionGroup from '@/components/ActionGroup.vue';
import Alert from '@/components/Alert.vue';
import Card from '@/components/Card.vue';
import CardMargin from '@/components/CardMargin.vue';
import DeleteObject from '@/components/DeleteObject.vue';
import EveIcon from '@/components/EveIcon.vue';
import InstalledRigs from '@/structure/components/InstalledRigs.vue';
import InstalledServices from '@/structure/components/InstalledServices.vue';
import Item from '@/components/Item.vue';
import Loader from '@/components/Loader.vue';
import PageHeader from '@/components/PageHeader.vue';
import ProjectGroupSelector from '@/components/selectors/ProjectGroupSelector.vue';
import StructureInfo from '@/structure/components/StructureInfo.vue';
import System from '@/components/System.vue';

@Component({
    components: {
        NButton,
        NList,
        NListItem,
        NSpace,
        NTable,

        ActionGroup,
        Alert,
        Card,
        CardMargin,
        DeleteObject,
        EveIcon,
        InstalledRigs,
        InstalledServices,
        Item,
        Loader,
        PageHeader,
        ProjectGroupSelector,
        StructureInfo,
        System,
    }
})
class StructureUpdate extends Vue {
    @Prop({
        required: true,
        type: String,
    })
    public structureId!: Uuid;

    public busy: boolean = false;
    public errors = {
        notFound: false,
        updating: false,
        deleting: false,
    };

    public structure!: Structure;

    public async created() {
        this.busy = true;
        await StructureService
            .fetch(this.structureId)
            .then(x => {
                this.structure = x;
                return this.structure.fetchPermissionIsOwner()
            })
            .then(_ => {
                this.busy = false;
            })
            .catch(e => {
                if (e.response.status === 404) {
                    this.errors.notFound = true;
                }

                this.busy = false;
            });
    }

    public async save() {
        await this.structure
            .update()
            .catch(_ => this.errors.updating = true);
    }

    public async deleteObject() {
        await this.structure
            .remove()
            .then(_ => {
                this.$router.push({
                    name: ROUTE_STRUCTURES,
                });
            })
            .catch(_ => this.errors.deleting = true);
    }

    public async resolveStructureName() {
        //this.loadingBar.start();

        await StructureService
            .resolve(this.structure.ingameId)
            .then(x => {
                this.structure.name = x.name;
                return this.structure.update();
            })
            .then(_ => {
                //this.loadingBar.finish();
            })
            .catch(_ => {
                //this.loadingBar.error();
            });
    }

    get isReadonly(): boolean {
        if (this.structure && this.structure.isOwner) {
            return !this.structure.isOwner;
        }

        return true;
    }
}

export default toNative(StructureUpdate);
</script>
