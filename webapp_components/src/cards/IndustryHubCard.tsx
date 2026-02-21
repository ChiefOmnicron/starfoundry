import { Card, Checkbox, Flex, Group, Stack, Text, Title, UnstyledButton } from "@mantine/core";
import { cloneIndustryHub } from "@internal/services/industry-hub/clone";
import { CopyText } from "@internal/misc/CopyText";
import { Dotlan } from "@internal/misc/Dotlan";
import { IndustryHubViewModal } from "@internal/detailView/IndustryHubView";
import { InternalLink } from "@internal/links/InternalLink";
import { StructureRigBadge } from "@internal/structure/StructureRigBadge";
import { StructureServiceBadge } from "@internal/structure/StructureServiceBadge";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { useMutation } from "@tanstack/react-query";
import type { IndustryHub } from "@internal/services/industry-hub/list";
import type { Item } from "@internal/services/item/model";
import type { StructureRig, System } from "@internal/services/structure/list";

export function IndustryHubCard({
    industryHub,

    viewTarget = '_self',

    cloneLink = false,
    editLink  = undefined,
    viewLink  = true,

    onCloneSuccess = () => {},
    onCloneError = () => {},

    checkable = false,
    checked = [],
    onChange = () => {},
}: IndustryHubCardProp) {
    const [openedView, { open: openView, close: closeView }] = useDisclosure(false);
    const [isSelected, setIsSelected] = useState<boolean>(false);

    useEffect(() => {
        setIsSelected(!!checked.find(x => x.id === industryHub.id));
    }, [checked]);

    const cloneMutation = useMutation({
        mutationFn: () => {
            return cloneIndustryHub(industryHub.id);
        },
        onSuccess: () => {
            onCloneSuccess();
        },
        onError: (error) => {
            onCloneError(error.message);
        }
    });

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
        const clone = cloneLink
            ?   <UnstyledButton
                    onClick={() => {
                        cloneMutation.mutate();
                    }}
                    style={{
                        color: 'var(--mantine-color-blue-4)',
                        fontSize: 'var(--mantine-font-size-sm)',
                    }}
                >
                    Clone
                </UnstyledButton>
            :   <></>
        const edit = editLink
            ?   <InternalLink
                    to={editLink}
                    params={{
                        industryHubId: industryHub.id,
                    } as any}
                    target={viewTarget}
                    content='Edit'
                />
            :   <></>
        const view = viewLink
            ?   <UnstyledButton
                    onClick={openView}
                    style={{
                        color: 'var(--mantine-color-blue-4)',
                        fontSize: 'var(--mantine-font-size-sm)',
                    }}
                >
                    View
                </UnstyledButton>
            :   <></>

        if (cloneLink || editLink || viewLink) {
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
                        { clone }

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

            industryHub={industryHub}
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

type IndustryHubCardRequiredProps = {
    industryHub: IndustryHub;

    viewTarget?: '_blank' | '_self'
}

export type IndustryHubCardAdditionalProps = {
    cloneLink?: boolean;
    viewLink?:  boolean;
    editLink?: string;

    onCloneSuccess?: () => void;
    onCloneError?: (error: string) => void;

    // Determines if a checkbox is added or not
    checkable?: boolean,
    checked?: IndustryHub[];
    onChange?: (event: 'checked' | 'unchecked', industry: IndustryHub) => void;
}

export type IndustryHubCardProp = IndustryHubCardRequiredProps & IndustryHubCardAdditionalProps;
