import type { System } from "@/services/structure/list";
import { InternalLink } from "./InternalLink";

export function Dotlan({
    system,
}: Props) {
    let dotlanLink = `https://evemaps.dotlan.net/map/${system.region_name}/${system.system_name}`;

    return <>
        <InternalLink
            to={dotlanLink}
            target="_blank"
            content={ `${system.system_name} (${system.region_name})` }
        />
    </>
}

export type Props = {
    system: System,
}
