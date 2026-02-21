export function normalizeRigServiceName(name: string): string {
    return name
        .replace('Standup M-Set ', '')
        .replace('Standup L-Set ', '')
        .replace('Standup XL-Set ', '')
        .replace('Standup ', '');
}

export function systemRigBonusModifier(systemSecurityStr: string): number {
    switch(systemSecurityStr){
        case 'LOWSEC':
            return 1.9;
        case 'NULLSEC':
            return 2.1;
        default:
            return 1
    }
}
