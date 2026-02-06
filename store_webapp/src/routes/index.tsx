import { createFileRoute, useLocation, useNavigate, useRouteContext } from '@tanstack/react-router'
import { Route as LoginRoute } from '@/routes/auth/login';
import { Route as StoreRoute } from '@/routes/products/index';

export const Route = createFileRoute('/')({
    component: IndexComponent,
});

function IndexComponent() {
    const location = useLocation();
    const navigation = useNavigate();
    const context = useRouteContext({ from: '/' });

    context.auth.isAuthenticated()
        .then(authed => {
            if (location.pathname === '/' && authed) {
                navigation({
                    to: StoreRoute.to,
                });
            } else if (authed) {
                return;
            } else {
                navigation({
                    to: LoginRoute.to,
                });
            }
        })
        .catch(_ => {
            navigation({
                to: LoginRoute.to,
            });
        });
}
