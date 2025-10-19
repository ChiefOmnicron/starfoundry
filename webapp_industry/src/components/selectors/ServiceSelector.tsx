import { Select } from "@mantine/core";
import { useState } from "react";

import type { TypeId } from "@/services/utils";
import type { StructureService } from "@/services/structure/resolveStructure";

export function ServiceSelector({
    services,
    label,
    filter = [],
    selected = null,
    onSelect,
}: Props) {
    const [value, setValue] = useState<string | null>(selected ? selected.toString() : null);

    let servicesSorted = services
        .services
        .sort((a, b) => a.name.localeCompare(b.name));

    const options = servicesSorted
        .map((service) => {
            let selected = filter.includes(service.type_id);

            return {
                value: service.type_id.toString(),
                label: service.name,
                disabled: selected,
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
            placeholder="Select Service"
            nothingFoundMessage="No Service found"
            clearable
            searchable
        />
    </>
}

export type Props = {
    // list of all rigs
    services:  StructureService;
    // label that should be shown
    label:     string,
    // selected value
    selected?: TypeId | null;
    // list of rigs that are already selected
    filter?:   (TypeId | null)[];
    // event that fires when an element was selected
    onSelect:  (selected: null | TypeId) => void
}
