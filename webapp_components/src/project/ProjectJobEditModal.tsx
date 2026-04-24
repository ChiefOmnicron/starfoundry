import { addExcessEntry, type AddExcessEntryRequest } from "@internal/services/projects/addExcess";
import { addJobEntry, type AddJobEntryRequest } from "@internal/services/projects/addJob";
import { addMarketEntry, type AddMarketEntryRequest } from "@internal/services/projects/addMarket";
import { Alert, Button, Grid, Group, NumberInput, SegmentedControl, Stack, Tabs, Text } from "@mantine/core";
import { LIST_PROJECT_JOBS, type ProjectJob, type ProjectJobStatus } from "@internal/services/projects/listJobs";
import { LoadingError } from "@internal/misc/LoadingError";
import { MaterialList } from "@internal/list/MaterialList";
import { ModalWrapper } from "@internal/wrapper/Modal";
import { splitJobCheck, type SplitJobRequest, type SplitJobResponse } from "@internal/services/projects/splitJobCheck";
import { updateProjectJob, type UpdateProjectJob } from "@internal/services/projects/updateJob";
import { useEffect, useState } from "react";
import { useMutation } from "@tanstack/react-query";
import type { Uuid } from "@internal/services/utils";

const SELECTABLE_STATES = [{
    label: 'Waiting for materials',
    value: 'WAITING_FOR_MATERIALS'
}, {
    label: 'Ready to start',
    value: 'READY_TO_START',
    disabled: true,
}, {
    label: 'Building',
    value: 'BUILDING'
}, {
    label: 'Done',
    value: 'DONE'
}];

