import { AppShell, Avatar, Burger, createTheme, DEFAULT_THEME, Group, MantineProvider, mergeMantineTheme, ScrollArea, Text, Title, UnstyledButton } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { CustomLink } from '@/components/RouterLink';

import { createRootRouteWithContext, Outlet, useRouterState } from '@tanstack/react-router';
import type { ReactElement } from 'react';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import type { RouterContext } from '@/main';

import { Route as ProjectGroupRoute } from '@/routes/project-groups_/$projectGroupId.route';
import { Route as ProjectGroupOverviewRoute } from '@/routes/project-groups_/$projectGroupId.overview';
import { Route as ProjectGroupProjectsRoute } from '@/routes/project-groups_/$projectGroupId.projects';

const routes = [
    {
        link: '/project-groups',
        label: 'Project Groups',
        subpath: ProjectGroupRoute.to,
        paths: [{
            link: ProjectGroupOverviewRoute.to,
            label: 'Overview'
        }, {
            link: ProjectGroupProjectsRoute.to,
            label: 'Projects'
        }, {
            link: '/project-groups/$projectGroupId/structures',
            label: 'Structures'
        }, {
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
});

function LayoutComponent(): ReactElement {
    const [opened, { toggle }] = useDisclosure();

    const router = useRouterState();

    const projectGroups = (): ReactElement[] => {
        return routes
            .map(route => {
                if (router.matches[router.matches.length - 2].fullPath === route.subpath) {
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

                <AppShell.Main>
                    <Outlet />

                    <ReactQueryDevtools />
                </AppShell.Main>
            </AppShell>
        </MantineProvider>
    );
}
