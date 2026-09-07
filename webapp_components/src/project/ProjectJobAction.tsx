import { ActionBar, Button } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { CheckResourcesModal } from "./CheckResourcesModal";
import { CreateBuildOrderModal } from "./CreateBuildOrderModal";
import type { Uuid } from "../services/utils";

export function ProjectJobAction({
    selected,

    onCreated,
}: ProjectJobActionProps) {
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

        <ActionBar
            opened={selected.length > 0 && !(checkResourcesModalOpened || createBuildOrderModalOpened)}
        >
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
        </ActionBar>
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
