export function normalizeRigServiceName(name: string): string {
    return name
        .replace('Standup M-Set ', '')
        .replace('Standup L-Set ', '')
        .replace('Standup XL-Set ', '')
        .replace('Standup ', '');
}
