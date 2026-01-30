import { InGameSearch } from "@/components/selectors/InGameSearch";
import { useRef } from "react";

export function ShareIndustryHub() {
    const inGameSearchRef = useRef<any>({} as any);

    return <>
        <InGameSearch
            category="character"
            onSelect={() => {}}
            ref={inGameSearchRef}
        />
    </>
}
