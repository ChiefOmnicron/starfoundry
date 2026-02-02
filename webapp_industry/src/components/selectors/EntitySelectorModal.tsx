import { Button, Flex, Modal, Stack, UnstyledButton } from "@mantine/core";
import { CharacterCorporationAllianceList, type CharacterCorporationAlliance } from "../EntityList";
import { InGameSearch, type InGameSearchRef } from "./InGameSearch";
import { useEffect, useRef, useState, type ReactElement } from "react";

export function EntitySelectorModal({
    opened,
    onClose,
    onSelect,

    selected,
}: EntitySelectorModalProp): ReactElement {
    // all structures selected by the user
    const [selectedEntities, setSelectedEntities] = useState<CharacterCorporationAlliance[]>([]);

    const inGameSearchRef = useRef<InGameSearchRef>({} as any);

    useEffect(() => {
        setSelectedEntities(selected);
    }, [opened, selected]);

    const entityList = () => {
        return <CharacterCorporationAllianceList
            characterCorporationAlliances={selectedEntities}

            //characterCorporationAllianceCardProps={{
            //    checkable: true,
            //    checked: selectedEntities,
            //    onChange: (event: 'checked' | 'unchecked', entity: CharacterCorporationAlliance) => {
            //        setSelectedEntities(
            //            event === 'checked'
            //                ? [...selectedEntities, entity]
            //                : selectedEntities.filter((y) => y.id !== entity.id)
            //        );
            //    }
            //}}
        />
    }

    //const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
    //    setSearch(event.currentTarget.value);
    //};

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
                        x as CharacterCorporationAlliance,
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
    onSelect: (entry: CharacterCorporationAlliance[]) => void;
    onClose: () => void;

    // structures the user can select
    entities: CharacterCorporationAlliance[],
    // list of values that are already selected
    selected: CharacterCorporationAlliance[],
}
