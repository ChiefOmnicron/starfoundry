import { Alert, Button, Group, Stack, Textarea, Title } from "@mantine/core";
import { BlueprintList } from "@internal/list/BlueprintList";
import { checkResources, type CheckMaterialsRequest, type CheckMaterialsResponse } from "@internal/services/projects/checkResource";
import { createJobOrder } from "@internal/services/projects/createJobOrder";
import { LoadingError } from "@internal/misc/LoadingError";
import { MaterialList } from "@internal/list/MaterialList";
import { ModalWrapper } from "@internal/wrapper/Modal";
import { useMutation } from "@tanstack/react-query";
import { useState } from "react";
import type { ProjectJobMinimal } from "./ProjectJobAction";
import type { Uuid } from "@internal/services/utils";

export function CreateBuildOrderModal({
    jobs,

    opened,
    close,

    onCreated,
}: CreateBuildOrderModalProps) {
    const [hasError, setHasError] = useState<boolean>(false);
    const [checkResult, setCheckResult] = useState<CheckMaterialsResponse | undefined>(undefined);
    const [existingMaterials, setExistingMaterials] = useState<string>('');

    const checkResourceMutation = useMutation({
        mutationFn: async (data: CheckMaterialsRequest) => {
            return await checkResources(data);
        },
        onSuccess: (data) => {
            setHasError(false);

            data.materials = data.materials.filter(x => x.quantity > 0);
            setCheckResult(data);
        },
    });

    const createJobOrderMutation = useMutation({
        mutationFn: async () => {
            return await createJobOrder(jobs);
        },
        onSuccess: (data) => {
            setHasError(false);
            onCreated(data.id);
        },
    });

    const checkResourcesClick = () => {
        checkResourceMutation.mutate({
            job_ids:        jobs.map(x => x.job_id),
            materials_str:  existingMaterials,
        });
    }

    const tableContent = () => {
        if (checkResult) {
            if (checkResult.materials.length > 0) {
                return <>
                    <Alert
                        color="yellow"
                    >
                        Missing materials
                    </Alert>

                    <MaterialList
                        materials={checkResult.materials}
                    />
                </>
            } else {
                return <>
                    <Alert
                        color="green"
                    >
                        No missing materials.
                    </Alert>

                    <Title order={4}>Needed Blueprints</Title>
                    <BlueprintList
                        blueprints={checkResult.blueprints}
                    />

                    <Group
                        justify="flex-end"
                    >
                        <Button
                            onClick={() => createJobOrderMutation.mutate()}
                        >
                            Create
                        </Button>
                    </Group>
                </>
            }
        } else {
            return <></>
        }
    }

    const showError = () => {
        if (hasError) {
            return LoadingError();
        }
    }

    return <ModalWrapper
        close={() => {
            setCheckResult(undefined);
            setExistingMaterials('');
            close();
        }}
        opened={opened}
        title="Create Build Order"
        size="50%"
    >
        <Stack>
            {showError()}

            <Textarea
                label="Stored materials"
                description="Insert the materials you already have (can be empty)"
                placeholder="Tritanium 100&#10;Pyerite 500"
                resize="vertical"
                rows={10}
                value={existingMaterials}
                onChange={(event) => setExistingMaterials(event.currentTarget.value)}
            />

            <Group
                justify="flex-end"
            >
                <Button
                    onClick={checkResourcesClick}
                    loading={checkResourceMutation.isPending}
                    disabled={checkResourceMutation.isPending}
                >
                    Check resources
                </Button>
            </Group>

            {tableContent()}
        </Stack>
    </ModalWrapper>
}

export type CreateBuildOrderModalProps = {
    jobs: ProjectJobMinimal[];

    opened: boolean;
    close: () => void;

    onCreated: (id: Uuid) => void;
}
