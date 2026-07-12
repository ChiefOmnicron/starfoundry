import { axiosClient } from "@internal/services/client";

export const triggerTag = async (): Promise<void> => (await axiosClient())
    .put(
        `/api/tags`,
        {},
        {
            headers: {
                'Content-Type': 'application/json',
            }
        }
    )
    .then(x => {
        if (x.status === 204) {
            return [];
        }

        return x.data;
    });
