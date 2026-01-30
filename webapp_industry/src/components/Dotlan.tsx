import type { System } from "@/services/structure/list";
import { InternalLink } from "./InternalLink";

export function Dotlan({
    system,
}: Props) {
    const regionName = system.region_name.replace(/ /g, '_');
    const systemName = system.system_name.replace(/ /g, '_');
    const dotlanLink = `https://evemaps.dotlan.net/map/${regionName}/${systemName}`;

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
