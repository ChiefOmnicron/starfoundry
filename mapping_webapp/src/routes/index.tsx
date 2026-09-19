import { createFileRoute, useLocation, useNavigate } from '@tanstack/react-router'
import { Route as RoutingRoute } from '@/routes/routing/index';

export const Route = createFileRoute('/')({
    component: IndexComponent,
});

async function IndexComponent() {
    const location = useLocation();
    const navigation = useNavigate();

    if (location.pathname === '/') {
        navigation({
            to: RoutingRoute.to,
        });
    } else {
        return;
    }
}
