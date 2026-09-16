import { createFileRoute } from '@tanstack/react-router'
import { ForbiddenComponent } from '@starfoundry/components/auth/Forbidden';

export const Route = createFileRoute('/auth/forbidden')({
    component: ForbiddenComponent,
})
