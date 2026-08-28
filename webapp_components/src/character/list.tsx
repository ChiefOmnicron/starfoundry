import { CharacterTable } from "../list/CharacterList";
import { LoadingAnimation } from "../misc/LoadingAnimation";
import { LoadingError } from "../misc/LoadingError";
import { useListCharacters } from "../services/character/list";
import { Button, Group, Stack } from "@mantine/core";
import { axiosClient } from "../services/client";

export function CharacterList() {
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

            <CharacterTable
                characters={characters || []}
            />
        </Stack>
    </>
}
