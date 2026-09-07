import { BadgeWrapper } from "../wrapper/Badge";
import { type MantineSize } from "@mantine/core"

export function ProjectStatusBadge({
    status,

    size = 'xs',
}: ProjectStatusBadgeProps) {
    if (status.length === 0) {
        return <></>
    }

    let color = '';
    let content = '';

    switch(status) {
        case 'DONE':
            color = 'green.9';
            content = 'Done';
            break;
        case 'IN_PROGRESS':
            color = 'blue.9';
            content = 'In Progress';
            break;
        case 'READY_TO_START':
            color = 'cyan.9';
            content = 'Ready to start';
            break;
        case 'PAUSED':
            color = 'yellow.9';
            content = 'Pause';
            break;
        default:
            color = 'gray.9';
            content = 'Draft';
    }

    return <BadgeWrapper
        key={content}
        color={color}
        size={size}
    >
        {content}
    </BadgeWrapper>
}

export type ProjectStatusBadgeProps = {
    status:   'DONE' | 'IN_PROGRESS' | 'READY_TO_START' | 'PAUSED' | 'DRAFT';

    size?:  MantineSize;
}
