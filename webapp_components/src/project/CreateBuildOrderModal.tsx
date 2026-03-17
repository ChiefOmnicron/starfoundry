import { Alert, Button, Group, Stack, Textarea, Title } from "@mantine/core";
import { checkResources, type CheckMaterialsRequest, type CheckMaterialsResponse } from "@internal/services/projects/check_resources";
import { LoadingError } from "@internal/misc/LoadingError";
import { MaterialList } from "@internal/list/MaterialList";
import { ModalWrapper } from "@internal/wrapper/modal";
import { useMutation } from "@tanstack/react-query";
import { useState } from "react";
import type { Uuid } from "@internal/services/utils";
import { BlueprintList } from "@internal/list/BlueprintList";

export function CreateBuildOrderModal({
    jobIds,

    opened,
    close,
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

    const checkResourcesClick = () => {
        checkResourceMutation.mutate({
            job_ids:        jobIds,
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
    jobIds: Uuid[];

    opened: boolean;
    close: () => void;
}
