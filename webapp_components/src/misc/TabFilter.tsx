import { Button, Pill, Table, Tabs } from "@mantine/core";
import { useState, type ReactNode } from "react";
import { ModalWrapper } from "../wrapper";
import { useDisclosure } from "@mantine/hooks";

export function TabFilter({
    filters,
    renderComponent,

    createFilter,
    editFilter,

    onDelete,
}: TabFilterProps) {
    if (!renderComponent) {
        return <></>;
    }

    //const [activeTab, setActiveTab] = useState<string | null>('settings');
    const [activeTab, setActiveTab] = useState<string | null>(filters[0].filter_name);

    const tabs = filters.map(x => <Tabs.Tab value={x.filter_name}>{`${x.filter_name}`}</Tabs.Tab>);
    const panels = filters.map(x => <Tabs.Panel value={x.filter_name}>{renderComponent(x.filters)}</Tabs.Panel>);

    return <>
        <Tabs value={activeTab} onChange={setActiveTab}>
            <Tabs.List>
                {tabs}

                <Tabs.Tab value="all">All</Tabs.Tab>

                <Tabs.Tab value="settings" ml="auto">
                    Filters
                </Tabs.Tab>
            </Tabs.List>

            {panels}

            <Tabs.Panel value="all">
                {renderComponent([])}
            </Tabs.Panel>

            <Tabs.Panel value="settings">
                <TabFilterSetting
                    filters={filters}

                    createFilter={createFilter}
                    editFilter={editFilter}

                    onDelete={onDelete}
                />
            </Tabs.Panel>
        </Tabs>
    </>
}

export function TabFilterSetting({
    filters,

    createFilter,
    editFilter,

    onDelete,
}: TabFilterSettingProps) {
    const [addFilterModalOpened, { open: openAddFilterModal, close: closeAddFilterModal }] = useDisclosure(false);
    // TODO: Modal for Create and Edit

    return <>
        <ModalWrapper
            title="Add Filter"
            children={createFilter()}
            opened={addFilterModalOpened}
            close={closeAddFilterModal}
        />

        <Table>
            <Table.Thead>
                <Table.Tr>
                    <Table.Th></Table.Th>
                    <Table.Th>Name</Table.Th>
                    <Table.Th>Filter</Table.Th>
                    <Table.Th>Edit/Remove</Table.Th>
                </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
                {
                    filters
                        .map(x => {
                            return <Table.Tr>
                                <Table.Td>
                                    D
                                </Table.Td>
                                <Table.Td>
                                    {x.filter_name}
                                </Table.Td>
                                <Table.Td>
                                    {x.filters.map(y => <Pill>{y.key}: {y.value}</Pill>)}
                                </Table.Td>
                                <Table.Td>
                                    <Button onClick={() => onDelete(x)}>
                                        Delete
                                    </Button>
                                    <Button onClick={() => editFilter(x)}>
                                        Edit
                                    </Button>
                                </Table.Td>
                            </Table.Tr>
                        })
                }
            </Table.Tbody>
        </Table>

        <Button
            onClick={() => openAddFilterModal()}
        >
            Add Filter
        </Button>
    </>
}

export type TabFilterProps = {
    filters:            TabFilterDefinition[];

    createFilter:       () => ReactNode;
    editFilter:         (filter: TabFilterDefinition) => ReactNode;
    renderComponent:    (filter: TabFilterFilterDefinition[]) => ReactNode;

    onDelete:           (filter: TabFilterDefinition) => void;
};

export type TabFilterSettingProps = {
    filters:        TabFilterDefinition[];

    createFilter:   () => ReactNode;
    editFilter:     (filter: TabFilterDefinition) => ReactNode;

    onDelete:       (filter: TabFilterDefinition) => void;
}

export type TabFilterDefinition = {
    filter_name: string;
    filters: TabFilterFilterDefinition[];
}

export type TabFilterFilterDefinition = {
    key:    string;
    value:  string | number;
}
