import { IndustryHubCard, type IndustryHubCardAdditionalProps } from "@internal/cards/IndustryHubCard";
import { SimpleGrid } from "@mantine/core";
import type { IndustryHub } from "@internal/services/industry-hub/list";

export function IndustryHubList({
    industryHubs,

    viewTarget = '_self',

    industryHubCardProps,
}: IndustryHubListProp) {
    const industryHubCard = () => {
        return industryHubs
            .map(x => <IndustryHubCard
                    key={x.id}
                    industryHub={x}
                    viewTarget={viewTarget}
                    {...industryHubCardProps}
                />
            );
    }

    return <>
        <SimpleGrid
            cols={{
                base: 1,
                sm: 2,
            }}
        >
            { industryHubCard() }
        </SimpleGrid>
    </>
}

export type IndustryHubListProp = {
    industryHubs: IndustryHub[];

    viewTarget?: '_blank' | '_self';

    industryHubCardProps?: IndustryHubCardAdditionalProps;
}
