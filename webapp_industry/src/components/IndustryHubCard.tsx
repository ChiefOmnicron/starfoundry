import { Card, Checkbox, Flex, Group, SimpleGrid, Stack, Text, Title, UnstyledButton } from "@mantine/core";
import { Dotlan } from "@/components/Dotlan";
import { InternalLink } from "./InternalLink";
import { Route as IndustryHubRoute } from '@/routes/industry-hubs_/$industryHubId.index';
import type { IndustryHub } from "@/services/industry-hub/list";
import type { Item } from "@/services/item/model";
import type { StructureRig, System } from "@/services/structure/list";
import { CopyText } from "./CopyText";
import { StructureServiceBadge } from "@/routes/structures/-components/StructureServiceBadge";
import { StructureRigBadge } from "@/routes/structures/-components/StructureRigBadge";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { IndustryHubViewModal } from "./IndustryHubView";

export function IndustryHubList({
    industryHubs,

    viewTarget = '_self',

    industryHubCardProps,
}: IndustryHubListProp) {
    const industryHubCard = () => {
        return industryHubs
            .map(x => <IndustryHubCard
                    key={x.id}
                    industryHub={x}
                    viewTarget={viewTarget}
                    {...industryHubCardProps}
                />
            );
    }

    return <>
        <SimpleGrid
            cols={{
                base: 1,
                sm: 2,
            }}
        >
            { industryHubCard() }
        </SimpleGrid>
    </>
}

export function IndustryHubCard({
    industryHub,

    viewTarget = '_self',

    editLink = false,
    viewLink = true,

    checkable = false,
    checked = [],
    onChange = () => {},
}: IndustryHubCardProp) {
    const [openedView, { open: openView, close: closeView }] = useDisclosure(false);
    const [isSelected, setIsSelected] = useState<boolean>(false);

    useEffect(() => {
        setIsSelected(!!checked.find(x => x.id === industryHub.id));
    }, [checked]);

    const systems: System[] = [];
    industryHub
        .structures
        .map(x => x.system)
        .forEach(x => {
            const exists = systems.find(y => y.system_id === x.system_id);

            if (!exists) {
                systems.push(x);
            }
        })

    const rigs: StructureRig[] = [];
    industryHub
        .structures
        .flatMap(x => x.rigs)
        .forEach(x => {
            const exists = rigs.find(y => y.item.type_id === x.item.type_id);

            if (!exists) {
                rigs.push(x);
            }
        });

    const services: Item[] = [];
    industryHub
        .structures
        .flatMap(x => x.services)
        .forEach(x => {
            const exists = services.find(y => y.type_id === x.type_id);

            if (!exists) {
                services.push(x);
            }
        });

    const selectIndustryHub = (
        state: boolean,
    ) => {
        if (!checkable) {
            return;
        }

        setIsSelected(state);

        if (state) {
            onChange('checked', industryHub);
        } else {
            onChange('unchecked', industryHub);
        }
    }

    const checkbox = () => {
        if (!checkable) {
            return <></>
        }

        return <>
            <Checkbox
                checked={isSelected}
                size='xs'
                onChange={(event) => {
                    selectIndustryHub(event.currentTarget.checked);
                }}
            />
        </>
    }

    const card = () => {
        return <>
            <Stack
                gap="xs"
                justify='space-between'
                style={{
                    width: '100%'
                }}
            >
                <Group
                    justify='space-between'
                >
                    <Title order={3}>
                        <CopyText
                            value={industryHub.name}
                        />
                    </Title>

                    { checkbox() }
                </Group>

                <Group
                    gap={'xs'}
                >
                    <Text
                        size='sm'
                        fw={700}
                    >
                        System(s):
                    </Text>
                    <Text
                        size='sm'
                    >
                        { systems.map(x => <><Dotlan system={x} /> </>) }
                    </Text>
                </Group>

                <Group
                    gap={'xs'}
                >
                    <Text
                        size='sm'
                        fw={700}
                    >
                        Service(s):
                    </Text>

                    <StructureServiceBadge
                        services={services}
                    />
                </Group>

                <Group
                    gap={'xs'}
                >
                    <Text
                        size='sm'
                        fw={700}
                    >
                        Rig(s):
                    </Text>

                    <StructureRigBadge
                        rigs={rigs}
                    />
                </Group>
            </Stack>
        </>
    }

    const actionBar = () => {
        const edit = editLink
            ?   <InternalLink
                    to={IndustryHubRoute.to}
                    params={{
                        industryHubId: industryHub.id,
                    } as any}
                    target={viewTarget}
                    content='Edit'
                />
            :   <></>
        const view = viewLink
            ?   <UnstyledButton
                    onClick={ openView }
                    style={{
                        color: 'var(--mantine-color-blue-4)',
                        fontSize: 'var(--mantine-font-size-sm)',
                    }}
                >
                    View
                </UnstyledButton>
            :   <></>

        if (editLink || viewLink) {
            return <>
                <Flex
                    align='flex-end'
                    justify='flex-end'
                    style={{
                        backgroundColor: 'rgba(93,93,104, 0.1)',
                        padding: '5px',
                    }}
                >
                    <Group>
                        { edit }

                        { view }
                    </Group>
                </Flex>
            </>
        } else {
            return <></>
        }
    }

    return <>
        <IndustryHubViewModal
            opened={openedView}
            onClose={closeView}

            industryHubId={industryHub.id}
        />

        <Card
            key={ industryHub.id }
            style={{
                padding: 0,
                border: isSelected ? '1px solid var(--mantine-color-blue-9)' : '',
            }}
        >
            <Card.Section
                onClick={ () => {
                    selectIndustryHub(!isSelected);
                }}
                style={{
                    margin: '10px',
                    height: '100%'
                }}
            >
                { card() }
            </Card.Section>

            { actionBar() }
        </Card>
    </>
}

export type IndustryHubListProp = {
    industryHubs: IndustryHub[];

    viewTarget?: '_blank' | '_self';

    industryHubCardProps?: IndustryHubCardPAdditionalProps;
}

type IndustryHubCardMandatoryProps = {
    industryHub: IndustryHub;

    viewTarget?: '_blank' | '_self'
}

type IndustryHubCardPAdditionalProps = {
    editLink?: boolean;
    viewLink?: boolean;

    // Determines if a checkbox is added or not
    checkable?: boolean,
    checked?: IndustryHub[];
    onChange?: (event: 'checked' | 'unchecked', industry: IndustryHub) => void;
}

export type IndustryHubCardProp = IndustryHubCardMandatoryProps & IndustryHubCardPAdditionalProps;