export function ProjectJobEditModal({
    projectId,
    job,

    opened,
    close,
}: ProjectJobEditModalProps) {
    const [hasError, setHasError] = useState<boolean>(false);
    const [updateSuccess, setUpdateSuccess] = useState<boolean>(false);

    const [cost, setCost] = useState<number>(0);
    const [status, setStatus] = useState<ProjectJobStatus>('WAITING_FOR_MATERIALS');

    const [currentRuns, setCurrentRuns] = useState<number>(0);
    const [splitRuns, setSplitRuns] = useState<number>(0);

    const [hasChecked, setHasChecked] = useState<boolean>(false);
    const [checkedJobChange, setCheckedJobChange] = useState<SplitJobResponse>({
        excess: [],
        jobs: [],
        materials: [],
    });

    const saveJobMutation = useMutation({
        mutationFn: async (data: UpdateProjectJob) => {
            return await updateProjectJob(
                projectId,
                job.id,
                data,
            );
        },
        onMutate: async (updatedJob, context) => {
            await context.client.cancelQueries({ queryKey: [LIST_PROJECT_JOBS, projectId] });
            const previousJobs = context.client.getQueryData([LIST_PROJECT_JOBS, projectId]);

            context.client.setQueryData([LIST_PROJECT_JOBS, projectId], (jobs: ProjectJob[]) => {
                const old = jobs.find(x => x.id === job.id);
                if (job) {
                    job.cost = updatedJob.cost;
                    job.status = updatedJob.status;
                }
                return old;
            });

            return { jobs: previousJobs }
        },
        onError: (_err, _new, onMutateResult, context) => {
            setHasError(true);

            if (onMutateResult) {
                context.client.setQueryData([LIST_PROJECT_JOBS, projectId], onMutateResult.jobs);
            }
        },
        onSuccess: (_) => {
            setHasError(false);
            setUpdateSuccess(true);
        },
    });

    const splitJobCheckMutation = useMutation({
        mutationFn: async (data: SplitJobRequest) => {
            return await splitJobCheck(projectId, data);
        },
        onSuccess: (data) => {
            setHasError(false);
            setCheckedJobChange(data);
            setHasChecked(true);
        },
    });

    const addExcessMutation = useMutation({
        mutationFn: async (data: AddExcessEntryRequest[]) => {
            return await addExcessEntry(projectId, data);
        },
        onSuccess: () => {
            setHasError(false);
        },
        onError: () => {
            setHasError(true);
        }
    });

    const addJobMutation = useMutation({
        mutationFn: async (data: AddJobEntryRequest[]) => {
            return await addJobEntry(projectId, data);
        },
        onSuccess: () => {
            setHasError(false);
        },
        onError: () => {
            setHasError(true);
        }
    });

    const addMarketMutation = useMutation({
        mutationFn: async (data: AddMarketEntryRequest[]) => {
            return await addMarketEntry(projectId, data);
        },
        onSuccess: () => {
            setHasError(false);
        },
        onError: () => {
            setHasError(true);
        }
    });

    useEffect(() => {
        setCost(job.cost || 0);
        setStatus(job.status);

        setCurrentRuns(job.runs);
        setSplitRuns(0);
    }, [job]);

    const updateJob = () => {
        saveJobMutation.mutate({
            cost:   cost ? cost : undefined,
            status: status,
        });
    }

    const splitJobCheckClick = () => {
        splitJobCheckMutation
            .mutate({
                old: {
                    type_id: job.item.type_id,
                    runs: job.runs,
                },
                new: [{
                    type_id: job.item.type_id,
                    runs: currentRuns,
                }, {
                    type_id: job.item.type_id,
                    runs: splitRuns,
                }]
            });
    }

    const saveSplitJob = () => {
        let excess = checkedJobChange
                .excess
                .map(x => {
                    return {
                        quantity: x.quantity,
                        type_id:  x.item.type_id,
                    }
                });
        let jobs = checkedJobChange
                .jobs
                .map(x => {
                    return {
                        runs:           x.runs,
                        type_id:        x.item.type_id,
                        structure_id:   x.structure_id,
                    }
                });
        let market = checkedJobChange
                .materials
                .map(x => {
                    return {
                        quantity: x.quantity,
                        type_id:  x.item.type_id,
                    }
                });

        addExcessMutation.mutate(excess);
        addJobMutation.mutate(jobs);
        addMarketMutation.mutate(market);
    }

    const showJobChange = () => {
        if (checkedJobChange.jobs.length === 0 && checkedJobChange.materials.length === 0) {
            return <></>;
        }

        return <>
            {
                checkedJobChange.materials.length > 0
                ?   <>
                        <Alert>The following additional materials are required</Alert>
                        <MaterialList
                            materials={checkedJobChange.materials}
                        />
                    </>
                :   <></>
            }

            {
                checkedJobChange.jobs.length > 0
                ?   <>
                        <Alert>The following additional jobs are required</Alert>
                        <MaterialList
                            materials={checkedJobChange.jobs.map(x => {
                                return {
                                    item:       x.item,
                                    quantity:   x.runs,
                                }
                            })}
                        />
                    </>
                :   <></>
            }
        </>
    }

    const showError = () => {
        if (hasError) {
            return LoadingError();
        }
    }

    const showUpdateSuccess = () => {
        if (updateSuccess) {
            return <Alert
                variant='light'
                color='green'
                data-cy="updateSuccessful"
                onClose={ () => setUpdateSuccess(false) }
                withCloseButton
            >
                Job was successfully updated
            </Alert>;
        } else {
            return <></>;
        }
    }

    return <ModalWrapper
        close={() => {
            setCheckedJobChange({
                excess: [],
                jobs: [],
                materials: [],
            });
            close();
        }}
        opened={opened}
        title="Edit job"
        size="50%"
    >
        {showError()}

        <Tabs defaultValue="general">
            <Tabs.List>
                <Tabs.Tab
                    value="general"
                >
                    General
                </Tabs.Tab>
                <Tabs.Tab
                    value="split_job"
                >
                    Split Job
                </Tabs.Tab>
            </Tabs.List>

            <Tabs.Panel
                value="general"
            >
                <Stack>
                    {showUpdateSuccess()}

                    <Text size="sm" fw={500} mt={3}>
                        Status
                    </Text>
                    <SegmentedControl
                        fullWidth
                        data={SELECTABLE_STATES}
                        value={status}
                        onChange={(status: string) => {
                            setStatus(status as ProjectJobStatus);
                        }}
                    />

                    <NumberInput
                        label="Cost"
                        description="Cost it took to start the job"
                        placeholder="5000 ISK"
                        min={0}
                        allowDecimal={false}
                        thousandSeparator=","
                        suffix=" ISK"
                        value={cost}
                        onChange={(x) => {
                            setCost(x as number)
                        }}
                    />

                    <Group
                        justify="flex-end"
                    >
                        <Button
                            onClick={updateJob}
                            loading={saveJobMutation.isPending}
                            disabled={saveJobMutation.isPending}
                        >
                            Save
                        </Button>
                    </Group>
                </Stack>
            </Tabs.Panel>

            <Tabs.Panel
                value="split_job"
            >
                <Stack>
                    <Alert>
                        Splitting the job runs will create a new job entry, and if necessary add additional materials needed due to the change.
                    </Alert>

                    <Text size="sm" fw={500} mt={3}>
                        Total runs: {job.runs}
                    </Text>

                    <Grid>
                        <Grid.Col span={6}>
                            <NumberInput
                                value={currentRuns}
                                disabled
                            />
                        </Grid.Col>
                        <Grid.Col span={6}>
                            <NumberInput
                                value={splitRuns}
                                onChange={(value) => {
                                    if (!value) {
                                        setSplitRuns(0);
                                    } else {
                                        setSplitRuns(value as number);
                                    }

                                    setCurrentRuns(job.runs - (value as number));
                                }}
                                min={0}
                                max={job.runs - 1}
                            />
                        </Grid.Col>
                    </Grid>

                    {showJobChange()}

                    <Group
                        justify="flex-end"
                    >
                        <Button
                            onClick={splitJobCheckClick}
                            disabled={!(splitRuns > 0)}
                            loading={splitJobCheckMutation.isPending}
                        >
                            Check adjusted
                        </Button>
                        <Button
                            disabled={!hasChecked}
                            onClick={saveSplitJob}
                        >
                            Save
                        </Button>
                    </Group>
                </Stack>
            </Tabs.Panel>
        </Tabs>
    </ModalWrapper>
}

export type ProjectJobEditModalProps = {
    projectId:  Uuid
    job:        ProjectJob,

    opened: boolean;
    close: () => void;
}
