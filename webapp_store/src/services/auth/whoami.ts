import { axiosClient } from "../client";
import { useQuery } from "@tanstack/react-query";

export const WHO_AM_I = 'whoami';

export const whoami = async (): Promise<WhoAmI> => (await axiosClient())
    .get(
        `/api/auth/whoami`,
    )
    .then(x => x.data);

export type WhoAmI = {
    character_id: number,
    character_name: string,
    corporation_id: number,
    corporation_name: string,
    alliance_id?: number,
    alliance_name?: string,
    permission: 'ADMIN' | 'USER';
}

// For general use
export const useWhoami = () => {
    return useQuery({
        queryKey: [WHO_AM_I],
        queryFn: async () => whoami(),
        // keep the data cached for a day, after that the refresh_token will be invalid
        // ms * s * m * h
        staleTime: 1000 * 60 * 60 * 24,
    });
}
