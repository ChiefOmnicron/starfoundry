import { Button, Flex, InputBase, Stack, UnstyledButton } from "@mantine/core";
import { StructureList } from "../list/StructureList";
import { useEffect, useState, type ReactElement } from "react";
import type { Structure } from "@internal/services/structure/list";
import { ModalWrapper } from "@internal/wrapper/Modal";

export function StructureSelectorModal({
    opened,
    onClose,
    onSelect,

    multiple = false,

    structures,

    selected,
}: StructureSelectorModalProp): ReactElement {
    // all structures selected by the user
    const [selectedStructures, setSelectedStructures] = useState<Structure[]>([]);
    const [search, setSearch] = useState('');

    useEffect(() => {
        setSelectedStructures(selected);
    }, [opened, selected]);

    const structureList = <StructureList
        structures={structures}
        groupBySystem={false}

        filter={{
            search,
        }}
        multiple={multiple}

        selectedStructures={selectedStructures}
        onSelect={setSelectedStructures}
    />

    const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
        setSearch(event.currentTarget.value);
    };

    return <ModalWrapper
        opened={opened}
        close={onClose}
        title="Structures"
    >
        <Stack>
            <InputBase
                name="Name"
                description='Search for the name or system of the structure'
                placeholder="Jita 4-4"
                onChange={handleSearch}
            ></InputBase>

            { structureList }

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
    </ModalWrapper>
}

export type StructureSelectorModalProp = {
    // modal control
    opened: boolean;
    onSelect: (entry: Structure[]) => void;
    onClose: () => void;

    multiple?: boolean;

    // structures the user can select
    structures: Structure[],
    // list of values that are already selected
    selected:   Structure[],
}
