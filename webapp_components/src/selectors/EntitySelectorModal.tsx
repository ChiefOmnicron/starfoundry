import { Button, Flex, Modal, Stack, UnstyledButton } from "@mantine/core";
import { EntityList, type Entity } from "@internal/list/EntityList";
import { InGameSearch, type InGameSearchRef } from "./InGameSearch";
import { useEffect, useRef, useState, type ReactElement } from "react";

export function EntitySelectorModal({
    opened,
    onClose,
    onSelect,

    selected,
}: EntitySelectorModalProp): ReactElement {
    // all structures selected by the user
    const [selectedEntities, setSelectedEntities] = useState<Entity[]>([]);

    const inGameSearchRef = useRef<InGameSearchRef>({} as any);

    useEffect(() => {
        setSelectedEntities(selected);
    }, [opened, selected]);

    const entityList = () => {
        return <EntityList
            entities={selectedEntities}

            entityCardProps={{
                editable: true,
                onChange: (event: 'remove', entity: Entity) => {
                    if (event === 'remove') {
                        setSelectedEntities(
                            selectedEntities.filter(x => x.id !== entity.id)
                        );
                    }
                }
            }}
        />
    }

    return <Modal
        opened={opened}
        onClose={onClose}
        title="Characters / Corporations / Alliances"
        overlayProps={{
            backgroundOpacity: 0.55,
            blur: 3,
        }}
        size="70%"
        centered
        closeOnEscape
        closeOnClickOutside
    >
        <Stack>
            <InGameSearch
                categories={['character', 'corporation', 'alliance']}
                ref={inGameSearchRef}
                onSelect={(x) => {
                    setSelectedEntities([
                        x as Entity,
                        ...selectedEntities,
                    ]);
                    inGameSearchRef.current.reset();
                }}

                withinPortal
            />

            {entityList()}

            <Flex
                justify='flex-end'
                gap='xs'
            >
                <UnstyledButton
                    onClick={onClose}
                >
                    Close without change
                </UnstyledButton>
                <Button
                    onClick={() => onSelect(selectedEntities)}
                >
                    Select ({ selectedEntities.length }) entities
                </Button>
            </Flex>
        </Stack>
    </Modal>
}

export type EntitySelectorModalProp = {
    // modal control
    opened: boolean;
    onSelect: (entry: Entity[]) => void;
    onClose: () => void;

    // list of entities that are selected
    selected: Entity[],
}
