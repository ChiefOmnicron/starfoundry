import { AppShell, Burger, createTheme, DEFAULT_THEME, Divider, Group, Image, MantineProvider, mergeMantineTheme, ScrollArea } from '@mantine/core';
import { CharacterComponent } from '@starfoundry/components/misc/Character';
import { createRootRouteWithContext, Link, Outlet, useRouterState } from '@tanstack/react-router';
import { CustomLink } from '@starfoundry/components/links/RouterLink';
import { Footer } from '@starfoundry/components/misc/Footer';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { Route as AboutRoute } from '@/routes/about';
import { Route as IndexRoute } from '@/routes';
import { Route as LegalRoute } from '@/routes/legal';
import { UnauthorizedShell } from './auth/login';
import { useDisclosure } from '@mantine/hooks';
import type { ReactElement } from 'react';
import type { RouterContext } from '@/main';

type RouteDefinition = {
    entries: RouteEntry[];
}

type RouteEntry = {
    link: string;
    label: string;
    subpath?: string;
    paths: { link: string, label: string }[];
}

const routes: RouteDefinition[] = [{
    entries: [{
        link: '/projects',
        label: 'Projects',
        subpath: '/projects/$projectId',
        paths: [{
            link: '/projects/$projectId/overview',
            label: 'Overview'
        }, {
            link: '/projects/$projectId/misc',
            label: 'Miscellaneous'
        }, {
            link: '/projects/$projectId/market',
            label: 'Market'
        }, {
            link: '/projects/$projectId/jobs',
            label: 'Jobs'
        }]
    }, {
        link: '/project-groups',
        label: 'Project Groups',
        subpath: '/project-groups/$projectGroupId',
        paths: [{
            link: '/project-groups/$projectGroupId/overview',
            label: 'Overview'
        }, {
            link: '/project-groups/$projectGroupId/projects',
            label: 'Projects'
        }, {
            link: '/project-groups/$projectGroupId/members',
            label: 'Members'
        }, {
            link: '/project-groups/$projectGroupId/industry-hubs',
            label: 'Industry Hubs'
        }, {
            link: '/project-groups/$projectGroupId/defaults',
            label: 'Defaults'
        }]
    }]
}, {
    entries: [{
        link: '/industry-hubs',
        label: 'Industry Hubs',
        paths: [],
    }, {
        link: '/structures',
        label: 'Structures',
        paths: [],
    }]
}, {
    entries: [{
        link: '/characters',
        label: 'Characters',
        paths: [],
    }]
}];

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
            isAuthenticated: await context.auth.isAuthenticated()
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
    const [opened, { toggle }] = useDisclosure();

    const navigation = (): ReactElement[] => {
        const groups = routes
            .map(group => {
                return group.entries
                    .map(route => {
                        if (!(route && route.label)) {
                            return <></>;
                        }

                        if (
                            router.matches[router.matches.length - 1] &&
                            router.matches[router.matches.length - 1].fullPath.indexOf(route.subpath || '') > -1
                        ) {
                            const match = router.matches[router.matches.length - 2];

                            if (!match) {
                                return <></>;
                            }

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
                                    rightSection={<></> }
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
            });

        return groups
            .map((x, index) => <>
                { x }

                {
                    index === groups.length -1
                    ? <></>
                    : <Divider />
                }
            </>)
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

    return (
        <MantineProvider
            forceColorScheme='dark'
            theme={theme}
        >
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
                            <Image
                                src="/sf_logo.png"
                                h="calc(3rem * var(--mantine-scale))"
                                w="auto"
                                fit="contain"
                            />
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
                        paddingBottom: '10%'
                    }}
                >
                    <Outlet />

                    <ReactQueryDevtools />
                </AppShell.Main>

                <Footer
                    legalRoute={LegalRoute.to}
                />
            </AppShell>
        </MantineProvider>
    );
}
