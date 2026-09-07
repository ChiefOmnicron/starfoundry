import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/jobs_/$assignmentId/')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/jobs_/$assignmentId/"!</div>
}
