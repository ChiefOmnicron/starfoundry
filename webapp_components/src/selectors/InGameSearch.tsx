import { Avatar, Combobox, Group, Input, InputBase, Loader, Pill, Text, useCombobox } from "@mantine/core";
import { useDebouncedCallback } from "@mantine/hooks";
import { useState, type ReactElement } from "react";
import type { Category } from "@internal/services/utils";
import { inGameSearch, type InGameSearchFilter, type InGameSearchResponse } from "@internal/services/inGameSearch";

function SelectOption(searchResult: InGameSearchResponse) {
    let category;
    switch (searchResult.category) {
        case 'alliance':
            category = 'alliances';
            break;
        case 'corporation':
            category = 'corporations';
            break;
        case 'character':
            category = 'characters';
            break;
        default:
            category = 'types';
    }

    let variation = searchResult.category === 'character'
        ? 'portrait'
        : 'logo';

    return (
        <Group key={searchResult.id}>
            {
                <Avatar
                    src={`https://images.evetech.net/${category}/${searchResult.id}/${variation}`}
                    radius="xl"
                />
            }

            <div>
                <Text fz="sm" fw={500}>
                    {searchResult.name}
                </Text>

                <Pill>
                    {searchResult.category}
                </Pill>
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

    categories,

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

    const [search, setSearch] = useState('');
    
    const [options, setOptions] = useState<any[]>([]);
    const [_, setSelectedValue] = useState<InGameSearchResponse | undefined>(undefined);
    const [searchResponse, setSearchResponse] = useState<InGameSearchResponse[]>([]);

    const resetValue = () => {
        setSelectedValue(undefined);
        setOptions([]);
    };
    ref.current.reset = resetValue;

    const fetchData = useDebouncedCallback(async (query: string) => {
        if (query.length < 3) {
            return;
        }

        setLoading(true);

        let filter: InGameSearchFilter = {
            categories,
            search: query,
        };

        inGameSearch(filter)
            .then(x => {
                setLoading(false);
                setSearchResponse(x);

                if (!x) {
                    return;
                }

                const data = x
                    .map((searchResult) => (
                        <Combobox.Option
                            value={searchResult.id.toString()}
                            key={searchResult.id}
                        >
                            <SelectOption key={searchResult.id} {...searchResult} />
                        </Combobox.Option>
                    ));

                setOptions(data);
            })
    }, 200);

    const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
        setSearch(event.currentTarget.value);
        fetchData(event.currentTarget.value);
    };

    return (
        <Combobox
            store={combobox}
            withinPortal={withinPortal}
            onOptionSubmit={(val) => {
                const entry = searchResponse
                    .find(x => x.id.toString() === val) as any

                onSelect(entry);
                setSelectedValue(entry)
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
                        //selectedValue
                        //    ? selectedValue.name
                        //    : <Input.Placeholder>Search for an alliance, corporation or character</Input.Placeholder>
                        <Input.Placeholder>Search for an alliance, corporation or character</Input.Placeholder>
                    }
                </InputBase>
            </Combobox.Target>

            <Combobox.Dropdown>
                <Combobox.Search
                    value={search}
                    onChange={handleSearch}
                    placeholder="Search for an alliance, corporation or character"
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
    onSelect: (entry: InGameSearchResponse) => void;

    categories: Category[];

    ref: { current: InGameSearchRef };
    withinPortal?: boolean;
}

export type InGameSearchRef = {
    reset: () => void,
}
