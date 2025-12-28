import { MultiSelect } from "@mantine/core";
import { useEffect, useState } from "react";

import type { TypeId } from "@/services/utils";
import type { StructureService } from "@/services/structure/list";

export function ServiceSelector({
    services,
    selected = [],
    onSelect,
    readonly = false,
}: Props) {
    const [value, setValue] = useState<string[]>([]);
    useEffect(() => {
        setValue(selected.map(x => x.toString()));
    }, [selected]);

    let servicesSorted = services
        .services
        .sort((a, b) => a.name.localeCompare(b.name));

    const options = servicesSorted
        .map((service) => {
            return {
                value: service.type_id.toString(),
                label: service.name,
            }
        });

    return <>
        <MultiSelect
            data-1p-ignore
            data-cy="serviceSelector"
            data={options}
            label={"Services"}
            value={value}
            maxValues={services.slots}
            onChange={(value) => {
                setValue(value)
                onSelect(value.map(x => parseInt(x)));
            }}
            placeholder="Select Service"
            nothingFoundMessage="No Service found"
            disabled={readonly}
            clearable
            searchable
        />
    </>
}

export type Props = {
    // list of all rigs
    services:  StructureService;
    // selected value
    selected?: TypeId[];
    // event that fires when an element was selected
    onSelect:  (selected: TypeId[]) => void,
    // will make all inputs readonly
    readonly?: boolean;
}
