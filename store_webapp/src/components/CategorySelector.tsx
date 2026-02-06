import { useListCategory } from "@/services/product/category_list";
import { Combobox, InputBase, useCombobox } from "@mantine/core";
import { useState } from "react";

export type CategorySelectorProps = {
    onChange(e: string): void,
    defaultValue: string | undefined,
}

export function CategorySelector({
    onChange,
    defaultValue,
}: CategorySelectorProps) {
    const [value, setValue] = useState<string>(defaultValue || 'Uncategorized');
    const [search, setSearch] = useState<string>(defaultValue || 'Uncategorized');

    const {
        data: categories,
    } = useListCategory();

    const combobox = useCombobox({
        onDropdownClose: () => combobox.resetSelectedOption(),
    });

    const options = categories.map((item) => (
        <Combobox.Option value={item} key={item}>
            {item}
        </Combobox.Option>
    ));

    return (
        <Combobox
            store={combobox}
            withinPortal={false}
            onOptionSubmit={(val) => {
                if (val === "$create") {
                    setValue(search || 'Uncategorized');
                    onChange(search || 'Uncategorized');
                } else {
                    setValue(val || 'Uncategorized');
                    setSearch(val || 'Uncategorized');
                    onChange(val || 'Uncategorized');
                }

                combobox.closeDropdown();
            }}
        >
            <Combobox.Target>
                <InputBase
                    label="Category"
                    description="Select an existing category, or create a category"
                    rightSection={<Combobox.Chevron />}
                    value={search}
                    onChange={(event) => {
                        combobox.openDropdown();
                        combobox.updateSelectedOptionIndex();
                        setSearch(event.currentTarget.value);
                    }}
                    onClick={() => combobox.openDropdown()}
                    onFocus={() => combobox.openDropdown()}
                    onBlur={() => {
                        combobox.closeDropdown();
                        setSearch(value || "");
                    }}
                    placeholder="Search value"
                    rightSectionPointerEvents="none"
                    withAsterisk
                />
            </Combobox.Target>

            <Combobox.Dropdown>
                <Combobox.Options>
                    {options}
                    {<Combobox.Option value="$create">+ Create {search}</Combobox.Option>}
                </Combobox.Options>
            </Combobox.Dropdown>
        </Combobox>
    );
}
