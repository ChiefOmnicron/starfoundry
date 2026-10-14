import { Button, Pill, Table, Tabs } from "@mantine/core";
import { useState, type ReactNode } from "react";
import { ModalWrapper } from "../wrapper";
import { useDisclosure } from "@mantine/hooks";
import { useMutation } from "@tanstack/react-query";
import { deleteFilter } from "../services/filter/delete";
import type { Uuid } from "../services/utils";
import { LIST_FILTERS } from "../services/filter/list";

export function TabFilter({
    filters,
    renderComponent,

    createFilter,
    editFilter,

    onDelete = () => {},
}: TabFilterProps) {
    if (!renderComponent) {
        return <></>;
    }

    const [activeTab, setActiveTab] = useState<string | null>('settings');
    //const [activeTab, setActiveTab] = useState<string | null>(filters[0].filter_name);

    const tabs = filters.map(x => <Tabs.Tab value={x.name}>{`${x.name}`}</Tabs.Tab>);
    const panels = filters.map(x => <Tabs.Panel value={x.name}>{renderComponent(x.filters)}</Tabs.Panel>);

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
                {renderComponent({})}
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
    const [editFilterModalOpened, { open: openEditFilterModal, close: closeEditFilterModal }] = useDisclosure(false);
    const [filterEdit, setFilterEdit] = useState<TabFilterDefinition | undefined>(undefined);

    const [successDelete, setSuccessDelete] = useState<string | undefined>();
    const [errorDelete, setErrorDelete] = useState<string | undefined>();

    const deleteFilterMutation = useMutation({
        mutationFn: async (filter: TabFilterDefinition) => {
            return await deleteFilter(filter.id);
        },  
        onError: (error) => {
            setSuccessDelete(undefined);
            setErrorDelete(error.message);
        },
        onSettled: (_data, _err, original, _result, context) => {
            setSuccessDelete('success');
            setErrorDelete(undefined);
            onDelete(original);

            context.client.invalidateQueries({ queryKey: [LIST_FILTERS]});
        },
    });

    return <>
        <ModalWrapper
            title="Add Filter"
            children={createFilter(closeAddFilterModal)}
            opened={addFilterModalOpened}
            close={closeAddFilterModal}
        />

        <ModalWrapper
            title="Edit Filter"
            children={editFilter(filterEdit, closeEditFilterModal)}
            opened={editFilterModalOpened}
            close={closeEditFilterModal}
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
                                    {x.name}
                                </Table.Td>
                                <Table.Td>
                                    {Object.keys(x.filters).map(y => <Pill>{y}: {x.filters[y]}</Pill>)}
                                </Table.Td>
                                <Table.Td>
                                    <Button
                                        onClick={() => {
                                            setFilterEdit(x);
                                            openEditFilterModal();
                                        }}
                                        variant="subtle"
                                        color="blue.9"
                                    >
                                        Edit
                                    </Button>
                                    <Button
                                            onClick={() => {
                                            deleteFilterMutation.mutate(x)
                                        }}
                                        variant="subtle"
                                        color="red.9"
                                    >
                                        Delete
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

    createFilter:       (close: () => void) => ReactNode;
    editFilter:         (filter: TabFilterDefinition | undefined, close: () => void) => ReactNode;
    renderComponent:    (filter: TabFilterFilterDefinition) => ReactNode;

    onDelete?:          (filter: TabFilterDefinition) => void;
};

export type TabFilterSettingProps = {
    filters:        TabFilterDefinition[];

    createFilter:   (close: () => void) => ReactNode;
    editFilter:     (filter: TabFilterDefinition | undefined, close: () => void) => ReactNode;

    onDelete:       (filter: TabFilterDefinition) => void;
}

export type TabFilterDefinition = {
    id:         Uuid;
    name:       string;
    filters:    TabFilterFilterDefinition;
}

export type TabFilterFilterDefinition = { [key: string]: string };
