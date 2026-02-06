import { Card, Checkbox, Flex, Group, SimpleGrid, Stack, Text, Title, UnstyledButton } from '@mantine/core';
import { CopyText } from '@/components/CopyText';
import { Dotlan } from './Dotlan';
import { EveIcon } from './EveIcon';
import { InternalLink } from './InternalLink';
import { Route as StructureEditRoute } from '@/routes/structures_/$structureId.index';
import { StructureRigBadge } from '@/routes/structures/-components/StructureRigBadge';
import { StructureServiceBadge } from '@/routes/structures/-components/StructureServiceBadge';
import { StructureViewModal } from './StructureView';
import { useDisclosure } from '@mantine/hooks';
import { useEffect, useState } from 'react';
import type { Structure, System } from '@/services/structure/list';

export function StructureList({
    structures,

    viewTarget = '_self',
    groupBySystem = true,

    structureCardProps,
}: StructureListProps) {
    const systems: System[] = [];
    structures
        .map(x => {
            if (!systems.find(y => y.system_id === x.system.system_id)) {
                systems.push(x.system);
            }
        });

    const structureCardBySystem = (systemId: number) => {
        return structures
            .filter(x => x.system.system_id == systemId)
            .map(x => <StructureCard
                    key={x.id}
                    structure={x}
                    viewTarget={viewTarget}
                    {...structureCardProps}
                />
            );
    }

    if (groupBySystem) {
        return systems
            .map(x => {
                return <>
                    <Title
                        order={2}
                        mt='xs'
                    >
                        { x.system_name }
                    </Title>

                    <SimpleGrid
                        cols={{
                            base: 1,
                            sm: 2,
                        }}
                    >
                        { structureCardBySystem(x.system_id) }
                    </SimpleGrid>
                </>
            });
    } else {
        return <>
            <SimpleGrid
                cols={{
                    base: 1,
                    sm: 2,
                }}
            >
                {
                    structures
                        .map(x => <StructureCard
                                structure={x}
                                viewTarget={viewTarget}
                                {...structureCardProps}
                            />
                        )
                }
            </SimpleGrid>
        </>;
    }
}

export function StructureCard({
    structure,

    viewTarget = '_self',

    editLink = false,
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
        state: boolean,
    ) => {
        if (!checkable) {
            return;
        }

        setIsSelected(state);

        if (state) {
            onChange('checked', structure);
        } else {
            onChange('unchecked', structure);
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
                    selectStructure(event.currentTarget.checked);
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

                    { checkbox() }
                </Group>

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

    const actionBar = () => {
        const edit = editLink
            ?   <InternalLink
                    to={StructureEditRoute.to}
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

            structureId={structure.id}
        />

        <Card
            key={ structure.id }
            style={{
                padding: 0,
                border: isSelected ? '1px solid var(--mantine-color-blue-9)' : '',
            }}
        >
            <Card.Section
                onClick={ () => {
                    selectStructure(!isSelected);
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

export type StructureListProps = {
    structures: Structure[];

    viewTarget?:    '_blank' | '_self';
    groupBySystem?: boolean;

    structureCardProps?: StructureCardAdditionalProps;
}

type StructureCardMandatoryProps = {
    structure: Structure;

    viewTarget?: '_blank' | '_self'
}

type StructureCardAdditionalProps = {
    editLink?: boolean;
    viewLink?: boolean;

    // Determines if a checkbox is added or not
    checkable?: boolean,
    checked?: Structure[];
    onChange?: (event: 'checked' | 'unchecked', structure: Structure) => void;
}

export type StructureCardProps = StructureCardMandatoryProps & StructureCardAdditionalProps;
