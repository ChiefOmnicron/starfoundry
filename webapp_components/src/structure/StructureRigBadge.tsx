import { Badge, Text } from "@mantine/core"
import { useState } from "react";
import type { StructureRig } from "@internal/services/structure/list";

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
            badges.push(<Badge
                    onClick={() => setShowAll(true)}
                    size={size}
                    radius='xs'
                >
                    Show all
                </Badge>
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
            badges.push(<Badge
                autoContrast
                key={rig.item.type_id}
                size={size}
                color="orange.9"
                radius='xs'
            >
                { name }
            </Badge>);
        } else {
            badges.push(<Badge
                autoContrast
                key={rig.item.type_id}
                size={size}
                color="gray"
                radius='xs'
            >
                { name }
            </Badge>);
        }
    }

    if (badges.length > 2 && showAll) {
        badges.push(<Badge
                onClick={() => setShowAll(false)}
                size={size}
                radius='xs'
            >
                Show less
            </Badge>
        );
    }

    return badges;
}

export type StructureRigBadgeProps = {
    rigs: StructureRig[],

    size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
}
