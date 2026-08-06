import { CreateRoute } from '@starfoundry/components/routes/Create'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/routes/')({
    component: RouteComponent,
})

function RouteComponent() {
    return <>
        <CreateRoute />
    </>
}
