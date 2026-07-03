import { createFileRoute } from '@tanstack/react-router'
import { Tabs, Title } from '@mantine/core';
import { CharacterList } from '@starfoundry/components/character/list';

export const Route = createFileRoute('/characters/')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
});

function RouteComponent() {
    return <>
        <Title order={1}>Characters</Title>

        <Tabs defaultValue="login">
            <Tabs.List>
                <Tabs.Tab value="login">
                    Characters
                </Tabs.Tab>
                <Tabs.Tab value="jobs">
                    Industry Job Configuration
                </Tabs.Tab>
            </Tabs.List>

            <Tabs.Panel value="login">
                <CharacterList />
            </Tabs.Panel>

            <Tabs.Panel value="jobs">
                TODO
            </Tabs.Panel>
        </Tabs>
    </>
}
