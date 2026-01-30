import { Flex } from "@mantine/core";
import type { ProjectJob, ProjectJobGroup } from "@/services/projects/listJobs";

export function ProjectProgressBar({
    jobs,
}: ProjectProgressBarProp) {
    const backgroundColor = (job: ProjectJob): string => {
        switch (job.status) {
            case 'WAITING_FOR_MATERIALS':
                return 'var(--mantine-color-red-9)'
            case 'BUILDING':
                return 'var(--mantine-color-blue-9)'
            case 'DONE':
                return 'var(--mantine-color-green-9)'
            default:
                return '';
        }
    }

    const entries = () => {
        const jobsMerged = (jobs || []).flatMap(x => x.entries);

        return jobsMerged
            .map(x => {
                return <div
                    key={x.id}
                    style={{
                        width: `100%`,
                        backgroundColor: backgroundColor(x),
                        height: `5px`,
                        float: 'left',
                    }}
                />
            });
    }

    return <>
        <Flex
            style={{
                width: '100%',
                overflow: 'hidden',
            }}
        >
            { entries() }
        </Flex>
    </>
}

export type ProjectProgressBarProp = {
    jobs: ProjectJobGroup[];
};
