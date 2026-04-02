import { Text } from "@mantine/core"
import { useState } from "react";
import type { StructureRig } from "@internal/services/structure/list";
import { BadgeWrapper } from "@internal/wrapper/Badge";

export function StructureRigBadge({
    rigs,

    size = 'xs'
}: StructureRigBadgeProps) {
    const [showAll, setShowAll] = useState<boolean>(false);

    if (rigs.length === 0) {
        return <Text>The structure has no rigs installed</Text>
    }

    const t1Rigs = rigs
        .filter(x => !x.item.name.endsWith(' II'))
        .sort((a, b) => a.item.name.localeCompare(b.item.name));
    const t2Rigs = rigs
        .filter(x => x.item.name.endsWith(' II'))
        .sort((a, b) => a.item.name.localeCompare(b.item.name));

    const badges = [];
    for (const rig of [...t1Rigs, ...t2Rigs]) {
        if (badges.length > 2 && !showAll) {
            badges.push(<BadgeWrapper
                    onClick={() => setShowAll(true)}
                    size={size}
                >
                    Show all
                </BadgeWrapper>
            );
            break;
        }

        const name = rig
            .item
            .name
            .replace('Standup M-Set ', '')
            .replace('Standup L-Set ', '')
            .replace('Standup XL-Set ', '');

        if (name.endsWith(' II')) {
            badges.push(<BadgeWrapper
                key={rig.item.type_id}
                size={size}
                color="orange.9"
            >
                { name }
            </BadgeWrapper>);
        } else {
            badges.push(<BadgeWrapper
                key={rig.item.type_id}
                size={size}
                color="gray"
            >
                { name }
            </BadgeWrapper>);
        }
    }

    if (badges.length > 2 && showAll) {
        badges.push(<BadgeWrapper
                onClick={() => setShowAll(false)}
                size={size}
            >
                Show less
            </BadgeWrapper>
        );
    }

    return badges;
}

export type StructureRigBadgeProps = {
    rigs: StructureRig[],

    size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
}
