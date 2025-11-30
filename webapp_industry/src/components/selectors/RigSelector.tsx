import { MultiSelect } from "@mantine/core";
import { useState } from "react";

import type { TypeId } from "@/services/utils";
import type { StructureRig } from "@/services/structure/list";

export function RigSelector({
    rigs,
    selected = [],
    onSelect,
    readonly = false,
}: Props) {
    const [value, setValue] = useState<string[]>(
        () => {
            onSelect(selected);
            return selected
                ? selected.map(x => x.toString())
                : []
        }
    );

    let rigsSorted = rigs
        .sort((a, b) => a.item.name.localeCompare(b.item.name));

    const options = rigsSorted
        .map((rig) => {
            let excluded = rig.excludes.some(x => value.includes(x.toString()));

            return {
                value: rig.item.type_id.toString(),
                label: rig.item.name,
                disabled: excluded,
            }
        });

    return <>
        <MultiSelect
            data-1p-ignore
            data-cy="rigSelector"
            data={options}
            label={"Rigs"}
            value={value}
            maxValues={3}
            onChange={(value) => {
                setValue(value);
                onSelect(value.map(x => parseInt(x)));
            }}
            placeholder="Select Rig"
            nothingFoundMessage="No Rig found"
            disabled={readonly}
            clearable
            searchable
        />
    </>
}

export type Props = {
    // list of all rigs
    rigs:      StructureRig[];
    // selected value
    selected?: TypeId[];
    // event that fires when an element was selected
    onSelect:  (selected: TypeId[]) => void,
    // will make all inputs readonly
    readonly?: boolean;
}
