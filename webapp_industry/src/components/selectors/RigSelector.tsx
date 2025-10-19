import { Select } from "@mantine/core";
import { useState } from "react";

import type { TypeId } from "@/services/utils";
import type { StructureRig } from "@/services/structure/resolveStructure";

export function RigSelector({
    rigs,
    label,
    filter = [],
    selected = null,
    onSelect,
}: Props) {
    const [value, setValue] = useState<string | null>(selected ? selected.toString() : null);

    let rigsSorted = rigs
        .sort((a, b) => a.item.name.localeCompare(b.item.name));

    const options = rigsSorted
        .map((rig) => {
            let selected = filter.includes(rig.item.type_id);
            let excluded = rig.excludes.some(x => filter.includes(x));

            return {
                value: rig.item.type_id.toString(),
                label: rig.item.name,
                disabled: selected || excluded,
            }
        });

    return <>
        <Select
            data={options}
            label={label}
            value={value}
            onChange={(value) => {
                setValue(value)
                onSelect(value as any as TypeId);
            }}
            placeholder="Select Rig"
            nothingFoundMessage="No Rig found"
            clearable
            searchable
        />
    </>
}

export type Props = {
    // list of all rigs
    rigs:      StructureRig[];
    // label that should be shown
    label:     string,
    // selected value
    selected?: TypeId | null;
    // list of rigs that are already selected
    filter?:   (TypeId | null)[];
    // event that fires when an element was selected
    onSelect:  (selected: null | TypeId) => void
}
