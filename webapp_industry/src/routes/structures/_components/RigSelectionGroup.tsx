import { RigSelector } from "@/components/selectors/RigSelector";
import { useState } from "react";
import { type TypeId } from "@/services/utils";
import type { StructureRig } from "@/services/structure/resolveStructure";

export function RigSelectionGroup({
    rigs,
    onSelect,
}: Props) {
    const [selectedRigs, setSelectedRigs] = useState<(TypeId | null)[]>([]);

    return <>
        <RigSelector
            label="Rig 1"
            rigs={rigs}
            filter={selectedRigs}
            selected={selectedRigs[0]}
            onSelect={(selected: null | TypeId) => {
                let rigs = [
                    Number(selected),
                    selectedRigs[1],
                    selectedRigs[2],
                ];
                setSelectedRigs(rigs);
                onSelect(rigs);
            }}
        />
        <RigSelector
            label="Rig 2"
            rigs={rigs}
            filter={selectedRigs}
            selected={selectedRigs[1]}
            onSelect={(selected: null | TypeId) => {
                let rigs = [
                    selectedRigs[0],
                    Number(selected),
                    selectedRigs[2],
                ];
                setSelectedRigs(rigs);
                onSelect(rigs);
            }}
        />
        <RigSelector
            label="Rig 3"
            rigs={rigs}
            filter={selectedRigs}
            selected={selectedRigs[2]}
            onSelect={(selected: null | TypeId) => {
                let rigs = [
                    selectedRigs[0],
                    selectedRigs[1],
                    Number(selected),
                ];
                setSelectedRigs(rigs);
                onSelect(rigs);
            }}
        />
    </>
}

export type Props = {
    // list of all rigs
    rigs:      StructureRig[];
    // values that were selected
    values?: TypeId[];
    // event fired when the suer selects a rig
    onSelect: (selected: (TypeId | null)[]) => void;
}
