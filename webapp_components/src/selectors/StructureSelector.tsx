import { Combobox, Group, Input, InputBase, Text, useCombobox } from "@mantine/core";
import { useEffect, useState, type ReactElement } from "react";
import type { Structure } from "@internal/services/structure/list";
import { EveIcon } from "@internal/misc/EveIcon";

export function StructureSelector({
    structures,

    selected,

    onSelect,
}: StructureSelectorModalProp): ReactElement {
    const [value, setValue] = useState<string | null>(null);
    const [search, setSearch] = useState('');
    const selectedOption = structures.find((item) => item.id === value);

    useEffect(() => {
        if (selected) {
            setValue(selected.id);
        }
    }, [selected]);

    const combobox = useCombobox({
        onDropdownClose: () => combobox.resetSelectedOption(),
    });

    const options = structures
        .filter(x => {
            return x.name.toLowerCase().includes(search.toLowerCase().trim()) ||
            x.system.system_name.toLowerCase().includes(search.toLowerCase().trim())
        })
        .map(x => {
            return <>
                <Combobox.Option value={x.id} key={x.id}>
                    <SelectOption {...x} />
                </Combobox.Option>
            </>
        })

    return <>
        <Combobox
            store={combobox}
            withinPortal={false}
            onOptionSubmit={(value) => {
                setValue(value);
                setSearch(value);

                const selected = structures.find((item) => item.id === value);
                if (selected) {
                    onSelect(selected);
                }
                combobox.closeDropdown();
            }}
        >
            <Combobox.Target targetType="button">
                <InputBase
                    component="button"
                    type="button"
                    pointer
                    rightSection={<Combobox.Chevron />}
                    onClick={() => combobox.toggleDropdown()}
                    rightSectionPointerEvents="none"
                    multiline
                >
                    {
                        selectedOption
                        ?   <SelectOption {...selectedOption} />
                        :   <Input.Placeholder>Search for structure</Input.Placeholder>
                    }
                </InputBase>
            </Combobox.Target>

            <Combobox.Dropdown>
                <Combobox.Search
                    value={search}
                    onChange={(event) => setSearch(event.currentTarget.value)}
                    placeholder="Search structures"
                />

                <Combobox.Options>{options}</Combobox.Options>
            </Combobox.Dropdown>
        </Combobox>
    </>
}

function SelectOption({item, name, rigs, services, system}: Structure) {
    return <>
        <Group>
            <EveIcon id={item.type_id} />
            <Text>{name}</Text>
        </Group>

        <Group>
            <Text fz="xs" opacity={0.6} fw={500}>System: </Text>
            <Text fz="xs" opacity={0.6}>
                {system.system_name}
            </Text>
        </Group>

        <Group>
            <Text fz="xs" opacity={0.6} fw={500}>Rigs: </Text>
            <Text fz="xs" opacity={0.6}>
                {
                    rigs.length === 0
                    ?   <>No Rigs</>
                    :   rigs.map(x => x.item.name).join(', ')
                }
            </Text>
        </Group>

        <Group>
            <Text fz="xs" opacity={0.6} fw={500}>Services: </Text>
            <Text fz="xs" opacity={0.6}>
                {
                    services.length === 0
                    ?   <>No Services</>
                    :   services.map(x => x.name).join(', ')
                }
            </Text>
        </Group>
    </>;
}


export type StructureSelectorModalProp = {
    // structures the user can select
    structures: Structure[],
    // list of values that are already selected
    selected?:  Structure,

    onSelect(structure: Structure): void;
}
