import { Alert, AppShell, Center, Group, Stack } from '@mantine/core'
import { createFileRoute, Link, Outlet } from '@tanstack/react-router'
import { Route as AboutRoute } from '@/routes/about';
import { Route as IndexRoute } from '@/routes';
import { Route as LegalRoute } from '@/routes/legal';

export const Route = createFileRoute('/auth/login')({
    component: RouteComponent,
})

function RouteComponent() {
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

export function UnauthorizedShell() {
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
                    justify="space-between"
                >
                    <Link
                        key="index"
                        to={ IndexRoute.to }
                        style={{
                            textDecoration: 'None',
                            color: 'var(--mantine-color-dark-0)'
                        }}
                    >
                        StarFoundry Industry
                    </Link>

                    <Link
                        key="about"
                        to={ AboutRoute.to }
                        style={{
                            textDecoration: 'None',
                            color: 'var(--mantine-color-dark-0)'
                        }}
                    >
                        About
                    </Link>
                </Group>
            </AppShell.Header>

            <AppShell.Main
                style={{
                    paddingLeft: 'var(--app-shell-padding)',
                }}
            >
                <Outlet />
            </AppShell.Main>

            <AppShell.Footer>
                <div
                    style={{
                        fontSize: '10px',
                    }}
                >
                    All

                    <Link
                        to={ LegalRoute.to }
                        style={{
                            color: 'var(--mantine-color-blue-9)',
                            padding: '5px',
                            textDecoration: 'none',
                        }}
                    >
                        Eve related materials
                    </Link>

                    are property of

                    <a
                        href="https://www.ccpgames.com"
                        target="_blank"
                        style={{
                            color: 'var(--mantine-color-blue-9)',
                            padding: '5px',
                            textDecoration: 'none',
                        }}
                    >
                        CCP Games
                    </a>

                    See

                    <Link
                        to={ LegalRoute.to }
                        style={{
                            color: 'var(--mantine-color-blue-9)',
                            paddingLeft: '5px',
                            textDecoration: 'none',
                        }}
                    >
                        legal notice
                    </Link>
                    .
                </div>
            </AppShell.Footer>
        </AppShell>
    </>
}
