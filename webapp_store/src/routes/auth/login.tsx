import { Alert, AppShell, Center, Group, Stack } from '@mantine/core'
import { createFileRoute } from '@tanstack/react-router'
import { useGeneralInformation } from '@/services/general/fetch'

export const Route = createFileRoute('/auth/login')({
    component: LoginComponent,
})

export function LoginComponent() {
    return <>
        <Center>
            <Stack>
                <Alert
                    variant="light"
                    color="blue"
                    title="Please Login"
                >
                    Please login to see all offerings
                </Alert>

                <img
                    onClick={() => { window.location.href = "/api/auth/login"}}
                    src="https://web.ccpgamescdn.com/eveonlineassets/developers/eve-sso-login-black-large.png"
                    style={{
                        cursor: 'pointer'
                    }}
                />
            </Stack>
        </Center>
    </>
}

export function UnauthorizedShell() {
    const {
        data: generalInformation
    } = useGeneralInformation();

    const storeName = () => {
        if (generalInformation) {
            return generalInformation.name;
        } else {
            return 'StarFoundry Store';
        }
    }

    return <>
        <AppShell
            header={{ height: 60 }}
            navbar={{
                width: 225,
                breakpoint: 'sm',
            }}
            padding="md"
        >
            <AppShell.Header>
                <Group
                    h="100%"
                    px="md"
                    justify="flex-start"
                >
                    { storeName() }
                </Group>
            </AppShell.Header>

            <AppShell.Main
                style={{
                    paddingLeft: 'var(--app-shell-padding)',
                }}
            >
                { LoginComponent() }
            </AppShell.Main>
        </AppShell>
    </>
}
