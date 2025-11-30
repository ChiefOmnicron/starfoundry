import { Link } from "@tanstack/react-router";
import { UnstyledButton } from "@mantine/core";
import type { Structure } from "@/services/structure/list";
import { Route as StructureRoute } from '@/routes/structures_/$structureId.index';

export function StructureLink({
    structure,
}: Props) {
    const name = structure
        .name
        .replace(`${structure.system.system_name} - `, '');

    return <>
        <UnstyledButton
            component={Link}
            to={StructureRoute.to}
            params={{
                structureId: structure.id,
            } as any}
            target="_blank"
            style={{
                color: 'var(--mantine-color-blue-4)',
                fontSize: 'var(--mantine-font-size-sm)'
            }}
        >
            { name }
        </UnstyledButton>
    </>
}

export type Props = {
    structure: Structure,
}
