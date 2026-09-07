import { Alert, Center, Image, Stack } from '@mantine/core'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/auth/login')({
    component: LoginComponent,
})

function LoginComponent() {
    return <>
        <Center>
            <Stack>
                <Alert
                    variant="light"
                    color="blue"
                    title="Login"
                >
                    Please login
                </Alert>

                <Center>
                    <Image
                        onClick={() => { window.location.href = "/api/auth/login"}}
                        src="https://web.ccpgamescdn.com/eveonlineassets/developers/eve-sso-login-black-large.png"
                        w={270}
                        style={{
                            cursor: 'pointer'
                        }}
                    />
                </Center>
            </Stack>
        </Center>
    </>
}
