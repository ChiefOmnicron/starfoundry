import { axiosClient, type AbortSignal } from "@internal/services/client";
import { useQuery } from "@tanstack/react-query";
import type { GenericAbortSignal } from "axios";

export const LIST_CHARACTERS = 'listCharacter';

export const listCharacters = async (
    signal?: GenericAbortSignal,
): Promise<AuthedCharacterInfo[]> => (await axiosClient())
    .get(
        '/api/eve/characters',
        {
            signal,
        }
    )
    .then(x => {
        if (x.status === 204) {
            return [];
        }

        return x.data;
    });

export const useListCharacters = () => {
    return useQuery({
        queryKey: [LIST_CHARACTERS],
        queryFn: async ({
            signal
        }: AbortSignal) => listCharacters(signal),
        // 10 minutes (ms * s * m)
        staleTime: 1000 * 60 * 10,
    })
}

export type AuthedCharacterInfo = {
    character_name:    string;
    character_id:      number;

    corporation_name?: string;
    corporation_id:    number;

    alliance_name?:    string;
    alliance_id?:      number;

    scopes:            string[];
}
