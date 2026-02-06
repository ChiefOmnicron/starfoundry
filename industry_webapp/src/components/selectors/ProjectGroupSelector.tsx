import type { Uuid } from "@/services/utils";
import { ActionIcon, Combobox, Flex, Group, Input, InputBase, Text, useCombobox } from "@mantine/core";
import { useState, type ReactElement } from "react";
import { faArrowsRotate } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useQueryClient } from "@tanstack/react-query";
import type { ProjectGroup } from "@/services/project-group/fetch";
import { LIST_PROJECT_GROUPS } from "@/services/project-group/list";

function SelectOption(projectGroup: ProjectGroup) {
    return (
        <Group key={projectGroup.id}>
            <div>
                <Text fz="sm" fw={500}>
                    {projectGroup.name}
                </Text>
            </div>
        </Group>
    );
}

export function ProjectGroupSelector({
    onSelect,
    projectGroups,
    selected = [],
}: ProjectGroupSelectorProp): ReactElement {
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

    const projectGroupById = (
        id: string | null
    ): ProjectGroup | undefined => {
        if (id) {
            return (projectGroups || []).find(x => x.id === id)
        } else {
            return undefined;
        }
    }

    const options = (projectGroups || [])
        .filter((x: ProjectGroup) => {
            return x.name
                .toLocaleLowerCase()
                .indexOf(
                    search.toLocaleLowerCase()
                ) >= 0
        })
        .filter((x: ProjectGroup) => selected.indexOf(x.id) === -1)
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
            queryKey: [LIST_PROJECT_GROUPS]
        })
    }

    return (
        <Flex
            direction="row"
            wrap="nowrap"
        >
            <Combobox
                store={combobox}
                withinPortal
                onOptionSubmit={(val) => {
                    setValue(val);
                    onSelect(projectGroups.find(x => x.id === val) as any);
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
                        label="Project Group"
                        description="Select a project group"
                        component="button"
                        type="button"
                        withErrorStyles={false}
                        rightSection={<Combobox.Chevron />}
                        onClick={() => combobox.toggleDropdown()}
                        rightSectionPointerEvents="none"
                        pointer
                        withAsterisk
                        style={{
                            width: '100%'
                        }}
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
                                        marginTop: 'calc(var(--mantine-spacing-xs) / 2)',
                                    }}
                                >
                                    <FontAwesomeIcon icon={faArrowsRotate} />
                                </ActionIcon>
                            </Flex>
                        }}
                    >
                        {
                            projectGroupById(value)?.name ||
                            <Input.Placeholder>My project group</Input.Placeholder>
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

export type ProjectGroupSelectorProp = {
    onSelect: (entry: ProjectGroup) => void;

    // list of values that are already selected, and filtered out
    selected?:      Uuid[],

    projectGroups:  ProjectGroup[],
}
