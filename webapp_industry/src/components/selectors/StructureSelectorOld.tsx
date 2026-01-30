import { LIST_STRUCTURE, type Structure } from "@/services/structure/list";
import type { Uuid } from "@/services/utils";
import { ActionIcon, Combobox, Flex, Group, Input, InputBase, Text, useCombobox } from "@mantine/core";
import { useState, type ReactElement } from "react";
import { EveIcon } from "../EveIcon";
import { faArrowsRotate } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useQueryClient } from "@tanstack/react-query";

function SelectOption(structure: Structure) {
    return (
        <Group key={structure.id}>
            <EveIcon
                id={structure.item.type_id}
            />

            <div>
                <Text fz="sm" fw={500}>
                    {structure.name}
                </Text>
                <Text fz="xs" opacity={0.6}>
                    {structure.system.region_name} - {structure.system.system_name}
                </Text>
            </div>
        </Group>
    );
}

export function StructureSelectorOld({
    onSelect,
    structures,
    selected = [],
    ref = {
        current: {
            reset: () => {}
        }
    },
    withLabel = false,
    required = false,
}: StructureSelectorProp): ReactElement {
    const queryClient = useQueryClient();

    const [value, setValue] = useState<string | null>(null);
    const [search, setSearch] = useState('');

    const combobox = useCombobox({
        onDropdownClose: () => {
            combobox.resetSelectedOption();
            combobox.focusTarget();
            setSearch('');
        },

        onDropdownOpen: () => {
            combobox.focusSearchInput();
        },
    });

    const resetValue = () => {
        setValue(null)
    };
    ref.current.reset = resetValue;

    const structureById = (
        id: string | null
    ): Structure | undefined => {
        if (id) {
            return (structures || []).find(x => x.id === id)
        } else {
            return undefined;
        }
    }

    const options = (structures || [])
        .filter((x: Structure) => {
            return x.name
                .toLocaleLowerCase()
                .indexOf(
                    search.toLocaleLowerCase()
                ) >= 0
        })
        .filter((x: Structure) => selected.indexOf(x.id) === -1)
        .map((item) => (
            <Combobox.Option
                value={item.id}
                key={item.id}
            >
                <SelectOption key={item.id} {...item} />
            </Combobox.Option>
        ));

    const refresh = () => {
        queryClient.invalidateQueries({
            queryKey: [LIST_STRUCTURE]
        })
    }

    return (
        <Flex
            direction="row"
            wrap="nowrap"
        >
            <Combobox
                store={combobox}
                withinPortal={false}
                onOptionSubmit={(val) => {
                    setValue(val);
                    onSelect(structures.find(x => x.id === val) as any);
                    combobox.closeDropdown();
                }}
                styles={{
                    dropdown: {
                        backgroundColor: 'var(--mantine-color-dark-7)'
                    },
                    search: {
                        backgroundColor: 'var(--mantine-color-dark-7)'
                    },
                }}
                position="bottom"
            >
                <Combobox.Target>
                    <InputBase
                        label={ withLabel ? "Structure Group" : null }
                        description={withLabel ? "Select a structure group" : null}
                        component="button"
                        type="button"
                        withErrorStyles={false}
                        rightSection={<Combobox.Chevron />}
                        onClick={() => combobox.toggleDropdown()}
                        rightSectionPointerEvents="none"
                        pointer
                        style={{
                            width: '100%',
                        }}
                        withAsterisk={required}
                        inputContainer={(children) => {
                            return <Flex>
                                <div style={{ width: '100%' }}>
                                    { children }
                                </div>

                                <ActionIcon
                                    size="input-sm"
                                    color="gray"
                                    onClick={refresh}
                                    style={{
                                        marginTop: withLabel ? 'calc(var(--mantine-spacing-xs) / 2)' : 0
                                    }}
                                >
                                    <FontAwesomeIcon icon={faArrowsRotate} />
                                </ActionIcon>
                            </Flex>
                        }}
                    >
                        {
                            structureById(value)?.name ||
                            <Input.Placeholder>Select a structure</Input.Placeholder>
                        }
                    </InputBase>
                </Combobox.Target>

                <Combobox.Dropdown>
                    <Combobox.Search
                        value={search}
                        onChange={(event) => setSearch(event.currentTarget.value)}
                        placeholder="Search structures"
                    />
                    <Combobox.Options
                        mah={300}
                        style={{ overflowY: 'auto' }}
                    >
                        {
                            options.length > 0
                                ? options
                                : <Combobox.Empty>Nothing found</Combobox.Empty>
                        }
                    </Combobox.Options>
                </Combobox.Dropdown>
            </Combobox>
        </Flex>
    )
}

export type StructureSelectorProp = {
    onSelect: (entry: Structure) => void;

    // list of structures that should be shown
    structures:     Structure[],

    // list of values that are already selected, and filtered out
    selected?:      Uuid[],
    // allows for additional controls of the component
    ref?:           { current: StructureSelectorRef },

    // shows a label and description
    withLabel?:     boolean;
    // adds an asterisk
    required?:      boolean;

}

export type StructureSelectorRef = {
    reset: () => void,
}
