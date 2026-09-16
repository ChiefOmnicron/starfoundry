import { QueryClient } from "@tanstack/react-query";

export const DEFAULT_QUERY_CLIENT = new QueryClient({
    defaultOptions: {
        queries: {
            // @ts-ignore err: unknown -> error: AxiosError
            retry: (failureCount, error: AxiosError) => {
                // do not retry on 404
                if (error.response?.status === 401) {
                    return false;
                }

                const defaultRetry = new QueryClient().getDefaultOptions().queries?.retry;
                return Number.isSafeInteger(defaultRetry)
                // @ts-ignore
                ? failureCount < (defaultRetry ?? 0)
                : false;
            }
        }
    }
})
