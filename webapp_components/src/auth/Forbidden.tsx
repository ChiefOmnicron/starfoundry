import { Alert, Center, Stack } from "@mantine/core";

export function ForbiddenComponent() {
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
