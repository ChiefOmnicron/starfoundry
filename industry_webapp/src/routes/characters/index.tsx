import { axiosClient } from '@starfoundry/components/services/client';
import { CharacterList } from '@starfoundry/components/list/CharacterList';
import { createFileRoute } from '@tanstack/react-router'
import { Button, Group, Stack, Title } from '@mantine/core';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { useListCharacters } from '@starfoundry/components/services/character/list';

export const Route = createFileRoute('/characters/')({
    beforeLoad: async ({ context }) => {
        if (!(await context.auth.isAuthenticated())) {
            throw context.auth.login();
        }
    },
    component: RouteComponent,
});

function RouteComponent() {
    const {
        isPending,
        isError,
        data: characters,
    } = useListCharacters();

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const loginCorporation = async () => {
        (await axiosClient())
            .get('/api/auth/login/corporation')
            .then((x: any) => {
                window.location.href = x.data.url;
            })
            .catch(e => {
                console.error(e)
            })
    }
    const loginCharacter = async () => {
        (await axiosClient())
            .get('/api/auth/login/character')
            .then((x: any) => {
                window.location.href = x.data.url;
            })
            .catch(e => {
                console.error(e)
            })
    }

    return <>
        <Title order={1}>Characters</Title>

        <Stack>
            <Group
                justify="flex-end"
            >
                <Button
                    onClick={loginCharacter}
                >
                    Login Character
                </Button>
                <Button
                    onClick={loginCorporation}
                >
                    Login Corporation
                </Button>
            </Group>

            <CharacterList
                characters={characters || []}
            />
        </Stack>
    </>
}
