import { Stepper } from '@mantine/core';
import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import { GeneralInfo } from '@/routes/projects/assistant/-components/GeneralInfo';
import { Solution } from '@/routes/projects/assistant/-components/Solution';
import type { Uuid } from '@starfoundry/components/services/utils';

export const Route = createFileRoute('/projects/assistant/')({
    component: RouteComponent,
});

function RouteComponent() {
    const [active, setActive] = useState<number>(0);

    const [generalInfo, setGeneralInfo] = useState<ProjectAssistantGeneralInformation | null>(null);

    const nextStep = () => setActive((current) => (current < 3 ? current + 1 : current));
    const prevStep = () => setActive((current) => (current > 0 ? current - 1 : current));
    
    return <>
        <Stepper
            active={active}
            onStepClick={setActive}
            allowNextStepsSelect={false}
        >
            <Stepper.Step
                label="First step"
                description="General information"
            >
                <GeneralInfo
                    nextStep={(info: ProjectAssistantGeneralInformation) => {
                        setGeneralInfo(info);
                        nextStep();
                    }}
                    prevStep={prevStep}
                />
            </Stepper.Step>
            <Stepper.Step
                label="Second step"
                description="Adjust solution"
            >
                <Solution
                    nextStep={() => {}}
                    prevStep={() => {}}
                    projectGroupId={(generalInfo || { projectGroupId: '' }).projectGroupId}
                />
            </Stepper.Step>
            <Stepper.Step
                label="Final step"
                description="Project created"
            >
                Step 3 content: Get full access
            </Stepper.Step>
            <Stepper.Completed>
                Completed, click back button to get to previous step
            </Stepper.Completed>
        </Stepper>
    </>
}

export type ProjectAssistantGeneralInformation = {
    name: string,
    orderer: string,
    sellPrice: number,

    projectGroupId: Uuid,
}
