import { Button, Flex, InputBase, Stack, UnstyledButton } from "@mantine/core";
import { type IndustryHub } from "@internal/services/industry-hub/list";
import { useEffect, useState, type ReactElement } from "react";
import { IndustryHubList } from "@internal/list/IndustryHubList";
import { ModalWrapper } from "@internal/wrapper/Modal";

export function IndustryHubSelectorModal({
    opened,
    onClose,

    industryHubs,
    selected = [],

    onSelect,
}: IndustryHubSelectorModalProp): ReactElement {
    // all industry hubs selected by the user
    const [selectedIndustryHubs, setSelectedIndustryHubs] = useState<IndustryHub[]>([]);
    const [_, setSearch] = useState('');

    useEffect(() => {
        setSelectedIndustryHubs(selected);
    }, [opened, selected]);

    const industryHubList = () => {
        return <IndustryHubList
            industryHubs={industryHubs}

            industryHubCardProps={{
                checkable: true,
                checked: selectedIndustryHubs,
                onChange: (event: 'checked' | 'unchecked', industryHub: IndustryHub) => {
                    setSelectedIndustryHubs(
                        event === 'checked'
                            ? [...selectedIndustryHubs, industryHub]
                            : selectedIndustryHubs.filter((y) => y.id !== industryHub.id)
                    );
                }
            }}
        />
    }

    const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
        setSearch(event.currentTarget.value);
    };

    return <ModalWrapper
        opened={opened}
        close={onClose}
        title="Structures"
    >
        <Stack>
            <InputBase
                name="Name"
                description='Search for the name of the structure'
                placeholder="Jita 4-4"
                onChange={handleSearch}
            ></InputBase>

            { industryHubList() }

            <Flex
                justify='flex-end'
                gap='xs'
            >
                <UnstyledButton
                    onClick={onClose}
                >
                    Close without change
                </UnstyledButton>
                <Button
                    onClick={() => onSelect(selectedIndustryHubs)}
                >
                    Select ({ selectedIndustryHubs.length }) structures
                </Button>
            </Flex>
        </Stack>
    </ModalWrapper>
}

export type IndustryHubSelectorModalProp = {
    // modal control
    opened: boolean;
    onSelect: (entry: IndustryHub[]) => void;
    onClose: () => void;

    // industry hubs the user can select
    industryHubs: IndustryHub[],
    // list of values that are already selected
    selected:     IndustryHub[],
}
