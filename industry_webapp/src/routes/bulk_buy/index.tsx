import { Stepper } from '@mantine/core'
import { ProjectList } from '@starfoundry/components/project/ProjectList'
import { useListProjects } from '@starfoundry/components/services/projects/list'
import { createFileRoute } from '@tanstack/react-router'
import { useState } from 'react'

export const Route = createFileRoute('/bulk_buy/')({
  component: RouteComponent,
})

function RouteComponent() {
    let {
        data: projects,
    } = useListProjects({
        status: 'DRAFT'
    });

    const [active, setActive] = useState(1);

    // Project Groups -> Configure Hauling routes
    // Select a starting structure
    // Select intermediate
    // Set alarms for hauling routes with low cynos/fuel

    // - Select projects
    // - For loop
    //  - Set product
    //  - Name
    //  - Insert Stock
    //  - BPCs
    //  - Market
    //  - Move stock
    // - Shipping
    //  - Create contracts with the items given
    //  - Track those contracts using the description field -> generate a short id
    // - Distribution

    return <>
        <Stepper
            size='xs'
            active={active}
            onStepClick={setActive}
        >
            <Stepper.Step
                label="Setup"
            >
                Configuration
            </Stepper.Step>

            <Stepper.Step
                label="Buy Materials"
            >
                <ProjectList
                    projects={projects || []}
                />
            </Stepper.Step>

            <Stepper.Step
                label="Shipping"
            >
                BBBB
            </Stepper.Step>

            <Stepper.Step
                label="Distribution"
            >
                <ProjectList
                    projects={projects || []}
                />
            </Stepper.Step>
        </Stepper>
    </>
}
