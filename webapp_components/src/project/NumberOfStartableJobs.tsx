import { Text } from "@mantine/core";
import type { ProjectJob } from "@internal/services/projects/listJobs"

export function NumberOfStartableJobs({
    jobs,
}: NumberOfStartableJobsProps) {
    const jobCount = jobs
        .filter(x => x.status === 'READY_TO_START')
        .reduce((prev) => prev += 1, 0);

    return <Text>Number of startable jobs: {jobCount}</Text>
}

export type NumberOfStartableJobsProps = {
    jobs: ProjectJob[];
}
