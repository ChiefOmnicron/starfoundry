import { Button, Dialog, Group } from "@mantine/core";
import { useDisclosure, useMediaQuery } from "@mantine/hooks";
import { CheckResourcesModal } from "./CheckResourcesModal";
import { CreateBuildOrderModal } from "./CreateBuildOrderModal";
import type { Uuid } from "@internal/services/utils";

export function ProjectJobAction({
    selected,

    onCreated,
}: ProjectJobActionProps) {
    console.log(selected)
    const isMobile = useMediaQuery('(max-width: 50em)');

    const [checkResourcesModalOpened, { open: checkResourcesModalOpen, close: checkResourcesModalClose }] = useDisclosure(false);
    const [createBuildOrderModalOpened, { open: createBuildOrderModalOpen, close: createBuildOrderModalClose }] = useDisclosure(false);

    return <>
        {
            <CheckResourcesModal
                jobIds={selected.map(x => x.job_id)}
                close={checkResourcesModalClose}
                opened={checkResourcesModalOpened}
            />
        }

        {
            <CreateBuildOrderModal
                jobs={selected}
                close={createBuildOrderModalClose}
                opened={createBuildOrderModalOpened}
                onCreated={onCreated}
            />
        }

        <Dialog
            opened={selected.length > 0 && !(checkResourcesModalOpened || createBuildOrderModalOpened)}
            size="xl"
            position={{
                bottom: 50,
                right: isMobile ? '0' : '35%',
            }}
        >
            <Group grow>
                <Button
                    onClick={() => createBuildOrderModalOpen()}
                >
                    Create build order
                </Button>

                <Button
                    onClick={() => checkResourcesModalOpen()}
                >
                    Check resources
                </Button>
            </Group>
        </Dialog>
    </>
}

export type ProjectJobActionProps = {
    selected: ProjectJobMinimal[];

    onCreated: (id: Uuid) => void;
}

export type ProjectJobMinimal = {
    project_id: Uuid;
    job_id:     Uuid;
}
