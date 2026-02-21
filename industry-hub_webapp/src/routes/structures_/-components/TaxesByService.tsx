import { NumberInput } from "@mantine/core";
import type { StructureTax } from "@starfoundry/components/services/structure/list";

const taxByService = [{
    name: 'Biochemical Reactions',
    serviceTypeId: 45539,
    default: 1,
}, {
    name: 'Hybrid Reactions',
    serviceTypeId: 45538,
    default: 1,
}, {
    name: 'Composite Reactions',
    serviceTypeId: 45537,
    default: 1,
}, {
    name: 'Manufacturing',
    serviceTypeId: 35878,
    default: 1,
}, {
    name: 'Manufacturing Capitals',
    serviceTypeId: 35881,
    default: 1,
}, , {
    name: 'Manufacturing Super Capitals',
    serviceTypeId: 35877,
    default: 1,
}, , {
    name: 'Research',
    serviceTypeId: 35891,
    default: 1,
}, , {
    name: 'Research',
    serviceTypeId: 45550,
    default: 1,
}, {
    name: 'Invention',
    serviceTypeId: 35886,
    default: 1,
}, {
    name: 'Market',
    serviceTypeId: 35892,
    default: 1.5,
}, {
    name: 'Reprocessing',
    serviceTypeId: 35899,
    default: 2,
}];

export function TaxByService({
    taxes,
    services,

    onChange,
}: TaxByServiceProps) {
    const taxInput = services
        .map(x => {
            const entry = taxByService.find(y => y?.serviceTypeId === x);
            if (entry) {
                return <NumberInput
                    label={entry.name}
                    value={taxes[x]}
                    suffix="%"
                    min={0}
                    allowDecimal
                    defaultValue={1}
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
