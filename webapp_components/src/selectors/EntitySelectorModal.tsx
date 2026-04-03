import { Button, Flex, Stack, UnstyledButton } from "@mantine/core";
import { EntityList, type Entity } from "@internal/list/EntityList";
import { InGameSearch, type InGameSearchRef } from "./InGameSearch";
import { useEffect, useRef, useState, type ReactElement } from "react";
import type { Category } from "@internal/services/utils";
import { ModalWrapper } from "@internal/wrapper/Modal";

export function EntitySelectorModal({
    opened,
    onClose,
    onSelect,

    categories = ['character', 'corporation', 'alliance'],

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

    return <ModalWrapper
        opened={opened}
        close={onClose}
        title="Characters / Corporations / Alliances"
    >
        <Stack>
            <InGameSearch
                categories={categories}
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
    </ModalWrapper>
}

export type EntitySelectorModalProp = {
    // modal control
    opened: boolean;
    onSelect: (entry: Entity[]) => void;
    onClose: () => void;

    categories?: Category[],

    // list of entities that are selected
    selected: Entity[],
}
