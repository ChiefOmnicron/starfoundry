import { createFileRoute } from '@tanstack/react-router'
import { ProjectGroupDefaultsBlacklist } from '@starfoundry/components/projectGroup/defaults/Blacklist';
import { ProjectGroupDefaultsBlueprintOverwrite } from '@starfoundry/components/projectGroup/defaults/BlueprintOverwrite';
import { ProjectGroupDefaultsJobSplitting } from '@starfoundry/components/projectGroup/defaults/JobSplitting';
import { ProjectGroupDefaultsMarket } from '@starfoundry/components/projectGroup/defaults/Market';
import { Route as StructureRoute } from '@/routes/structures';
import { SaveDialog } from '@starfoundry/components/misc/SaveDialog';
import { Tabs } from '@mantine/core';
import { useState } from 'react';

export const Route = createFileRoute(
    '/project-groups_/$projectGroupId/defaults',
)({
    component: RouteComponent,
})

function RouteComponent() {
    const { projectGroupId } = Route.useParams();

    const [isDirtyMarket, setIsDirtyMarket] = useState<boolean>(false);
    const [isDirtyBlacklist, setIsDirtyBlacklist] = useState<boolean>(false);
    const [isDirtyBlueprintOverwrite, setIsDirtyBlueprintOverwrite] = useState<boolean>(false);
    const [isDirtyJobSplitting, setIsDirtyJobSplitting] = useState<boolean>(false);

    const [triggerSave, setTriggerSave] = useState<number>(0);
    const [triggerReset, setTriggerReset] = useState<number>(0);

    return <>
        <Tabs
            defaultValue="market"
            style={{
                marginTop: '10px'
            }}
        >
            <Tabs.List>
                <Tabs.Tab value="market">
                    Market
                </Tabs.Tab>
                <Tabs.Tab value="blacklist">
                    Blacklist
                </Tabs.Tab>
                <Tabs.Tab value="overwrites">
                    Blueprint Overwrites
                </Tabs.Tab>
                <Tabs.Tab value="job_splitting">
                    Job splitting
                </Tabs.Tab>
            </Tabs.List>

            <Tabs.Panel value="market">
                <ProjectGroupDefaultsMarket
                    structureView={StructureRoute.to}
                    projectGroupId={projectGroupId}

                    onTouchChange={setIsDirtyMarket}
                    triggerSave={triggerSave}
                    triggerReset={triggerReset}
                />
            </Tabs.Panel>

            <Tabs.Panel value="blacklist">
                <ProjectGroupDefaultsBlacklist
                    projectGroupId={projectGroupId}
                    onTouchChange={setIsDirtyBlacklist}
                    triggerSave={triggerSave}
                    triggerReset={triggerReset}
                />
            </Tabs.Panel>

            <Tabs.Panel value="overwrites">
                <ProjectGroupDefaultsBlueprintOverwrite
                    projectGroupId={projectGroupId}
                    onTouchChange={setIsDirtyBlueprintOverwrite}
                    triggerSave={triggerSave}
                    triggerReset={triggerReset}
                />
            </Tabs.Panel>

            <Tabs.Panel value="job_splitting">
                <ProjectGroupDefaultsJobSplitting
                    projectGroupId={projectGroupId}
                    onTouchChange={setIsDirtyJobSplitting}
                    triggerSave={triggerSave}
                    triggerReset={triggerReset}
                />
            </Tabs.Panel>
        </Tabs>

        <SaveDialog
            onReset={() => {
                setTriggerReset(triggerReset + 1);
            }}
            onSave={() => {
                setTriggerSave(triggerSave + 1);
            }}
            show={ isDirtyMarket || isDirtyBlacklist || isDirtyBlueprintOverwrite || isDirtyJobSplitting }
        />
    </>
}

