import { Badge } from "@mantine/core"
import type { ProjectJobStatus } from "@internal/services/projects/listJobs";

export function JobStatusBadge({
    jobStatus,

    size = 'xs',
}: JobStatusBadgeProps) {
    let color = 'red.9';
    let content = '';

    switch (jobStatus) {
        case 'WAITING_FOR_MATERIALS':
            color = 'red.9';
            content = 'Waiting for Materials';
            break;
        case 'READY_TO_START':
            color = 'orange.9';
            content = 'Ready to start';
            break;
        case 'BUILDING':
            color = 'blue.9';
            content = 'Building';
            break;
        case 'DONE':
            color = 'green.9';
            content = 'Done';
            break;
        default:
            break;
    }

    return <Badge
        autoContrast
        key={content}
        size={size}
        color={color}
        radius='xs'
    >
        {content}
    </Badge>
}

export type JobStatusBadgeProps = {
    jobStatus: ProjectJobStatus,

    size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
}
