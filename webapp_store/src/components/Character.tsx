import { Avatar, Group, Text, UnstyledButton } from "@mantine/core";
import { LoadingAnimation } from "./LoadingAnimation";
import { LoadingError } from "./LoadingError";
import { useWhoami } from "@/services/auth/whoami";

export function CharacterComponent() {
    const {
        isPending,
        isError,
        data: character,
    } = useWhoami();

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    return <>
        <UnstyledButton>
            <Group>
                <Avatar
                    src={`https://images.evetech.net/characters/${character.character_id}/portrait`}
                    radius="xl"
                />

                <div style={{ flex: 1 }}>
                    <Text size="sm" fw={500}>
                        { character.character_name }
                    </Text>

                    <Text c="dimmed" size="xs">
                        { character.corporation_name }
                    </Text>
                </div>
            </Group>
        </UnstyledButton>
    </>
}
