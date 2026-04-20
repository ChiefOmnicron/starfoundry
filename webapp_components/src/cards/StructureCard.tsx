import { BaseCard } from "./BaseCard";
import { CopyText } from "@internal/misc/CopyText";
import { Dotlan } from "@internal/misc/Dotlan";
import { EveIcon } from "@internal/misc/EveIcon";
import { Flex, Group, Stack, Text, Title, UnstyledButton } from "@mantine/core";
import { InternalLink } from "@internal/links/InternalLink";
import { StructureRigBadge } from "@internal/structure/StructureRigBadge";
import { StructureServiceBadge } from "@internal/structure/StructureServiceBadge";
import { StructureViewModal } from "@internal/detailView/StructureView";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";
import type { Structure } from "@internal/services/structure/list";

export function StructureCard({
    structure,

    viewTarget = '_self',

    editLink = undefined,
    viewLink = true,

    checkable = false,
    checked = [],
    onChange = () => {},
}: StructureCardProps) {
    const [openedView, { open: openView, close: closeView }] = useDisclosure(false);
    const [isSelected, setIsSelected] = useState<boolean>(false);

    useEffect(() => {
        setIsSelected(!!checked.find(x => x.id === structure.id));
    }, [checked]);

    const structureName = structure
        .name
        .replace(`${structure.system.system_name} - `, '');

    const selectStructure = (
        state: 'checked' | 'unchecked',
    ) => {
        if (!checkable) {
            return;
        }

        // TODO: properly wrap the state
        setIsSelected(state === 'checked');

        if (state === 'checked') {
            onChange('checked', structure);
        } else {
            onChange('unchecked', structure);
        }
    }

    const header = () => {
        return <>
            <Group>
                <EveIcon
                    id={structure.item.type_id}
                />

                <Title order={3}>
                    <CopyText
                        display={structureName}
                        value={structure.name}
                    />
                </Title>
            </Group>
        </>
    }

    const body = () => {
        return <>
            <Stack
                gap="xs"
                justify='space-between'
                style={{
                    width: '100%'
                }}
            >
                <Group
                    gap={'xs'}
                >
                    <Text
                        size='sm'
                        fw={700}
                    >
                        System:
                    </Text>
                    <Text
                        size='sm'
                    >
                        <Dotlan system={structure.system} />
                    </Text>
                </Group>

                {
                    structure.services.length > 0
                    ?   <Group
                            gap={'xs'}
                        >
                            <Text
                                size='sm'
                                fw={700}
                            >
                                Services:
                            </Text>

                            <StructureServiceBadge
                                services={structure.services}
                            />
                        </Group>
                    : <></>
                }

                {
                    structure.rigs.length > 0
                    ?   <Group
                            gap={'xs'}
                        >
                            <Text
                                size='sm'
                                fw={700}
                            >
                                Rigs:
                            </Text>

                            <StructureRigBadge
                                rigs={structure.rigs}
                            />
                        </Group>
                    : <></>
                }
            </Stack>
        </>
    }

    const footer = () => {
        const edit = editLink
            ?   <InternalLink
                    to={editLink}
                    params={{
                        structureId: structure.id,
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
        <StructureViewModal
            opened={openedView}
            onClose={closeView}

            structure={structure}
            showBlueprintBonus
        />

        <BaseCard
            header={header()}
            footer={footer()}

            checkable={checkable}
            selected={isSelected}
            onCheckChange={selectStructure}
        >
            {body()}
        </BaseCard>
    </>
}

type StructureCardRequiredProps = {
    structure: Structure;

    viewTarget?: '_blank' | '_self'
}

export type StructureCardAdditionalProps = {
    editLink?: string;
    viewLink?: boolean;

    // Determines if a checkbox is added or not
    checkable?: boolean,
    checked?: Structure[];
    onChange?: (event: 'checked' | 'unchecked', structure: Structure) => void;
}

export type StructureCardProps = StructureCardRequiredProps & StructureCardAdditionalProps;
