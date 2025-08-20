import { createFileRoute, Outlet } from '@tanstack/react-router'
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { Text, Title } from '@mantine/core';
import { useFetchProduct } from '@/services/product/fetch';

export const Route = createFileRoute('/products_/$productUuid')({
    beforeLoad: async ({ context }) => {
        if (!await context.auth.isAuthenticated()) {
            throw context.auth.login();
        }
    },
    component: StoreDetailRoot,
})

export function StoreDetailRoot() {
    const { productUuid } = Route.useParams();

    const {
        isPending,
        isError,
        data: product
    } = useFetchProduct(productUuid);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    return <>
        <Title
            data-cy="header"
            order={1}
        >
            { product?.name }
        </Title>

        <Text>{ product?.description }</Text>

        <Outlet />
    </>
}
