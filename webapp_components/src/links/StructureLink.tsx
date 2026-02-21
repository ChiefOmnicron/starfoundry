import type { Structure } from "@internal/services/structure/list";
import { InternalLink } from "./InternalLink";

export function StructureLink({
    structure,
    structureRoute,
}: Props) {
    const name = structure
        .name
        .replace(`${structure.system.system_name} - `, '');

    return <>
        <InternalLink
            to={ structureRoute }
            params={{
                structureId: structure.id,
            } as any}
            target="_blank"
            content={ name }
        />
    </>
}

export type Props = {
    structure: Structure,
    structureRoute: string,
}
