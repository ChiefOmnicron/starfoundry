import { Avatar, Group, Text, UnstyledButton } from "@mantine/core";
import { LoadingError } from "./LoadingError";
import { characterInfo, type CharacterInfo } from "@internal/services/client";

export function CharacterComponent() {
    try {
        const character = characterInfo();
        return showCharacterInfo(character);
    } catch(_) {
        return LoadingError();
    }
}

function showCharacterInfo(character: CharacterInfo) {
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
