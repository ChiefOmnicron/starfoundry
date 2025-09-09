import { createFileRoute, Outlet } from '@tanstack/react-router'
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { Title } from '@mantine/core';
import { useFetchOrderAdmin } from '@/services/order/fetch_admin';

export const Route = createFileRoute('/admin/orders_/$orderUuid')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: StoreDetailRoot,
})

function StoreDetailRoot() {
    const { orderUuid } = Route.useParams();

    const {
        isPending,
        isError,
        data: order,
    } = useFetchOrderAdmin(orderUuid);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const orderName = () => {
        return order
            .products
            .find(x => !x.is_additional)?.name
    }

    return <>
        <Title
            data-cy="header"
            order={1}
        >
            Your '{ orderName() }' order
        </Title>

        <Outlet />
    </>
}
