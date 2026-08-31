import { Alert, Center, Stack } from '@mantine/core'

export function LoginComponent() {
    return <>
        <Center>
            <Stack>
                <Alert
                    variant="light"
                    color="blue"
                    title="Please Login"
                >
                    Please login to use the application
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
