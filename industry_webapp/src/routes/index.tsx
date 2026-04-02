import { createFileRoute, useLocation, useNavigate, useRouteContext } from '@tanstack/react-router'
import { Route as LoginRoute } from '@/routes/auth/login';
import { Route as ProjectRoute } from '@/routes/projects/index';

export const Route = createFileRoute('/')({
    component: IndexComponent,
});

async function IndexComponent() {
    const location = useLocation();
    const navigation = useNavigate();
    const context = useRouteContext({ from: '/' });

    const isLoggedIn = await context.auth.isAuthenticated();
    if (location.pathname === '/' && isLoggedIn) {
        navigation({
            to: ProjectRoute.to,
        });
    } else if (isLoggedIn) {
        return;
    } else {
        navigation({
            to: LoginRoute.to,
        });
    }
}
