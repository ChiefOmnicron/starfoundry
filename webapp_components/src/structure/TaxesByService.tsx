import type { StructureTax } from "@internal/services/structure/list";
import { NumberInput } from "@mantine/core";

export const TAXES_SERVICE_MODULES = [{
    name: 'Biochemical Reactions',
    serviceTypeId: 45539,
    default: 0,
}, {
    name: 'Hybrid Reactions',
    serviceTypeId: 45538,
    default: 0,
}, {
    name: 'Composite Reactions',
    serviceTypeId: 45537,
    default: 0,
}, {
    name: 'Manufacturing',
    serviceTypeId: 35878,
    default: 0,
}, {
    name: 'Manufacturing Capitals',
    serviceTypeId: 35881,
    default: 0,
}, {
    name: 'Manufacturing Super Capitals',
    serviceTypeId: 35877,
    default: 0,
}, {
    name: 'Research',
    serviceTypeId: 35891,
    default: 0,
}, {
    name: 'Research',
    serviceTypeId: 45550,
    default: 0,
}, {
    name: 'Invention',
    serviceTypeId: 35886,
    default: 0,
}, {
    name: 'Market',
    serviceTypeId: 35892,
    default: 0,
}, {
    name: 'Reprocessing',
    serviceTypeId: 35899,
    default: 0,
}];

export function TaxByService({
    taxes,
    services,

    onChange,
}: TaxByServiceProps) {
    const taxInput = services
        .map(x => {
            const entry = TAXES_SERVICE_MODULES.find(y => y?.serviceTypeId === x);
            if (entry) {
                return <NumberInput
                    label={entry.name}
                    value={taxes[x]}
                    suffix="%"
                    min={0}
                    allowDecimal
                    defaultValue={entry.default}
                    onChange={(value) => {
                        console.log(value)
                        taxes[x] = Number.parseFloat(value as string);
                        onChange(taxes);
                    }}
                />
            }
        });

    return <>
        {taxInput}
    </>
}

export type TaxByServiceProps = {
    taxes:    StructureTax,
    services: number[],

    onChange: (taxes: StructureTax) => void,
};
