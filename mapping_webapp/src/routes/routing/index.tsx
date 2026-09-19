import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/routing/')({
    component: RouteComponent,
})

function RouteComponent() {
    return <></>
}
