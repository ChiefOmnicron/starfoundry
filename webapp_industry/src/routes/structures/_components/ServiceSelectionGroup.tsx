import { ServiceSelector } from "@/components/selectors/ServiceSelector";
import { type  StructureService } from "@/services/structure/resolveStructure";
import { type TypeId } from "@/services/utils";
import { useState } from "react";

export function ServiceSelectionGroup({
    services,
    onSelect,
}: Props) {
    const [selectedService, setSelectedService] = useState<(TypeId | null)[]>([]);

    return <>
        <ServiceSelector
            label="Service 1"
            services={services}
            filter={selectedService}
            selected={selectedService[0]}
            onSelect={(selected: null | TypeId) => {
                let service = [
                    Number(selected),
                    selectedService[1],
                    selectedService[2],
                    selectedService[3],
                    selectedService[4],
                    selectedService[5],
                    selectedService[6],
                ];
                setSelectedService(service);
                onSelect(service);
            }}
        />
        <ServiceSelector
            label="Service 2"
            services={services}
            filter={selectedService}
            selected={selectedService[1]}
            onSelect={(selected: null | TypeId) => {
                let service = [
                    selectedService[0],
                    Number(selected),
                    selectedService[2],
                    selectedService[3],
                    selectedService[4],
                    selectedService[5],
                    selectedService[6],
                ];
                setSelectedService(service);
                onSelect(service);
            }}
        />
        <ServiceSelector
            label="Service 3"
            services={services}
            filter={selectedService}
            selected={selectedService[2]}
            onSelect={(selected: null | TypeId) => {
                let service = [
                    selectedService[0],
                    selectedService[1],
                    Number(selected),
                    selectedService[3],
                    selectedService[4],
                    selectedService[5],
                    selectedService[6],
                ];
                setSelectedService(service);
                onSelect(service);
            }}
        />
        {
            services.slots >= 4
            ? 
                <ServiceSelector
                    label="Service 4"
                    services={services}
                    filter={selectedService}
                    selected={selectedService[3]}
                    onSelect={(selected: null | TypeId) => {
                        let service = [
                            selectedService[0],
                            selectedService[1],
                            selectedService[2],
                            Number(selected),
                            selectedService[4],
                            selectedService[5],
                            selectedService[6],
                        ];
                        setSelectedService(service);
                        onSelect(service);
                    }}
                />
            : <></>
        }
        {
            services.slots >= 5
            ? 
                <ServiceSelector
                    label="Service 5"
                    services={services}
                    filter={selectedService}
                    selected={selectedService[4]}
                    onSelect={(selected: null | TypeId) => {
                        let service = [
                            selectedService[0],
                            selectedService[1],
                            selectedService[2],
                            selectedService[3],
                            Number(selected),
                            selectedService[5],
                            selectedService[6],
                        ];
                        setSelectedService(service);
                        onSelect(service);
                    }}
                />
            : <></>
        }
        {
            services.slots >= 6
            ? 
                <ServiceSelector
                    label="Service 6"
                    services={services}
                    filter={selectedService}
                    selected={selectedService[5]}
                    onSelect={(selected: null | TypeId) => {
                        let service = [
                            selectedService[0],
                            selectedService[1],
                            selectedService[2],
                            selectedService[3],
                            selectedService[4],
                            Number(selected),
                            selectedService[6],
                        ];
                        setSelectedService(service);
                        onSelect(service);
                    }}
                />
            : <></>
        }
        {
            services.slots >= 7
            ? 
                <ServiceSelector
                    label="Service 7"
                    services={services}
                    filter={selectedService}
                    selected={selectedService[6]}
                    onSelect={(selected: null | TypeId) => {
                        let service = [
                            selectedService[0],
                            selectedService[1],
                            selectedService[2],
                            selectedService[3],
                            selectedService[4],
                            selectedService[5],
                            Number(selected),
                        ];
                        setSelectedService(service);
                        onSelect(service);
                    }}
                />
            : <></>
        }
    </>
}

export type Props = {
    // list of all services
    services: StructureService;
    // values that were selected
    values?: TypeId[];
    // event fired when the suer selects a rig
    onSelect: (selected: (TypeId | null)[]) => void;
}
