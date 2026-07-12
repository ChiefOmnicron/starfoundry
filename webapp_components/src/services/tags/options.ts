import type { ComboboxData } from "@mantine/core";

const compareIs = {
    label: 'is',
    value: 'IS'
};
const compareIsNot = {
    label: 'is not',
    value: 'IS_NOT'
};
const compareContains = {
    label: 'contains',
    value: 'CONTAINS'
};
const comparePattern = {
    label: 'pattern',
    value: 'PATTERN'
};

export const tagOptions: TagOption[] = [{
    label: 'project.name',
    value: 'PROJECT_NAME',
    compare: [
        compareIs,
        compareIsNot,
        compareContains,
        comparePattern,
    ]
}, {
    label: 'project.orderer',
    value: 'PROJECT_ORDERER',
    compare: [
        compareIs,
        compareIsNot,
        compareContains,
        comparePattern,
    ]
}, {
    label: 'project.status',
    value: 'PROJECT_STATUS',
    compare: [
        compareIs,
        compareIsNot,
    ]
}]

type TagOption = {
    label: string;
    value: string;
    compare: ComboboxData;
}
