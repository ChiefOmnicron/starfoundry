import { Avatar, Combobox, Group, Input, InputBase, Loader, Text, useCombobox } from "@mantine/core";
import { EveIcon } from "../EveIcon";
import { useDebouncedCallback } from "@mantine/hooks";
import { useState, type ReactElement } from "react";
import type { Item } from "@/services/item/model";
import { inGameSearch, type InGameSearchFilter } from "@/services/inGameSearch";
import type { CharacterInfo } from "@/services/client";

function SelectOption(character: CharacterInfo) {
    return (
        <Group key={character.character_id}>
            {
                <Avatar
                    src={`https://images.evetech.net/characters/${character.character_id}/portrait`}
                    radius="xl"
                />
            }

            <div>
                <Text fz="sm" fw={500}>
                    {character.character_name}
                </Text>
            </div>
        </Group>
    );
}

// Usage:
//
// ```
// const [addItemSelect, setAddItemSelect] = useState<Item | undefined>();
// const InGameSearchRef = useRef<InGameSearchRef>({} as any);
// 
// const entries = [];
// 
// <InGameSearch
//     onSelect={setAddItemSelect}
//     selected={(entries || []).map(x => x.type_id)}
//     ref={InGameSearchRef as any}
// />
// ```
export function InGameSearch({
    onSelect,
    ref = {
        current: {
            reset: () => {}
        }
    },

    category,

    withinPortal = false,
}: InGameSearchProp): ReactElement {
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

    const [loading, setLoading] = useState<boolean>(false);

    const [value, setValue] = useState<string | null>(null);
    const [search, setSearch] = useState('');

    const [options, setOptions] = useState<any[]>([]);
    const [characterInfo, setCharacterInfo] = useState<CharacterInfo[]>([]);

    const resetValue = () => {
        setValue(null);
        setOptions([]);
    };
    ref.current.reset = resetValue;

    const fetchData = useDebouncedCallback(async (query: string) => {
        setLoading(true);

        let filter: InGameSearchFilter = {
            categories: [category],
            search: query,
        };

        inGameSearch(filter)
            .then(x => {
                setCharacterInfo(x);
                setLoading(false);

                const data = x
                    .map((character) => (
                        <Combobox.Option
                            value={character.character_id.toString()}
                            key={character.character_id}
                        >
                            <SelectOption key={character.character_id} {...character} />
                        </Combobox.Option>
                    ));

                setOptions(data);
            })
    }, 200);

    const itemByTypeId = (
        typeId: string | null
    ): CharacterInfo | undefined => {
        if (typeId) {
            const typeIdConverted = Number.parseInt(typeId || '0');
            return (characterInfo || []).find(x => x.character_id === typeIdConverted)
        } else {
            return undefined;
        }
    }

    const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
        setSearch(event.currentTarget.value);
        fetchData(event.currentTarget.value);
    };

    return (
        <Combobox
            store={combobox}
            withinPortal={withinPortal}
            onOptionSubmit={(val) => {
                setValue(val);
                onSelect(characterInfo.find(x => x.character_id.toString() === val) as any);
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
            position="bottom"
        >
            <Combobox.Target>
                <InputBase
                    component="button"
                    type="button"
                    withErrorStyles={false}
                    rightSection={loading ? <Loader size={18} /> : <Combobox.Chevron />}
                    onClick={() => combobox.toggleDropdown()}
                    rightSectionPointerEvents="none"
                    pointer
                >
                    {
                        //itemByTypeId(value as any)?.name ||
                        //<Input.Placeholder>Select an item</Input.Placeholder>
                    }
                </InputBase>
            </Combobox.Target>

            <Combobox.Dropdown>
                <Combobox.Search
                    value={search}
                    onChange={handleSearch}
                    placeholder="Search item"
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

export type InGameSearchProp = {
    onSelect: (entry: any) => void;

    category: 'agent' | 'alliance' | 'character' | 'constellation' | 'corporation' | 'faction' | 'inventory_type' | 'region' | 'solar_system' | 'station' | 'structure';

    ref: { current: InGameSearchRef };
    withinPortal?: boolean;
}

export type InGameSearchRef = {
    reset: () => void,
}
