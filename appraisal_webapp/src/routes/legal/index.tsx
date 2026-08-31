import { LegalComponent } from '@starfoundry/components/misc/Legal';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/legal/')({
    component: LegalComponent,
});
