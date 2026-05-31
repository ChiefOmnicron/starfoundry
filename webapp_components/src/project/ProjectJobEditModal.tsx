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
import { EveIcon } from "@internal/misc/EveIcon";

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

    onJobSplit,
}: ProjectJobEditModalProps) {
    const [hasError, setHasError] = useState<boolean>(false);
    const [updateSuccess, setUpdateSuccess] = useState<boolean>(false);

    const [cost, setCost] = useState<number | undefined>(undefined);
    const [jobId, setJobId] = useState<number | undefined>(undefined);
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
                if (!jobs) {
                    return;
                }

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
        onSuccess: (_data, _vars, _result, context) => {
            setHasError(false);
            context.client.invalidateQueries({
                queryKey: [LIST_PROJECT_JOBS],
            });
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
        setCost(job.cost || undefined);
        setJobId(job.job_id || undefined);
        setStatus(job.status);

        setCurrentRuns(job.runs);
        setSplitRuns(0);
    }, [job]);

    const updateJob = () => {
        saveJobMutation.mutate({
            cost:       cost ? cost : undefined,
            status:     status,
            job_id:     jobId ? jobId : undefined,
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
        const excess = checkedJobChange
                .excess
                .map(x => {
                    return {
                        quantity: x.quantity,
                        type_id:  x.item.type_id,
                    }
                });
        const jobs = checkedJobChange
                .jobs
                .map(x => {
                    return {
                        runs:           x.runs,
                        type_id:        x.item.type_id,
                        structure_id:   x.structure_id,
                    }
                });
        const market = checkedJobChange
                .materials
                .map(x => {
                    return {
                        quantity: x.quantity,
                        type_id:  x.item.type_id,
                    }
                });

        // adds new jobs that are required for the final product
        const addSubJobPromise = addJobMutation.mutateAsync(jobs);
        const addExcessPromise = addExcessMutation.mutateAsync(excess);
        const addMarketPromise = addMarketMutation.mutateAsync(market);

        // add the split runs
        const addJobPromise = addJobMutation.mutateAsync([{
            runs:           splitRuns,
            structure_id:   job.structure.id,
            type_id:        job.item.type_id
        }]);
        // update the amount of runs
        const saveJobPromise = saveJobMutation.mutateAsync({
            status: "WAITING_FOR_MATERIALS",
            runs: job.runs - splitRuns,
        });

        Promise.all([
                addSubJobPromise,
                addExcessPromise,
                addMarketPromise,
                addJobPromise,
                saveJobPromise,
            ])
            .then(_ => {
                onJobSplit();
                reset();
                close();
            });
    }

    const showJobChange = () => {
        if (checkedJobChange.jobs.length === 0 && checkedJobChange.materials.length === 0) {
            if (hasChecked) {
                return <>
                    <Alert>No additional materials/runs are needed</Alert>
                </>;
            }
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

    const reset = () => {
        setHasError(false);
        setUpdateSuccess(false);

        setCost(undefined);
        setJobId(undefined);
        setStatus('WAITING_FOR_MATERIALS');

        setCurrentRuns(0);
        setSplitRuns(0);
        setHasChecked(false);

        setCheckedJobChange({
            excess: [],
            jobs: [],
            materials: [],
        });
    }

    const showItemName = () => {
        if (!job || !job.item) {
            return <></>;
        }

        return <>
            <Group>
                <EveIcon
                    id={job.item.type_id}
                    category="types"
                    height={32}
                    width={32}
                />

                <Text>
                    {job.item.name}
                </Text>

                <Text>
                    x{job.runs}
                </Text>
            </Group>
        </>;
    }

    return <ModalWrapper
        close={() => {
            reset();
            close();
        }}
        opened={opened}
        title="Edit job"
        size="50%"
    >
        <Stack>
            {showError()}
            {showUpdateSuccess()}

            {showItemName()}

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
                        <Text size="sm" fw={500} mt={10}>
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
                                        if (hasChecked) {
                                            setHasChecked(false);
                                            setCheckedJobChange({
                                                excess: [],
                                                jobs: [],
                                                materials: [],
                                            });
                                        }

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
        </Stack>
    </ModalWrapper>
}

export type ProjectJobEditModalProps = {
    projectId:  Uuid
    job:        ProjectJob,

    opened: boolean;
    close: () => void;

    onJobSplit: () => void;
}
