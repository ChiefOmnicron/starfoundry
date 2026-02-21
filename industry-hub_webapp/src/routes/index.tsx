import { createFileRoute, useLocation, useNavigate, useRouteContext } from '@tanstack/react-router'
import { Route as IndustryHub } from '@/routes/industry-hubs/index';
import { Route as LoginRoute } from '@/routes/auth/login';

export const Route = createFileRoute('/')({
    component: IndexComponent,
});

async function IndexComponent() {
    const location = useLocation();
    const navigation = useNavigate();
    const context = useRouteContext({ from: '/' });

    console.log(location.pathname, location.pathname === '/' && await context.auth.isAuthenticated())
    const isLoggedIn = await context.auth.isAuthenticated();
    if (location.pathname === '/' && isLoggedIn) {
        navigation({
            to: IndustryHub.to,
        });
    } else if (isLoggedIn) {
        return;
    } else {
        navigation({
            to: LoginRoute.to,
        });
    }
}
