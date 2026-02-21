import { EntityList, type Entity } from "@starfoundry/components/list/EntityList";
import { InGameSearch } from "@starfoundry/components/selectors/InGameSearch";
import type { InGameSearchResponse } from "@starfoundry/components/services/inGameSearch";
import { useRef, useState } from "react";

export function ShareIndustryHub() {
    const [selectedEntities, setSelectedEntities] = useState<Entity[]>([]);

    const inGameSearchRef = useRef<any>({} as any);

    return <>
        <InGameSearch
            categories={["alliance", "corporation", "character"]}
            onSelect={(x: InGameSearchResponse) => {
                setSelectedEntities([x as any]);
            }}
            ref={inGameSearchRef}
        />

        <EntityList
            entities={selectedEntities}
        />
    </>
}
