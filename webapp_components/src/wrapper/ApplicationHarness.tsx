import { AppShell, Burger, Divider, Group, ScrollArea, Text } from "@mantine/core";
import { CharacterComponent } from "../misc/Character";
import { CustomLink } from "../links/RouterLink";
import { Footer } from "../misc/Footer";
import { Link, Outlet, useRouterState } from "@tanstack/react-router";
import { useDisclosure } from "@mantine/hooks";
import type { ReactElement } from "react";

export function ApplicationShell({
    name,

    aboutRoute,
    indexRoute,
    legalRoute,

    showSidebar = true,
    isAuthenticated = 'UNAUTHENTICATED',

    routes = [],
}: ApplicationShellProps) {
    const router = useRouterState();
    const [opened, { toggle }] = useDisclosure();

    const navigation = (): ReactElement[] => {
        const groups = routes
            .map(group => {
                return group.groups
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
        if (isAuthenticated === 'AUTHENTICATED') {
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

    return <>
        <AppShell
            header={{ height: 60 }}
            navbar={{
                width: 225,
                breakpoint: 'sm',
                collapsed: {
                    mobile: !opened
                },
            }}
            padding="md"
        >
            <AppShell.Header>
                <Group
                    h="100%"
                    px="md"
                    justify="space-between"
                >
                    {
                        showSidebar
                        ? <Burger
                                opened={opened}
                                onClick={toggle}
                                hiddenFrom="sm"
                                size="sm"
                            />
                        :   <></>
                    }

                    <Link
                        key="index"
                        to={indexRoute}
                        style={{
                            textDecoration: 'None',
                            color: 'var(--mantine-color-dark-0)'
                        }}
                    >
                        <Text
                            fw={700}
                            size="xl"
                        >
                                {name}
                        </Text>
                    </Link>

                    <Link
                        key="about"
                        to={aboutRoute}
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
                    paddingLeft: showSidebar ? undefined : 'var(--app-shell-padding)',
                    paddingBottom: '10%'
                }}
            >
                <Outlet />
            </AppShell.Main>

            {
                showSidebar
                ?   sideNavigation()
                :   <></>
            }

            <AppShell.Footer>
                <Footer
                    legalRoute={legalRoute}
                />
            </AppShell.Footer>
        </AppShell>
    </>
}

export type ApplicationShellProps = {
    name: string,

    aboutRoute: any;
    indexRoute: any;
    legalRoute: any;

    showSidebar?: boolean;
    isAuthenticated?: 'AUTHENTICATED' | 'UNAUTHENTICATED';
    routes?: ApplicationShellRouteDefinition[];
}

export type ApplicationShellRouteDefinition = {
    groups: ApplicationShellRouteGroup[];
}

export type ApplicationShellRouteGroup = {
    link: string;
    label: string;
    subpath?: string;
    paths: { link: string, label: string }[];
}
