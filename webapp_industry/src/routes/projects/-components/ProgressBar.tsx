import { useListProjectJobs, type ProjectJob } from "@/services/projects/list_jobs";
import type { Uuid } from "@/services/utils";

export function ProjectProgressBar({
    projectId,
}: ProjectProgressBarProp) {
    const {
        isPending,
        data: jobs
    } = useListProjectJobs(projectId);

    if (isPending) {
        return <></>;
    }

    const backgroundColor = (job: ProjectJob): string => {
        switch (job.status) {
            case 'WAITING_FOR_MATERIALS':
                return 'rgba(232, 128, 128, 0.5)';
            case 'BUILDING':
                return 'rgba(112, 192, 232, 0.5)';
            case 'DONE':
                return 'rgba(99, 226, 183, 0.5)';
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
                        width: `${100 / jobsMerged.length}%`,
                        backgroundColor: backgroundColor(x),
                        height: '15px',
                        float: 'left',
                    }}
                />
            });
    }

    return <>
        <div
            style={{
                width: '100%',
                overflow: 'hidden',
            }}
        >
            { entries() }
        </div>
    </>
}

export type ProjectProgressBarProp = {
    projectId: Uuid;
};
