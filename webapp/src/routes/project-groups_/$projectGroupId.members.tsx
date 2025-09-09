import { ActionIcon, Card, CopyButton, TextInput, Tooltip } from '@mantine/core';
import { createFileRoute } from '@tanstack/react-router'
import { faCopy } from '@fortawesome/free-regular-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { LoadingError } from '@/components/LoadingError';
import { useFetchProjectGroup } from '@/services/project-group/fetch';
import LoadingAnimation from '@/components/LoadingAnimation';

export const Route = createFileRoute(
    '/project-groups_/$projectGroupId/members',
)({
    component: ProjectGroupMembers,
})

export function ProjectGroupMembers() {
    const { projectGroupId } = Route.useParams();

    const {
        isError,
        isPending,
        data: projectGroup,
    } = useFetchProjectGroup(projectGroupId);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError && !projectGroup) {
        return LoadingError();
    }

    const inviteLink = `${window.location}/invite`;

    return <>
        <Card
            mt="sm"
        >
            <TextInput
                data-cy="inviteLink"
                label="Invite Link"
                description="Invite new characters to this project group"
                value={ inviteLink }
                rightSection={
                    <CopyButton
                        value={ inviteLink }
                    >
                        {
                            ({ copied, copy }) => (
                                <Tooltip label={copied ? 'Copied' : 'Copy'}>
                                    <ActionIcon onClick={copy}>
                                        {<FontAwesomeIcon icon={faCopy} />}
                                    </ActionIcon>
                                </Tooltip>
                            )
                        }
                    </CopyButton>
                }
                disabled
            />
        </Card>
    </>
}
