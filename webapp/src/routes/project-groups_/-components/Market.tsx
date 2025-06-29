import type { Uuid } from "@/services/utils";
import { Text } from "@mantine/core";
import type { ReactElement } from "react";

export function ProjectGroupMarket({
    entries,
}: ProjectGroupMarketProp): ReactElement {
    return <>
        {
            entries.map(x => <Text>{JSON.stringify(x)}</Text>)
        }
    </>
}

export type ProjectGroupMarketProp = {
    entries: Uuid[];
}
