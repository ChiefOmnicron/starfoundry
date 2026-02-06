import { CharacterCorporationAllianceList, type CharacterCorporationAlliance } from "@/components/EntityList";
import { InGameSearch } from "@/components/selectors/InGameSearch";
import type { InGameSearchResponse } from "@/services/inGameSearch";
import { useRef, useState } from "react";

export function ShareIndustryHub() {
    const [selectedEntities, setSelectedEntities] = useState<CharacterCorporationAlliance[]>([]);

    const inGameSearchRef = useRef<any>({} as any);

    return <>
        <InGameSearch
            categories={["alliance", "corporation", "character"]}
            onSelect={(x: InGameSearchResponse) => {
                setSelectedEntities([x as any]);
            }}
            ref={inGameSearchRef}
        />

        <CharacterCorporationAllianceList
            characterCorporationAlliances={selectedEntities}
        />
    </>
}
