import { Stack, Title } from "@mantine/core";
import { ProjectJobAction, type ProjectJobMinimal } from "./ProjectJobAction";
import { ProjectJobListTable } from "./ProjectJobListTable";
import { useState } from "react";
import type { ProjectJob, ProjectJobGroup, ProjectJobStatus } from "@internal/services/projects/listJobs"
import type { Uuid } from "@internal/services/utils";

export function ProjectJobList({
    projectId,
    jobs,

    status,

    showCost = false,
    showStatus = false,
    showRemaining = false,
    showStarted = false,

    checkable = false,
    groupByHeader = false,

    editable = false,
    showQuickFix = false,

    onCreated,
}: ProjectJobListProps) {
    const [selectedRows, setSelectedRows] = useState<ProjectJobMinimal[]>([]);

    const header = (
        header: string,
    ): string => {
        switch (header) {
            case 'INTERMEDIATE_REACTIONS':
                return 'Intermediate Reactions';
            case 'COMPOSITE_REACTIONS':
                return 'Composite Reactions';
            case 'BIOCHEM_REACTIONS':
                return 'Biochemical Reactions';
            case 'HYBRID_REACTIONS':
                return 'Hybrid Reactions';
            case 'CONSTRUCTION_COMPONENTS':
                return 'Construction Components';
            case 'ADVANCED_CAPITAL_CONSTRUCTION_COMPONENTS':
                return 'Advanced Capital Construction Components';
            case 'CAPITAL_CONSTRUCTION_COMPONENTS':
                return 'Capital Construction Components';
            case 'TOOLS':
                return 'Tools';
            case 'T1_STUFF':
                return 'T1 Stuff';
            case 'T2_STUFF':
                return 'T2 Stuff';
            case 'CHARGES':
                return 'Charges';
            case 'SHIPS':
                return 'Ships';
            default:
                return 'Unknown'
        }
    }

    const onSelect = (projectJobs: ProjectJob[]) => {
        setSelectedRows(projectJobs.map(x => {
            return {
                job_id:     x.id,
                project_id: x.project_id,
            }
        }))
    }

    const content = () => {
        if (groupByHeader) {
            return jobs
                .filter(x => {
                    if (!status) {
                        return true;
                    }

                    return !!x.entries.find(y => y.status === status);
                })
                .map(x => {
                    let entries = x.entries;
                    if (status) {
                        entries = x
                            .entries
                            .filter(y => y.status === status);
                    }

                    return <>
                        <Title order={3}>{header(x.header)}</Title>
                        <ProjectJobListTable
                            projectId={projectId}
                            jobs={entries}

                            checkable={checkable}
                            onSelect={onSelect}

                            editable={editable}
                            showQuickFix={showQuickFix}

                            showCost={showCost}
                            showStatus={showStatus}
                            showRemaining={showRemaining}
                        />
                    </>
                })
        } else {
            const flattened = jobs
                .flatMap(x => x.entries)
                .filter(x => {
                    if (!status) {
                        return true;
                    }

                    return x.status === status;
                });
            return <ProjectJobListTable
                projectId={projectId}
                jobs={flattened}

                checkable={checkable}
                onSelect={onSelect}

                editable={editable}
                showQuickFix={showQuickFix}

                showCost={showCost}
                showStatus={showStatus}
                showRemaining={showRemaining}
                showStarted={showStarted}
            />;
        }
    }

    return <>
        <ProjectJobAction
            selected={selectedRows}
            onCreated={onCreated}
        />

        <Stack>
            {content()}
        </Stack>
    </>
}

export type ProjectJobListProps = {
    projectId:      Uuid;
    jobs:           ProjectJobGroup[];

    status?:        ProjectJobStatus,

    showCost?:      boolean;
    showStatus?:    boolean;
    showRemaining?: boolean;
    showStarted?:   boolean;

    groupByHeader?: boolean;
    checkable?:     boolean;
    editable?:      boolean;
    showQuickFix?:  boolean;

    onCreated:      (id: Uuid) => void;
}

