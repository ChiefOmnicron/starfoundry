import type { Structure } from "@internal/services/structure/list";
import { useDisclosure } from "@mantine/hooks";
import { StructureViewModal } from "@internal/detailView/StructureView";
import { UnstyledButton } from "@mantine/core";

export function StructureLink({
    structure,
}: Props) {
    const [opened, { open, close }] = useDisclosure(false);

    const name = structure
        .name
        .replace(`${structure.system.system_name} - `, '');

    return <>
        <StructureViewModal
            onClose={close}
            opened={opened}

            structure={structure}
            showBlueprintBonus={false}
        />

        <UnstyledButton
            onClick={open}
        >
            { name }
        </UnstyledButton>
    </>
}

export type Props = {
    structure: Structure,
}
