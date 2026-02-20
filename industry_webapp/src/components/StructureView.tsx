import { CopyText } from '@/components/CopyText';
import { Dotlan } from '@/components/Dotlan';
import { Group, Modal, Stack, Table, Title } from '@mantine/core';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { StructureRigBadge } from '../routes/structures/-components/StructureRigBadge';
import { StructureServiceBadge } from '../routes/structures/-components/StructureServiceBadge';
import { useFetchStructure } from '@/services/structure/fetch';
import type { Uuid } from '@/services/utils';
import { BlueprintBonusList } from './BlueprintBonusList';

export function StructureView({
    structureId,
}: StructureViewProps) {
    const {
        isPending,
        isError,
        data: structure,
    } = useFetchStructure(structureId, {
        include_installable: true,
    });

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const showServices = () => {
        if (structure.services.length === 0) {
            return <></>;
        }

        return <>
            <Title order={2}>
                Services
            </Title>
            <Group>
                <StructureServiceBadge
                    services={structure.services || []}
                    size='md'
                />
            </Group>
        </>
    }

    const showRigs = () => {
        if (structure.rigs.length === 0) {
            return <></>;
        }

        return <>
            <Title order={2}>
                Rigs
            </Title>
            <Group>
                <StructureRigBadge
                    rigs={structure.rigs || []}
                    size='md'
                />
            </Group>
        </>
    }

    return <>
        <Stack>
            <Title order={2}>Information</Title>

            <Table>
                <Table.Tbody>
                    <Table.Tr>
                        <Table.Th>Name</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={structure.name}
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>In-Game ID</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={structure.structure_id}
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Type</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={structure.item.name}
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>System</Table.Th>
                        <Table.Td>
                            <Dotlan
                                system={structure.system}
                            />
                        </Table.Td>
                    </Table.Tr>
                </Table.Tbody>
            </Table>

            {showServices()}

            {showRigs()}

            {
                <>
                    <Title order={2}>
                        Bonused Blueprints
                    </Title>
                    <BlueprintBonusList
                        rigs={ structure.rigs.map(x => x.item.type_id) }
                        services={ structure.services.map(x => x.type_id) }
                        systemSecurityStr={ structure.system.security_str }
                    />
                </>
            }
        </Stack>
    </>;
}

export function StructureViewModal({
    opened,
    onClose,

    structureId,
}: StructureViewModalProps) {
    return <Modal
        opened={ opened }
        onClose={ onClose }
        overlayProps={{
            backgroundOpacity: 0.55,
            blur: 3,
        }}
        size="70%"
        centered
        closeOnEscape
        closeOnClickOutside
    >
        <StructureView
            structureId={ structureId }
        />
    </Modal>
}

export type StructureViewProps = {
    structureId: Uuid;
}

export type StructureViewModalProps = {
    opened: boolean;
    onClose: () => void;

    structureId: Uuid;
}
