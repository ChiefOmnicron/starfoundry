import { InputWrapper, SegmentedControl } from "@mantine/core";
import type { ProjectStatus } from "@internal/services/projects/list";

const SELECTABLE_STATES = [{
    label: 'Ready to Start',
    value: 'READY_TO_START'
}, {
    label: 'In Progress',
    value: 'IN_PROGRESS',
}, {
    label: 'Paused',
    value: 'PAUSED'
}, {
    label: 'Done',
    value: 'DONE'
}];

export function ProjectStatusSelector({
    selected,

    onChange,
}: ProjectStatusSelectorProps) {
    return <>
        <InputWrapper
            label="Status"
        >
            <SegmentedControl
                fullWidth
                data={SELECTABLE_STATES}
                value={selected}
                onChange={(status: string) => {
                    onChange(status as ProjectStatus);
                }}
            />
        </InputWrapper>
    </>
}

export type ProjectStatusSelectorProps = {
    selected: ProjectStatus,

    onChange: (status: ProjectStatus) => void;
}
