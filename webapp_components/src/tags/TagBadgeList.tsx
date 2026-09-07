import { BadgeWrapper } from "../wrapper/Badge";
import { Group, type MantineSize } from "@mantine/core"
import type { Tag } from "../services/tags/list";

export function TagBadgeList({
    tags,

    size = 'xs',
}: TagBadgeListProps) {
    if (tags.length === 0) {
        return <></>
    }

    const entries = tags
        .map(x => <BadgeWrapper
                key={x.content}
                size={size}
                color={x.color}
            >
                {x.content}
            </BadgeWrapper>
        );

    return <Group gap='xs'>
        {entries}
    </Group>

}

export type TagBadgeListProps = {
    tags:   Tag[];

    size?:  MantineSize;
}
