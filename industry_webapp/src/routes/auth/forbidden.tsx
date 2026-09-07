import { Alert, Center, Stack } from '@mantine/core'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/auth/forbidden')({
    component: LoginComponent,
})

function LoginComponent() {
    return <>
        <Center>
            <Stack>
                <Alert
                    variant="light"
                    color="red"
                    title="Forbidden"
                >
                    Character Is Not on the Whitelist. This Incident Will Be Reported.
                </Alert>
            </Stack>
        </Center>
    </>
}
