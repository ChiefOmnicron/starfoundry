import { Combobox, Pill, PillsInput, useCombobox } from "@mantine/core";
import { useEffect, useState } from "react";
import { listProjects, type ProjectListMinimal } from "../services/projects/list";
import type { Uuid } from "../services/utils";
import { useDebouncedCallback } from "@mantine/hooks";

export function ProjectSelector({
    selected,

    onChange,
}: ProjectSelectorProps) {
    const [loading, setLoading] = useState(false);
    const [search, setSearch] = useState<string>('');

    const [value, setValue] = useState<Uuid[]>([]);
    const [data, setData] = useState<ProjectListMinimal[]>([]);

    const [selectedProjects, setSelectedProjects] = useState<ProjectListMinimal[]>([]);

    useEffect(() => {
        const selectedMapped = [];
        for (const entry of selected) {
            let dataEntry = data.find(y => y.id === entry);
            if (dataEntry) {
                selectedMapped.push(dataEntry);
            } else {
                continue
            }
        }
        setSelectedProjects(selectedMapped || []);

        setValue(selected);
    }, [selected]);

    const combobox = useCombobox({
        onDropdownClose: () => {
            combobox.resetSelectedOption();
            setSearch('');
        },
        onDropdownOpen: () => {
            combobox.focusSearchInput();
        },
    });

    const handleValueSelect = (newValue: Uuid) => {
        const updatedValue = value.includes(newValue)
            ? value.filter((v) => v !== newValue)
            : [...value, newValue];

        setValue(updatedValue);
        setSearch('');

        const findValue = data.find(x => x.id === newValue);
        if (findValue) {
            setSelectedProjects([...selectedProjects, findValue]);
            onChange([...selectedProjects, findValue].filter(x => updatedValue.includes(x.id)));
        }
    };

    const handleValueRemove = (removeValue: Uuid) => {
        const updatedValue = value.filter((v) => v !== removeValue);

        setValue(updatedValue);
        onChange(selectedProjects.filter(x => updatedValue.includes(x.id)));
    };

    const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
        setSearch(event.currentTarget.value);
        searchProjects(event.currentTarget.value);
    };

    const searchProjects = useDebouncedCallback(async (query: string) => {
        setLoading(true);

        listProjects({
                name: query,
            })
            .then((response) => {
                setData(response);
                setLoading(false);
                combobox.resetSelectedOption();
            });
    }, 200);

    const values = value
        .map(x => {
            return <Pill key={x} withRemoveButton onRemove={() => handleValueRemove(x)}>
                {selectedProjects.find(y => y.id === x)?.name}
            </Pill>
        });

    const options = data
        .slice(0, 10) // show max 10 items
        .map((project) => (
            <Combobox.Option value={project.id} key={project.id} active={value.includes(project.id)}>
                {project.name}
            </Combobox.Option>
        ));

    return <>
        <Combobox
            store={combobox}
            withinPortal={false}
            onOptionSubmit={(value: string) => handleValueSelect(value)}
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
                            placeholder="Search systems"
                            onChange={(event) => {
                                combobox.updateSelectedOptionIndex();
                                handleSearch(event);
                            }}
                            onKeyDown={(event) => {
                                if (event.key === 'Backspace' && search.length === 0 && value.length > 0) {
                                    event.preventDefault();
                                    handleValueRemove(value[value.length - 1]);
                                }
                            }}
                        />
                        </Combobox.EventsTarget>
                    </Pill.Group>
                </PillsInput>
            </Combobox.DropdownTarget>

            <Combobox.Dropdown>
                <Combobox.Options>
                    {loading ? <Combobox.Empty>Loading....</Combobox.Empty> : options}

                    {options.length === 0 ? <Combobox.Empty>Search for projects....</Combobox.Empty> : <></>}
                </Combobox.Options>
            </Combobox.Dropdown>
        </Combobox>
    </>
}

export type ProjectSelectorProps = {
    selected: Uuid[];
    onChange: (projects: ProjectListMinimal[]) => void;
}

export type ProjectSelectorValue = {
    label: string,
    value: number,
}
