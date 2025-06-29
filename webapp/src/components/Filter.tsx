import {
    Combobox,
    Pill,
    PillsInput,
    useCombobox,
} from "@mantine/core";
import { useEffect, useState, type ReactElement } from "react";

//     const exampleData: FilterPropEntry[] = [{
//        label: 'Single Select',
//        key: 'single',
//        type: 'SELECT',
//        options: [{
//            label: 'A',
//            key: 'a',
//        }, {
//            label: 'B',
//            key: 'b',
//        }, {
//            label: 'C',
//            key: 'c',
//        }],
//    }, {
//        label: 'Multiselect',
//        key: 'multi',
//        type: 'MULTISELECT',
//        options: [{
//            label: 'D',
//            key: 'd'
//        }, {
//            label: 'E',
//            key: 'e'
//        }, {
//            label: 'F',
//            key: 'f'
//        }, {
//            label: 'G',
//            key: 'g'
//        }]
//    }, {
//        label: 'Name',
//        key: 'name',
//        type: 'STRING',
//    }];
//
//    const filterChange = (filters: SelectedFilter[]) => {
//        console.log(filters)
//    }
//
//    <Filter
//        entries={exampleData}
//        onFilterChange={filterChange}
//    />
//
// Inspired by https://medium.com/@lodestar-design/process-doc-how-i-polish-micro-interactions-for-search-filters-in-a-table-bd729a55895c
//
// TODO: allow for string filters to
//  - contain ...Az... - *search*
//  - startsWith Az... - search*
//  - endsWith ...Az   - *search
//  - equals =         - = search
//  - not equals !=    - != search
// https://ux.stackexchange.com/questions/75704/what-symbol-can-be-used-to-denote-contains
// TODO: allow for integer filters
// - greater than > - >
// - lesser than < - <
// - greater or equal >= - >=
// - lesser or equal <= - <=
// - equal = - =
// - not equal != - !=
export function Filter(
    {
        entries,
        onFilterChange,
    }: FilterProp,
): ReactElement {
    const [search, setSearch] = useState('');

    const [currentSelected, setCurrentSelected] = useState<FilterPropEntry | undefined>(undefined);
    const [currentSelectedOptions, setCurrentSelectedOptions] = useState<FilterPropOption[]>([]);
    const [selectedFilters, setSelectFilters] = useState<SelectedFilter[]>([]);

    const [options, setOptions] = useState(entries);
    const [originalOptions] = useState(entries);

    const combobox = useCombobox({
        onDropdownClose: () => combobox.resetSelectedOption(),
        onDropdownOpen: () => combobox.updateSelectedOptionIndex('active'),
    });

    useEffect(() => {
        resetOptions();

        // TODO: add mutliselect into an array
        onFilterChange(selectedFilters);
    }, [selectedFilters]);

    /// adds an entry to the list of selected filters
    const addToValues = (
        label: string,
        key: string,
        value: string,
    ) => {
        // add the entry to our array of selected filters
        setSelectFilters(filters => [...filters, {
            key,
            label,
            value,
        }]);
    }

    // check for options that no longer can be selected, or that were removed
    // and therefor can be added again
    const resetOptions = () => {
        setOptions(
            originalOptions.filter(x => {
                const findFilter = selectedFilters.filter(y => y.key === x.key);
                if (findFilter.length > 0) {
                    return x.type === 'MULTISELECT' && findFilter.length < (x.options || []).length;
                }

                return true;
            })
        );
    }

    const handleValueSelect = (value: string) => {
        if (currentSelectedOptions.length === 0) {
            const currentSelected = entries.find(x => x.key === value);
            // make sure the value of the selected option is shown
            setSearch(`${currentSelected?.label}: `);

            // set the selected option for later use
            setCurrentSelected(currentSelected);

            if (currentSelected?.type === 'STRING') {
                combobox.closeDropdown();
            }

            // replace the dropdown options with the options from the selected option
            const filters = selectedFilters.filter(y => y.key === value);
            setCurrentSelectedOptions(
                (
                    currentSelected?.options || []
                ).filter(x => !filters.find(y => y.value === x.label))
            );
        } else {
            // store the label for later use
            // it is pretty sure that the value is set, just making sure
            const label = (currentSelected || { label: ''}).label;
            const key = (currentSelected || { key: ''}).key;
            // add the entries to the input
            addToValues(label, key, value);

            // reset search, current selected, and the options
            setSearch('');
            setCurrentSelected(undefined);
            setCurrentSelectedOptions([]);
        }
    };

    const handleValueRemove = (val: SelectedFilter) => {
        setSelectFilters((current) => current.filter((v) => v !== val));
    };

    // show the primary entries
    const optionsFirstLevel = options
        .map((item) => (
            <Combobox.Option
                value={item.key}
                key={item.key}
            >
                <span>{item.label}</span>
            </Combobox.Option>
        ));

    // show sub options from an entry
    const optionsSecondLevel = currentSelectedOptions
        .map((item: FilterPropOption) => (
            <Combobox.Option
                value={item.label}
                key={item.key}
            >
                <span>{item.label}</span>
            </Combobox.Option>
        ));

    // render for the values
    const values = selectedFilters.map((item) => (
        <Pill
            key={`${item.key}_${item.value}`}
            withRemoveButton
            onRemove={() => handleValueRemove(item)}
            size="md"
            styles={{
                root: {
                    borderRadius: 0,
                }
            }}
        >
            <strong>{item.label}</strong>: {item.value}
        </Pill>
    ));

    const showDropdownEntries = () => {
        if (currentSelected && currentSelected.type === 'STRING') {
            return;
        } else if (currentSelectedOptions.length === 0) {
            if (optionsFirstLevel.length > 0) {
                return optionsFirstLevel;
            } else {
                return;
            }
        } else {
            return optionsSecondLevel;
        }
    }

    return (
        <Combobox
            data-cy="filterCombobox"
            store={combobox}
            onOptionSubmit={handleValueSelect}
            withinPortal={false}
        >
            <Combobox.DropdownTarget>
                <PillsInput
                    styles={{
                        input: {
                            borderLeft: 0,
                            borderRight: 0,
                            borderTop: 0,
                        }
                    }}
                >
                    <Pill.Group data-cy="filterSelectedGroup">
                        {values}

                        <Combobox.EventsTarget>
                            <PillsInput.Field
                                data-cy="filterInput"
                                onFocus={() => showDropdownEntries() ? combobox.openDropdown() : false}
                                onBlur={() => combobox.closeDropdown()}
                                value={search}
                                placeholder="Filter"
                                onChange={(event) => {
                                    combobox.updateSelectedOptionIndex();
                                    setSearch(event.currentTarget.value);

                                    if (event.currentTarget.value === '') {
                                        setSearch('');
                                        setCurrentSelected(undefined);
                                        setCurrentSelectedOptions([]);
                                        resetOptions();
                                    }
                                }}
                                onKeyDown={(event) => {
                                    if (
                                        event.key === "Backspace" &&
                                        search.length === 0
                                    ) {
                                        event.preventDefault();
                                        handleValueRemove(
                                            selectedFilters[selectedFilters.length - 1],
                                        );
                                    }

                                    if (search.endsWith(':')) {
                                        setSearch('');
                                        setCurrentSelected(undefined);
                                        setCurrentSelectedOptions([]);
                                    }

                                    // prevent that non-selectable items are added
                                    if (currentSelected?.type !== 'STRING') {
                                        return;
                                    }

                                    if (
                                        event.key === 'Enter' &&
                                        currentSelected &&
                                        search.length > 0 &&
                                        !search.endsWith(': ')
                                    ) {
                                        let value = search.replace(`${currentSelected.label}: `, '');

                                        addToValues(
                                            currentSelected.label,
                                            currentSelected.key,
                                            value,
                                        );

                                        setSearch('');
                                        setCurrentSelected(undefined);
                                        setCurrentSelectedOptions([]);

                                        if (showDropdownEntries()) {
                                            combobox.openDropdown();
                                        }
                                    }
                                }}
                            />
                        </Combobox.EventsTarget>
                    </Pill.Group>
                </PillsInput>
            </Combobox.DropdownTarget>

            <Combobox.Dropdown>
                <Combobox.Options data-cy="filterDropdownOption">
                    { showDropdownEntries() }
                </Combobox.Options>
            </Combobox.Dropdown>
        </Combobox>
    );
}

export type FilterProp = {
    entries: FilterPropEntry[];
    onFilterChange: (filters: SelectedFilter[]) => void,
}

export type FilterPropEntry = {
    // shown as the option name
    label: string;
    // unique key to identify the entry
    key: string;
    // input: free string input
    // select: one value from the entry field can be selected
    // multiselect: one or more values can be selected from the entry field
    type: 'STRING' | 'SELECT' | 'MULTISELECT';
    options?: FilterPropOption[];
};

export type FilterPropOption = {
    label: string;
    key: string;
};

export type SelectedFilter = {
    value: number | string | Array<string>;
    label: string;
    key: string;
};
