import { ActionIcon, Card, CopyButton, TextInput, Tooltip } from '@mantine/core';
import { createFileRoute } from '@tanstack/react-router'
import { faCopy } from '@fortawesome/free-regular-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useFetchProjectGroup } from '@starfoundry/components/services/project-group/fetch';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';

export const Route = createFileRoute(
    '/project-groups_/$projectGroupId/members',
)({
    component: RouteComponent,
})

function RouteComponent() {
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
