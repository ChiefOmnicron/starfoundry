import { AppShell, Avatar, Burger, Group, ScrollArea, Text, Title, UnstyledButton } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { CustomLink } from '@/components/RouterLink';

import { Outlet } from '@tanstack/react-router';
import type { ReactElement } from 'react';

const routes = [
    { link: '/structures', label: 'Structures' },
];

export default function Shell(): ReactElement {
  const [opened, { toggle }] = useDisclosure();

  return (
    <AppShell
        header={{ height: 60 }}
        navbar={{
            width: 200,
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

        <AppShell.Main>
            <Outlet />
        </AppShell.Main>
    </AppShell>
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
