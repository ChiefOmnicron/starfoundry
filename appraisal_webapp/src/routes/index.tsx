import { createFileRoute, useLocation, useNavigate } from '@tanstack/react-router'
import { Route as AppraisalRoute } from '@/routes/appraisal/index';

export const Route = createFileRoute('/')({
    component: IndexComponent,
});

async function IndexComponent() {
    const location = useLocation();
    const navigation = useNavigate();

    if (location.pathname === '/') {
        navigation({
            to: AppraisalRoute.to,
        });
    } else {
        return;
    }
}
