import type { Structure } from "@/services/structure/list";
import { Route as StructureRoute } from '@/routes/structures_/$structureId.index';
import { InternalLink } from "./InternalLink";

export function StructureLink({
    structure,
}: Props) {
    const name = structure
        .name
        .replace(`${structure.system.system_name} - `, '');

    return <>
        <InternalLink
            to={ StructureRoute.to }
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
}
