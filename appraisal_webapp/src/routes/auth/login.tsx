import { LoginComponent } from '@starfoundry/components/auth/Login';
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/auth/login')({
    component: LoginComponent,
});
