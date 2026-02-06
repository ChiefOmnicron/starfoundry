import { Combobox, Group, Input, InputBase, Loader, Text, useCombobox } from "@mantine/core";
import { EveIcon } from "../EveIcon";
import { listItem, type ItemFilter } from "@/services/item/list";
import { useDebouncedCallback } from "@mantine/hooks";
import { useState, type ReactElement } from "react";
import type { Item } from "@/services/item/model";
import type { TypeId } from "@/services/utils";

function SelectOption(item: Item) {
    return (
        <Group key={item.type_id}>
            {
                <EveIcon
                    id={item.type_id}
                    type={
                        item.name.endsWith('Blueprint')
                        ? 'bp'
                        : 'icon'
                    }
                />
            }

            <div>
                <Text fz="sm" fw={500}>
                    {item.name}
                </Text>
            </div>
        </Group>
    );
}

// Usage:
//
// ```
// const [addItemSelect, setAddItemSelect] = useState<Item | undefined>();
// const ItemSelectorRef = useRef<ItemSelectorRef>({} as any);
// 
// const entries = [];
// 
// <ItemSelector
//     onSelect={setAddItemSelect}
//     selected={(entries || []).map(x => x.type_id)}
//     ref={ItemSelectorRef as any}
// />
// ```
export function ItemSelector({
    onSelect,
    ref = {
        current: {
            reset: () => {}
        }
    },

    selected = [],
    blueprint = false,
    buildable = false,

    withinPortal = false,
}: ItemSelectorProp): ReactElement {
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
    const [items, setItems] = useState<Item[]>([]);

    const resetValue = () => {
        setValue(null);
        setOptions([]);
    };
    ref.current.reset = resetValue;

    const fetchData = useDebouncedCallback(async (query: string) => {
        setLoading(true);

        let filter: ItemFilter = {
            name: query,
        };

        if (buildable) {
            filter.buildable = buildable;
        }
        if (blueprint) {
            filter.blueprint = blueprint;
        }

        listItem(filter)
            .then(x => {
                setItems(x);
                setLoading(false);

                const data = x
                    .filter(x => selected.indexOf(x.type_id) === -1)
                    .map((item) => (
                        <Combobox.Option
                            value={item.type_id.toString()}
                            key={item.type_id}
                        >
                            <SelectOption key={item.type_id} {...item} />
                        </Combobox.Option>
                    ));

                setOptions(data);
            })
    }, 200);

    const itemByTypeId = (
        typeId: string | null
    ): Item | undefined => {
        if (typeId) {
            const typeIdConverted = Number.parseInt(typeId || '0');
            return (items || []).find(x => x.type_id === typeIdConverted)
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
                onSelect(items.find(x => x.type_id.toString() === val) as any);
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
                        itemByTypeId(value as any)?.name ||
                        <Input.Placeholder>Select an item</Input.Placeholder>
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

export type ItemSelectorProp = {
    onSelect: (entry: Item) => void;

    // list of values that are already selected, and filtered out
    selected?:      TypeId[],

    blueprint?: boolean;
    buildable?: boolean;

    ref: { current: ItemSelectorRef };
    withinPortal?: boolean;
}

export type ItemSelectorRef = {
    reset: () => void,
}
