import { createFileRoute } from '@tanstack/react-router'
import { LoginComponent } from '@starfoundry/components/auth/Login';

export const Route = createFileRoute('/auth/login')({
    component: () => <LoginComponent
        message='Please login to use the application'
    />,
});