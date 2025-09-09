import { AppShell, Burger, createTheme, DEFAULT_THEME, Flex, Group, MantineProvider, mergeMantineTheme, ScrollArea } from '@mantine/core';
import { CharacterComponent } from '@/components/Character';
import { createRootRouteWithContext, Link, Outlet, useRouterState } from '@tanstack/react-router';
import { CustomLink } from '@/components/RouterLink';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { Route as AboutRoute } from '@/routes/about';
import { Route as IndexRoute } from '@/routes';
import { Route as LegalRoute } from '@/routes/legal';
import { UnauthorizedShell } from './auth/login';
import { useDisclosure } from '@mantine/hooks';
import { useGeneralInformation } from '@/services/general/fetch';
import type { ReactElement } from 'react';
import type { RouterContext } from '@/main';

const themeOverride = createTheme({
    fontFamily: '"Roboto Mono", monospace',
    fontFamilyMonospace: '"Roboto Mono", monospace',
    radius: {
        lg: '0',
        md: '0',
        sm: '0',
        xl: '0',
        xs: '0',
    },
});

const theme = mergeMantineTheme(DEFAULT_THEME, themeOverride);

export const Route = createRootRouteWithContext<RouterContext>()({
    component: LayoutComponent,
    loader: async ({ context }) => {
        return {
            isAuthenticated: await context.auth.isAuthenticated(),
            isAdmin: await context.auth.isAdmin(),
        }
    }
});

function LayoutComponent(): ReactElement {
    const { isAuthenticated } = Route.useLoaderData();

    const shell = () => {
        if (isAuthenticated) {
            return Shell();
        } else {
            return UnauthorizedShell();
        }
    }

    return (
        <MantineProvider
            forceColorScheme='dark'
            theme={theme}
        >
            { shell() }
        </MantineProvider>
    );
}

function Shell() {
    const router = useRouterState();
    const { isAdmin } = Route.useLoaderData();
    const [opened, { toggle }] = useDisclosure();

    const {
        data: generalInformation
    } = useGeneralInformation();

    const routes = [
        {
            link: '/products',
            label: 'Products',
            //subpath: '/project-groups/$projectGroupId',
            paths: [],
        },
        {
            link: '/orders',
            label: 'Orders',
            //subpath: '/project-groups/$projectGroupId',
            paths: [],
        },
        {
            link: '/contact',
            label: 'Contact',
            paths: [],
        },
        isAdmin
            ? {
                link: '/admin',
                label: 'Admin',
                subpath: '/admin',
                paths: [{
                    link: '/admin/products/',
                    label: 'Products'
                }, {
                    link: '/admin/orders/',
                    label: 'Orders'
                }],
            }
            : {},
    ];

    const navigation = (): ReactElement[] => {
        return routes
            .map(route => {
                if (!(route && route.label)) {
                    return <></>;
                }

                if (
                    router.matches[router.matches.length - 1] &&
                    router.matches[router.matches.length - 1].fullPath.indexOf(route.subpath || '') > -1
                ) {
                    const match = router.matches[router.matches.length - 2];
                    const params: any = match.params;

                    const subRoutes = route
                        .paths
                        .map(subRoute => {
                            return (<CustomLink
                                key={ subRoute.label.toLowerCase() }
                                to={ subRoute.link }
                                label={ subRoute.label }
                                params={params}
                            />)
                        });

                    return (
                        <CustomLink
                            key={ route.label.toLowerCase() }
                            to={ route.link }
                            label={ route.label }
                            rightSection={ <></> }
                            opened
                        >
                            { subRoutes }
                        </CustomLink>
                    )
                } else {
                    return (
                        <CustomLink
                            key={ route.label.toLowerCase() }
                            to={ route.link }
                            label={ route.label }
                        />
                    );
                }
            });
    }

    const sideNavigation = (): ReactElement => {
        const { isAuthenticated } = Route.useLoaderData();

        if (isAuthenticated) {
            return (
                <AppShell.Navbar>
                    <AppShell.Section grow my="md" component={ScrollArea}>
                        { navigation() }
                    </AppShell.Section>


                    <AppShell.Section>
                        <CharacterComponent />
                    </AppShell.Section>
                </AppShell.Navbar>
            );
        } else {
            return <></>
        }
    }

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
                width: 250,
                breakpoint: 'sm',
                collapsed: {
                    mobile: !opened
                },
            }}
            padding='md'
        >
            <AppShell.Header>
                <Group
                    h="100%"
                    px="md"
                    justify="space-between"
                >
                    <Burger
                        opened={opened}
                        onClick={toggle}
                        hiddenFrom="sm"
                        size="sm"
                    />

                    <Link
                        key="index"
                        to={ IndexRoute.to }
                        style={{
                            textDecoration: 'None',
                            color: 'var(--mantine-color-dark-0)'
                        }}
                    >
                        { storeName() }
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

            { sideNavigation() }

            <AppShell.Main
                style={{
                    paddingBottom: '50px'
                }}
            >
                <Outlet />

                <ReactQueryDevtools />
            </AppShell.Main>

            <AppShell.Footer>
                <Flex
                    justify="flex-end"
                >
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
                </Flex>
            </AppShell.Footer>
        </AppShell>
    </>
}
