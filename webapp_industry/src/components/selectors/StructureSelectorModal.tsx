import { Button, Flex, InputBase, Modal, Stack, UnstyledButton } from "@mantine/core";
import { StructureList } from "../StructureCard";
import { useEffect, useState, type ReactElement } from "react";
import type { Structure } from "@/services/structure/list";

export function StructureSelectorModal({
    opened,
    onClose,
    onSelect,

    structures,

    selected,
}: StructureSelectorModalProp): ReactElement {
    // all structures selected by the user
    const [selectedStructures, setSelectedStructures] = useState<Structure[]>([]);
    const [_, setSearch] = useState('');

    useEffect(() => {
        setSelectedStructures(selected);
    }, [opened, selected]);

    const structureList = () => {
        return <StructureList
            structures={structures}
            groupBySystem={false}

            structureCardProps={{
                checkable: true,
                checked: selectedStructures,
                onChange: (event: 'checked' | 'unchecked', structure: Structure) => {
                    setSelectedStructures(
                        event === 'checked'
                            ? [...selectedStructures, structure]
                            : selectedStructures.filter((y) => y.id !== structure.id)
                    );
                }
            }}
        />
    }

    const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
        setSearch(event.currentTarget.value);
    };

    return <Modal
        opened={opened}
        onClose={onClose}
        title="Structures"
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
            <InputBase
                name="Name"
                description='Search for the name of the structure'
                placeholder="Jita 4-4"
                onChange={handleSearch}
            ></InputBase>

            { structureList() }

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
                    onClick={() => onSelect(selectedStructures)}
                >
                    Select ({ selectedStructures.length }) structures
                </Button>
            </Flex>
        </Stack>
    </Modal>
}

export type StructureSelectorModalProp = {
    // modal control
    opened: boolean;
    onSelect: (entry: Structure[]) => void;
    onClose: () => void;

    // structures the user can select
    structures: Structure[],
    // list of values that are already selected
    selected:   Structure[],

    blueprint?: boolean;
    buildable?: boolean;
}
