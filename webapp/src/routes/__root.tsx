import { AppShell, Avatar, Burger, createTheme, DEFAULT_THEME, Group, MantineProvider, mergeMantineTheme, ScrollArea, Text, Title, UnstyledButton } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { CustomLink } from '@/components/RouterLink';

import { createRootRouteWithContext, Outlet, useLoaderData, useRouterState } from '@tanstack/react-router';
import type { ReactElement } from 'react';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import type { RouterContext } from '@/main';

const routes = [
    {
        link: '/project-groups',
        label: 'Project Groups',
        subpath: '/project-groups/$projectGroupId',
        paths: [{
            link: '/project-groups/$projectGroupId/overview',
            label: 'Overview'
        },
        // TODO: implement
        /*{
            link: '/project-groups/$projectGroupId/projects',
            label: 'Projects'
        }, {
            link: '/project-groups/$projectGroupId/structures',
            label: 'Structures'
        }, */{
            link: '/project-groups/$projectGroupId/members',
            label: 'Members'
        }, {
            link: '/project-groups/$projectGroupId/defaults',
            label: 'Defaults'
        }]
    },
];

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
    const [opened, { toggle }] = useDisclosure();

    const router = useRouterState();

    const projectGroups = (): ReactElement[] => {
        return routes
            .map(route => {
                if (router.matches[router.matches.length - 2] && router.matches[router.matches.length - 2].fullPath === route.subpath) {
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
                }

                return (
                    <CustomLink
                        key={ route.label.toLowerCase() }
                        to={ route.link }
                        label={ route.label }
                    />
                );
            });
    }

    const sideNavigation = (): ReactElement => {
        const { isAuthenticated } = Route.useLoaderData();

        if (isAuthenticated) {
            return (
                <AppShell.Navbar>
                    <AppShell.Section grow my="md" component={ScrollArea}>
                        { projectGroups() }
                    </AppShell.Section>
                    <AppShell.Section>
                        <UnstyledButton>
                            <Group>
                                <Avatar
                                    src="https://raw.githubusercontent.com/mantinedev/mantine/master/.demo/avatars/avatar-8.png"
                                    radius="xl"
                                />

                                <div style={{ flex: 1 }}>
                                    <Text size="sm" fw={500}>
                                        John Doe
                                    </Text>

                                    <Text c="dimmed" size="xs">
                                        john.doe@example.com
                                    </Text>
                                </div>
                            </Group>
                        </UnstyledButton>
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
                    width: 225,
                    breakpoint: 'sm',
                    collapsed: { mobile: !opened },
                }}
                padding="md"
            >
                <AppShell.Header>
                    <Group
                        h="100%"
                        px="md"
                        justify="flex-start"
                    >
                        <Burger
                            opened={opened}
                            onClick={toggle}
                            hiddenFrom="sm"
                            size="sm"
                        />
                        <Title order={1}>StarFoundry</Title>
                    </Group>
                </AppShell.Header>

                { sideNavigation() }

                <AppShell.Main>
                    <Outlet />

                    <ReactQueryDevtools />
                </AppShell.Main>
            </AppShell>
        </MantineProvider>
    );
}
