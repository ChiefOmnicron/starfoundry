import { Alert, Center, Image, Stack } from '@mantine/core'

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
