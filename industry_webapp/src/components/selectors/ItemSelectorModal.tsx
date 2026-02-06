import { Button, Checkbox, Flex, InputBase, Modal, Stack, Table, Text, UnstyledButton } from "@mantine/core";
import { EveIcon } from "../EveIcon";
import { listItem, type ItemFilter } from "@/services/item/list";
import { LoadingAnimation } from "../LoadingAnimation";
import { useDebouncedCallback } from "@mantine/hooks";
import { useEffect, useState, type JSX, type ReactElement } from "react";
import type { Item } from "@/services/item/model";

export function ItemSelectorModal({
    opened,
    onClose,
    onSelect,

    selected,

    blueprint = false,
    buildable = false,
}: ItemSelectorModalProp): ReactElement {
    // all items selected by the user
    const [selectedItems, setSelectedItems] = useState<Item[]>([]);
    // list of items that can be selected
    const [selectableItems, setSelectableItems] = useState<Item[]>([]);

    const [loading, setLoading] = useState<boolean>(false);
    const [_, setSearch] = useState('');

    useEffect(() => {
        setSearch('');
        setSelectedItems(selected);
        setSelectableItems([]);
    }, [opened, selected]);

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
                setLoading(false);
                setSelectableItems(x);
            })
    }, 200);

    const itemRow = (
        items:          Item[],
        ignoreSelected: boolean,
    ) => {
        return items
            .filter(x => {
                const isInSelectedItems = selectedItems.find(y => y.type_id === x.type_id);
                // item is is not in the selected items
                if (!isInSelectedItems) {
                    return true;
                } else if (ignoreSelected && isInSelectedItems) {
                    return true;
                } else {
                    return false;
                }
            })
            .map(item => {
                return <>
                    <Table.Tr
                        key={item.name}
                    >
                        <Table.Td>
                            <Checkbox
                                aria-label="Select item"
                                checked={selectedItems.find(y => y.type_id === item.type_id) ? true : false}
                                onChange={(event) => setSelectedItems(
                                        event.currentTarget.checked
                                            ? [...selectedItems, item]
                                            : selectedItems.filter((y) => y.type_id !== item.type_id)
                                    )
                                }
                            />
                        </Table.Td>
                        <Table.Td>
                            <EveIcon
                                id={item.type_id}
                                type={
                                    item.name.endsWith('Blueprint')
                                    ? 'bp'
                                    : 'icon'
                                }
                            />
                        </Table.Td>
                        <Table.Td>{item.name}</Table.Td>
                        <Table.Td>{item.group.name}</Table.Td>
                        <Table.Td>{item.category.name}</Table.Td>
                    </Table.Tr>
                </>
            });
    }

    const tableWrapper = (entries: JSX.Element[]) => {
        return <Table>
            <Table.Thead>
                <Table.Tr>
                    <Table.Th />
                    <Table.Th
                        style={{
                            width: '5%'
                        }}
                    />
                    <Table.Th
                        style={{
                            width: '50%'
                        }}
                    >
                        Item name
                    </Table.Th>
                    <Table.Th
                        style={{
                            width: '25%'
                        }}
                    >
                        Group
                    </Table.Th>
                    <Table.Th
                        style={{
                            width: '25%'
                        }}
                    >
                        Category
                    </Table.Th>
                </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
                {
                    entries.length > 0
                    ? entries
                    : <Table.Tr>
                        <Table.Td colSpan={10}>
                            <Flex
                                justify="center"
                                align="center"
                            >
                                <Text>No data yet</Text>
                            </Flex>
                        </Table.Td>
                    </Table.Tr>
                }
            </Table.Tbody>
        </Table>
    }

    const itemTable = () => {
        if (loading) {
            return <LoadingAnimation />;
        }

        return tableWrapper(itemRow(selectableItems, false));
    }

    const selectedItemTable = () => {
        return tableWrapper(itemRow(selectedItems, true));
    }

    const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
        setSearch(event.currentTarget.value);
        fetchData(event.currentTarget.value);
    };

    return <Modal
        opened={ opened }
        onClose={ onClose }
        title="Items"
        overlayProps={{
            backgroundOpacity: 0.55,
            blur: 3,
        }}
        size="70%"
        centered
        closeOnEscape
        closeOnClickOutside
    >
        <Stack>
            <InputBase
                name="Name"
                description='Search for the name of an item'
                placeholder="Fuel block"
                onChange={handleSearch}
            ></InputBase>

            <Text>Search result</Text>
            { itemTable() }

            <Text>Selected items</Text>
            { selectedItemTable() }

            <Flex
                justify='flex-end'
                gap='xs'
            >
                <UnstyledButton
                    onClick={ onClose }
                >
                    Close without change
                </UnstyledButton>
                <Button
                    onClick={ () => onSelect(selectedItems) }
                >
                    Select ({ selectedItems.length }) items
                </Button>
            </Flex>
        </Stack>
    </Modal>
}

export type ItemSelectorModalProp = {
    opened: boolean;
    onSelect: (entry: Item[]) => void;
    onClose: () => void;

    // list of values that are already selected, and filtered out
    selected:   Item[],

    blueprint?: boolean;
    buildable?: boolean;
}
