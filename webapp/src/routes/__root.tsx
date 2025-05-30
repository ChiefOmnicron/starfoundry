import { AppShell, Avatar, Burger, createTheme, DEFAULT_THEME, Group, MantineProvider, mergeMantineTheme, ScrollArea, Text, Title, UnstyledButton } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { CustomLink } from '@/components/RouterLink';

import { createRootRoute, Outlet } from '@tanstack/react-router';
import type { ReactElement } from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';

const routes = [
    { link: '/project-groups', label: 'Project Groups' },
];

const themeOverride = createTheme({
    fontFamily: '"Roboto Mono", monospace',
    fontFamilyMonospace: '"Roboto Mono", monospace',
    radius: {
        lg: "0",
        md: "0",
        sm: "0",
        xl: "0",
        xs: "0"
    },
});

const theme = mergeMantineTheme(DEFAULT_THEME, themeOverride);
const queryClient = new QueryClient();

export const Route = createRootRoute({
    component: LayoutComponent,
});

function LayoutComponent(): ReactElement {
    console.log(theme)
    const [opened, { toggle }] = useDisclosure();

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
                        { navEntries() }
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

                <AppShell.Main m="md">
                    <QueryClientProvider client={queryClient}>
                        <Outlet />

                        <ReactQueryDevtools />
                    </QueryClientProvider>
                </AppShell.Main>
            </AppShell>
        </MantineProvider>
    );
}

function navEntries(): ReactElement[] {
    return routes
        .map(x => {
            return (
                <CustomLink
                    key={ x.label.toLowerCase() }
                    to={ x.link }
                    label={ x.label }
                />
            )
        })
}
