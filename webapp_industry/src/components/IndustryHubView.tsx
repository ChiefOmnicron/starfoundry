import { Group, Modal, Stack, Title } from '@mantine/core';
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { StructureList } from './StructureList';
import { StructureRigBadge } from '../routes/structures/-components/StructureRigBadge';
import { StructureServiceBadge } from '../routes/structures/-components/StructureServiceBadge';
import { useFetchIndustryHub } from '@/services/industry-hub/fetch';
import type { Uuid } from '@/services/utils';
import type { IndustryHub } from '@/services/industry-hub/list';

export function IndustryHubView({
    industryHub,
}: IndustryHubViewProps) {
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

    industryHub,
    industryHubId,
}: IndustryHubViewModalProps) {
    let hub;
    if (industryHubId) {
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

        hub = industryHub;
    } else if (industryHub) {
        hub = industryHub;
    } else {
        console.error('either a "industryHub" or a "industryHubId" is required');
        return <></>
    }

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
            industryHub={hub}
        />
    </Modal>
}

export type IndustryHubViewProps = {
    industryHub: IndustryHub;
}

export type IndustryHubViewModalProps = {
    opened: boolean;
    onClose: () => void;

    industryHub?:   IndustryHub,
    industryHubId?: Uuid;
}
