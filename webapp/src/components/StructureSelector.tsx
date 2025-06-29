import type { TypeId, Uuid } from "@/services/utils";
import { Text } from "@mantine/core";
import type { ReactElement } from "react";

export function StructureSelector({
    onSelect,
}: StructureSelectorProp): ReactElement {
    return <>

    </>
}

export type StructureSelectorProp = {
    onSelect: (entry: Uuid) => void;

    // required service for structures that are shown
    service: TypeId,
}
