import { Group, Modal, Stack, Title } from '@mantine/core';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { StructureList } from './StructureList';
import { StructureRigBadge } from '../routes/structures/-components/StructureRigBadge';
import { StructureServiceBadge } from '../routes/structures/-components/StructureServiceBadge';
import { useFetchIndustryHub } from '@/services/industry-hub/fetch';
import type { Uuid } from '@/services/utils';

export function IndustryHubView({
    industryHubId,
}: IndustryHubViewProps) {
    const {
        isPending,
        isError,
        data: industryHub,
    } = useFetchIndustryHub(industryHubId);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    return <>
        <Stack>
            <Title order={1}>{industryHub.name}</Title>

            <Title order={2}>
                Services
            </Title>
            <Group>
                <StructureServiceBadge
                    services={industryHub.structures.flatMap(x => x.services) || []}
                    size='md'
                />
            </Group>

            <Title order={2}>
                Rigs
            </Title>
            <Group>
                <StructureRigBadge
                    rigs={industryHub.structures.flatMap(x => x.rigs) || []}
                    size='md'
                />
            </Group>

            <Title order={2}>
                Structures
            </Title>
            <StructureList
                structures={industryHub.structures}
            />
        </Stack>
    </>;
}

export function IndustryHubViewModal({
    opened,
    onClose,

    industryHubId,
}: IndustryHubViewModalProps) {
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
        <IndustryHubView
            industryHubId={ industryHubId }
        />
    </Modal>
}

export type IndustryHubViewProps = {
    industryHubId: Uuid;
}

export type IndustryHubViewModalProps = {
    opened: boolean;
    onClose: () => void;

    industryHubId: Uuid;
}
