import { BadgeWrapper } from "@internal/wrapper/Badge";
import { Combobox, Group, Pill, PillsInput, useCombobox } from "@mantine/core";
import { useEffect, useState } from "react";
import type { Tag } from "@internal/services/tags/list";
import type { Uuid } from "@internal/services/utils";

export function TagSelector({
    disabled,

    onSelect,

    selected = [],
    tags,
}: TagSelectorProps) {
    const combobox = useCombobox({
        onDropdownClose: () => combobox.resetSelectedOption(),
        onDropdownOpen: () => combobox.updateSelectedOptionIndex("active"),
    });

    const [search, setSearch] = useState("");
    const [value, setValue] = useState<string[]>([]);

    useEffect(() => {
        if (selected) {
            setValue(selected);
        } else {
            setValue([]);
        }
    }, []);

    const handleValueSelect = (selectedTagId: string) => {
        console.log(value.find(x => x === selectedTagId))
        setValue((current) =>
            current.find(x => x === selectedTagId)
                ? current.filter((x) => x !== selectedTagId)
                : [...current, selectedTagId],
            );
    }

    const values = value
        .map((item: string) => {
            const entry = tags.find((x) => x.id === item);
            if (!entry) {
                return <></>;
            }

            return <>
                <BadgeWrapper
                    key={entry.id}
                    color={entry.color}
                >
                    {entry.content}
                </BadgeWrapper>
            </>
        });

    const options = () => {
        return tags
            .filter(item => item.content.toLowerCase().includes(search.trim().toLowerCase()))
            .filter(item => item.typ !== 'AUTO')
            .map((item) => <>
                    <Combobox.Option
                        value={item.id}
                        key={item.id}
                        active={true}
                    >
                        <Group gap="sm">
                            <BadgeWrapper
                                color={item.color}
                            >
                                {item.content}
                            </BadgeWrapper>
                        </Group>
                    </Combobox.Option>
                </>
            );
    }

    return (
        <Combobox
            disabled={disabled}
            store={combobox}
            onOptionSubmit={(val) => {
                handleValueSelect(val);
                onSelect(tags.find(x => x.id === val) as any);
            }}
            withinPortal={false}
        >
            <Combobox.DropdownTarget>
                <PillsInput onClick={() => combobox.openDropdown()}>
                    <Pill.Group>
                        {values}

                        <Combobox.EventsTarget>
                            <PillsInput.Field
                                onFocus={() => combobox.openDropdown()}
                                onBlur={() => combobox.closeDropdown()}
                                value={search}
                                placeholder="Search values"
                                onChange={(event) => {
                                    combobox.updateSelectedOptionIndex();
                                    setSearch(event.currentTarget.value);
                                }}
                                onKeyDown={(event) => {
                                    if (
                                        event.key === "Backspace" &&
                                        search.length === 0 &&
                                        value.length > 0
                                    ) {
                                        event.preventDefault();
                                    }
                                }}
                            />
                        </Combobox.EventsTarget>
                    </Pill.Group>
                </PillsInput>
            </Combobox.DropdownTarget>

            <Combobox.Dropdown>
                <Combobox.Options>
                {
                    options().length > 0 ? (
                        options()
                    ) : (
                        <Combobox.Empty>Nothing found...</Combobox.Empty>
                    )
                }
                </Combobox.Options>
            </Combobox.Dropdown>
        </Combobox>
    );
}

export type TagSelectorProps = {
    // disables the input
    disabled?: boolean;

    onSelect: (entry: Tag) => void;

    // tags that are pre-selected
    selected?: Uuid[];
    // tags that can be selected
    tags: Tag[];
}
