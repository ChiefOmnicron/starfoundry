import { useListStructure, type Structure, type StructureFilter } from "@/services/structure/list";
import type { Uuid } from "@/services/utils";
import { Button, Combobox, Group, Input, InputBase, Text, TextInput, useCombobox } from "@mantine/core";
import { useState, type ReactElement } from "react";
import { EveIcon } from "./EveIcon";

function SelectOption(structure: Structure) {
  return (
    <Group key={structure.id}>
        <EveIcon
            id={structure.structure.type_id}
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

export function StructureSelector({
    onSelect,
    selected = [],
    filters = {},
}: StructureSelectorProp): ReactElement {
    const {
        isError,
        isPending,
        data: structures,
    } = useListStructure(filters);

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

    if (isPending) {
        return <TextInput
            label="Select Structure"
            disabled
            rightSection={
                <Button
                    data-cy="loadingStructures"
                    loaderProps={{ type: 'oval' }}
                    loading
                    style={{
                        border: 'none',
                        backgroundColor: 'transparent'
                    }}
                />
            }
        />
    }

    const [value, setValue] = useState<string | null>(null);
    const [search, setSearch] = useState('');

    const structureById = (
        id: string | null
    ): Structure | undefined => {
        if (id) {
            return structures.find(x => x.id === id)
        } else {
            return undefined;
        }
    }

    const options = structures
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

    return (
        <Combobox
            store={combobox}
            withinPortal={false}
            onOptionSubmit={(val) => {
                setValue(val);
                onSelect(val);
                combobox.closeDropdown();
            }}
            styles={{
                dropdown: {
                    backgroundColor: 'var(--mantine-color-dark-7)'
                },
                search: {
                    backgroundColor: 'var(--mantine-color-dark-7)'
                }
            }}
        >
            <Combobox.Target>
                <InputBase
                    component="button"
                    type="button"
                    error={isError ? 'Error while loading structures' : ''}
                    withErrorStyles={false}
                    rightSection={<Combobox.Chevron />}
                    onClick={() => combobox.toggleDropdown()}
                    rightSectionPointerEvents="none"
                    disabled={isError}
                    pointer
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
    )
}

export type StructureSelectorProp = {
    onSelect: (entry: Uuid) => void;

    selected?: Uuid[],

    // filters that are directly given to the structure request
    filters?: StructureFilter,
}
