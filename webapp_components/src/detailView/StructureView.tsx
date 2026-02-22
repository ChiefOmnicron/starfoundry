import { BlueprintBonusList } from '../list/BlueprintBonusList';
import { CopyText } from '@internal/misc/CopyText';
import { Dotlan } from '@internal/misc/Dotlan';
import { Group, Modal, Stack, Table, Title } from '@mantine/core';
import { StructureRigBadge } from '@internal/structure/StructureRigBadge';
import { StructureServiceBadge } from '@internal/structure/StructureServiceBadge';
import type { Structure } from '@internal/services/structure/list';
import { TAXES_SERVICE_MODULES } from '@internal/structure/TaxesByService';

export function StructureView({
    structure,
    showBlueprintBonus = false,
}: StructureViewProps) {
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

    const taxes = () => {
        if (Object.keys(structure.taxes).length === 0) {
            return <></>;
        }

        const name = (serviceTypeId: number) => {
            const result = TAXES_SERVICE_MODULES
                .find(x => x.serviceTypeId == serviceTypeId as number);

            return result
                ? result.name
                : 'Unknown Service'
        }

        return Object
            .keys(structure.taxes)
            .map((x: any) => {
                return <Table.Tr>
                    <Table.Th>Tax: { name(x) }</Table.Th>
                    <Table.Td>
                        <CopyText
                            value={structure.taxes[x]}
                        />%
                    </Table.Td>
                </Table.Tr>
            })
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
                    { taxes() }
                </Table.Tbody>
            </Table>

            {showServices()}

            {showRigs()}

            {
                showBlueprintBonus
                ? <>
                        <Title order={2}>
                            Bonused Blueprints
                        </Title>
                        <BlueprintBonusList
                            rigs={ structure.rigs.map(x => x.item.type_id) }
                            services={ structure.services.map(x => x.type_id) }
                            systemSecurityStr={ structure.system.security_str }
                        />
                    </>
                : <></>
            }
        </Stack>
    </>;
}

export function StructureViewModal({
    opened,
    onClose,

    structure,
    showBlueprintBonus,
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
            structure={structure}
            showBlueprintBonus={showBlueprintBonus}
        />
    </Modal>
}

export type StructureViewProps = {
    structure: Structure;
    showBlueprintBonus: boolean;
}

export type StructureViewModalProps = {
    opened: boolean;
    onClose: () => void;

    structure: Structure;
    showBlueprintBonus: boolean;
}
