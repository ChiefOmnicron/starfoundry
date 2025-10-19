import { Link } from "@tanstack/react-router";
import { UnstyledButton } from "@mantine/core";
import type { StructureSystem } from "@/services/structure/list";

export function Dotlan({
    system,
}: Props) {
    let dotlanLink = `https://evemaps.dotlan.net/map/${system.region_name}/${system.system_name}`;

    return <>
        <UnstyledButton
            component={Link}
            to={dotlanLink}
            target="_blank"
            style={{
                color: 'var(--mantine-color-blue-4)',
                fontSize: 'var(--mantine-font-size-sm)'
            }}
        >
            { system.system_name }
        </UnstyledButton>
    </>
}

export type Props = {
    system: StructureSystem,
}
