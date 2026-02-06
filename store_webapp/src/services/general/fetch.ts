import { useQuery } from "@tanstack/react-query";
import axios from "axios";

export const FETCH_PRODUCT = 'generalInformation';

export const generalInformation = async (): Promise<GeneralInformation> => axios
    .get(
        `/api/general/info`,
    )
    .then(x => x.data);

export type GeneralInformation = {
    name: string,
};

// For general use
export const useGeneralInformation = () => {
    return useQuery({
        queryKey: [FETCH_PRODUCT],
        queryFn: async () => generalInformation(),
        // ms * s * m * h
        staleTime: 1000 * 60 * 60 * 24,
    });
}
