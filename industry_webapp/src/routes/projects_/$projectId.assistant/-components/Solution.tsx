import { Accordion, Alert, Button, Grid, Group, Stack, Text, Textarea, Title } from '@mantine/core';
import { BlueprintOverwriteList } from '@starfoundry/components/projectGroup/BlueprintOverwriteList';
import { CopyText } from '@starfoundry/components/misc/CopyText';
import { createColumnHelper, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { EveIcon } from '@starfoundry/components/misc/EveIcon';
import { generateSolution, type GenerateSolutionResponse, type SolutionManufacturing, type SolutionMaterial } from '@starfoundry/components/services/projects/generateSolution';
import { IndustryHubList } from '@starfoundry/components/list/IndustryHubList';
import { InitializeProject } from '@starfoundry/components/services/projects/initialize';
import { InternalLink } from '@starfoundry/components/links/InternalLink';
import { ItemList } from '@starfoundry/components/list/ItemList';
import { JobSplittingRunList } from '@starfoundry/components/projectGroup/JobSplittingRunList';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { Route as ProjectGroupDefaultsRoute } from '@/routes/project-groups_/$projectGroupId.defaults';
import { Route as ProjectView } from '@/routes/projects_/$projectId.overview';
import { TableWrapper } from '@starfoundry/components/wrapper/Table';
import { useEffect, useState } from 'react';
import { useListProjectGroupDefaultBlacklist } from '@starfoundry/components/services/project-group/listDefaultBlacklist';
import { useListProjectGroupDefaultBlueprintOverwrites, type BlueprintOverwrite } from '@starfoundry/components/services/project-group/listDefaultBlueprintOverwrites';
import { useListProjectGroupDefaultJobSplitting, type JobSplittingRun } from '@starfoundry/components/services/project-group/listDefaultJobSplitting';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useNavigate } from '@tanstack/react-router';
import type { IndustryHub } from '@starfoundry/components/services/industry-hub/list';
import type { Item } from '@starfoundry/components/services/item/model';
import { FETCH_PROJECT, type ProjectList } from '@starfoundry/components/services/projects/fetch';

const columnHelperMaterial = createColumnHelper<SolutionMaterial>();
const columnsMaterial = [
    columnHelperMaterial.display({
        id: 'icon',
        cell: props => <EveIcon
            id={props.row.original.item.type_id}
        />,
        size: 1,
        maxSize: 1,
    }),
    columnHelperMaterial.display({
        id: 'name',
        cell: props => <CopyText
            value={props.row.original.item.name}
        />,
        header: () => 'Name',
        size: 50,
    }),
    columnHelperMaterial.display({
        id: 'needed',
        cell: props => <>
            <CopyText
                value={props.row.original.needed}
                number
            />
        </>,
        header: () => 'Needed',
        size: 5,
    }),
    columnHelperMaterial.display({
        id: 'stock',
        cell: props => <CopyText
            value={props.row.original.stock}
            number
        />,
        header: () => 'Stock',
        size: 10,
    }),
    columnHelperMaterial.display({
        id: 'total',
        cell: props => <CopyText
            value={props.row.original.stock + props.row.original.needed}
            number
        />,
        header: () => 'Total',
        size: 10,
    }),
];

const columnHelperManufacturing = createColumnHelper<SolutionManufacturing>();
const columnsManufacturing = [
    columnHelperManufacturing.display({
        id: 'icon',
        cell: props => <EveIcon
            id={props.row.original.item.type_id}
        />,
        size: 1,
        maxSize: 1,
    }),
    columnHelperManufacturing.display({
        id: 'name',
        cell: props => <CopyText
            value={props.row.original.item.name}
        />,
        header: () => 'Name',
        size: 30,
    }),
    columnHelperManufacturing.display({
        id: 'runs',
        cell: props => {
            const countRuns: { [key: number]: number } = {};

            props
                .row
                .original
                .runs
                .forEach(x => {
                    if (countRuns[x]) {
                        countRuns[x] += 1;
                    } else {
                        countRuns[x] = 1;
                    }
                });

            const entries = Object
                .keys(countRuns)
                .map((x: any) => <>{countRuns[x] }x {x} runs<br /></>);

            return <>
                { entries }
            </>
        },
        header: () => 'Runs',
        size: 5,
    }),
    columnHelperManufacturing.display({
        id: 'build_tax',
        cell: props => <CopyText
            value={props.row.original.build_tax}
            number
        />,
        header: () => 'Taxes',
        size: 10,
    }),
    columnHelperManufacturing.display({
        id: 'structure',
        cell: props => <CopyText
            value={props.row.original.structure?.name}
        />,
        header: () => 'Structure',
    }),
];

export function Solution({
    project,
}: SolutionProps) {
    const navigation = useNavigate();
    const queryClient = useQueryClient();

    const [selectedBlacklist, setSelectedBlacklist] = useState<Item[]>([]);
    const [selectedBlueprintOverwrite, setSelectedBlueprintOverwrite] = useState<BlueprintOverwrite[]>([]);
    const [selectedJobSplittingRun, setSelectedJobSplittingRun] = useState<JobSplittingRun[]>([]);

    const [showError, setShowError] = useState<boolean>(false);

    const [products, setProducts] = useState<string>('');
    const [stocks, setStocks] = useState<string>('');

    const [generatedSolutions, setGeneratedSolutions] = useState<GenerateSolutionResponse[]>([]);
    const [selectedSolution, setSelectedSolution] = useState<GenerateSolutionResponse | undefined>(undefined);

    // tmp
    const generateSolutionMutation = useMutation({
        mutationFn: async () => {
            return await generateSolution({
                project_group_id: project.project_group.id,
                products_str: products,
                stocks_str: stocks,

                blacklist:              selectedBlacklist
                                            .map(x => x.type_id),
                blueprint_overwrite:    selectedBlueprintOverwrite
                                            .map(x => {
                                                return {
                                                    material_efficiency: x.material_efficiency,
                                                    type_id:             x.item.type_id,
                                                }
                                            }),
                job_splitting:          selectedJobSplittingRun
                                            .map(x => {
                                                return {
                                                    runs:       x.max_runs,
                                                    type_id:    x.item.type_id,
                                                }
                                            }),
            })
        },
        onSuccess: (data: GenerateSolutionResponse[]) => {
            setShowError(false);
            setGeneratedSolutions(data);
            setSelectedSolution(data[0]);
        },
        onError: () => {
            setShowError(true);
        },
    });

    const initializeProjectMutation = useMutation({
        mutationFn: async () => {
            if (!selectedSolution) {
                return;
            }

            return await InitializeProject(
                project.id,
                {
                    solution_id: selectedSolution.solution_id,
                }
            );
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: [FETCH_PROJECT] });
            navigation({
                to: ProjectView.to,
                params: {
                    projectId: project.id,
                },
            });
        },
        onError: () => {
            setShowError(true);
        },
    });

    const {
        isPending: isPendingBlacklist,
        isError: isErrorBlacklist,
        data: projectGroupBlacklist,
    } = useListProjectGroupDefaultBlacklist(project.project_group.id);

    const {
        isPending: isPendingBlueprintOverwrite,
        isError: isErrorBlueprintOverwrite,
        data: projectGroupBlueprintOverwrite,
    } = useListProjectGroupDefaultBlueprintOverwrites(project.project_group.id);

    const {
        isPending: isPendingJobSplitting,
        isError: isErrorJobSplitting,
        data: projectGroupDefaultJobSplittings,
    } = useListProjectGroupDefaultJobSplitting(project.project_group.id);

    useEffect(() => {
        if (isPendingBlacklist || isErrorBlacklist) {
            return;
        }
        if (isPendingBlueprintOverwrite || isErrorBlueprintOverwrite) {
            return;
        }
        if (isPendingJobSplitting || isErrorJobSplitting) {
            return;
        }

        setSelectedBlacklist(projectGroupBlacklist);
        setSelectedBlueprintOverwrite(projectGroupBlueprintOverwrite);
        setSelectedJobSplittingRun(projectGroupDefaultJobSplittings.runs);
    }, [projectGroupBlacklist, selectedBlueprintOverwrite, projectGroupDefaultJobSplittings]);

    const tableMaterials = useReactTable<SolutionMaterial>({
        columns: columnsMaterial,
        data: selectedSolution ? selectedSolution.material : [],
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });
    const tableManufacturing = useReactTable<SolutionManufacturing>({
        columns: columnsManufacturing,
        data: selectedSolution ? selectedSolution.manufacturing : [],
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    if (isPendingBlacklist) {
        return LoadingAnimation();
    }
    if (isErrorBlacklist) {
        return LoadingError();
    }

    const productsSelect = () => {
        return <>
            <Stack>
                <Title order={2}>Products & Stock</Title>

                <Grid>
                    <Grid.Col span={6}>
                        <Textarea
                            label="Products to build"
                            description="List of products that should be build. Format: Product Amount ME"
                            placeholder="Ragnarok 1 8"
                            onChange={(event) => setProducts(event.currentTarget.value)}
                            value={products}
                            minRows={10}
                            maxRows={10}
                            autosize
                        />
                    </Grid.Col>

                    <Grid.Col span={6}>
                        <Textarea
                            label="Products to buy"
                            description="List of products that should be bought. Product Amount"
                            placeholder="Mobile Depot 1"
                            minRows={10}
                            maxRows={10}
                            autosize
                        />
                    </Grid.Col>
                </Grid>

                <Textarea
                    label="Stock"
                    description="Already existing materials"
                    placeholder="Mobile Depot 1"
                    onChange={(event) => setStocks(event.currentTarget.value)}
                    value={stocks}
                    minRows={10}
                    maxRows={10}
                    autosize
                />

                <Group
                    justify="flex-end"
                >
                    <Button
                        data-cy="create"
                        mt="sm"
                        type="submit"
                        loading={generateSolutionMutation.isPending}
                        disabled={generateSolutionMutation.isPending}
                        onClick={() => {
                            setGeneratedSolutions([]);
                            setSelectedSolution(undefined);
                            generateSolutionMutation.mutate()
                        }}
                    >
                        Generate solution
                    </Button>
                </Group>

                {
                    generateSolutionMutation.isPending
                    ?   <>{ LoadingAnimation() }</>
                    :   <></>
                }
            </Stack>
        </>
    }

    const material = () => {
        if (generatedSolutions.length === 0) {
            return <></>
        }

        return <>
            <TableWrapper
                scrollable
                table={tableMaterials}
            />
        </>;
    }

    const manufacturing = () => {
        if (generatedSolutions.length === 0) {
            return <></>
        }

        return <>
            <TableWrapper
                scrollable
                table={tableManufacturing}
            />
        </>;
    }

    const showSolution = () => {
        if (generatedSolutions.length === 0) {
            return <></>;
        }

        return <>
            <Title order={2}>Generated Solutions</Title>

            <IndustryHubList
                industryHubs={generatedSolutions.map(x => x.industry_hub)}

                industryHubCardProps={{
                    checkable: true,
                    allowUncheck: false,
                    checked: selectedSolution ? [selectedSolution.industry_hub] : [],
                    onChange: (_: 'checked' | 'unchecked', industryHub: IndustryHub) => {
                        setSelectedSolution(
                            generatedSolutions
                                .find(x => x.industry_hub.id === industryHub.id)
                        );
                    }
                }}
            />

            <Accordion chevronPosition="right" variant="contained">
                <Accordion.Item value='materials'>
                    <Accordion.Control>
                        <Text>Materials</Text>

                        <Text size="sm" c="dimmed" fw={400}>
                            List of materials that are required to build the products
                        </Text>
                    </Accordion.Control>

                    <Accordion.Panel>
                        { material() }
                    </Accordion.Panel>
                </Accordion.Item>
                <Accordion.Item value='manufacturing'>
                    <Accordion.Control>
                        <Text>Industry Jobs</Text>

                        <Text size="sm" c="dimmed" fw={400}>
                            Industry jobs that need to be started
                        </Text>
                    </Accordion.Control>

                    <Accordion.Panel>
                        { manufacturing() }
                    </Accordion.Panel>
                </Accordion.Item>
            </Accordion>
        </>
    }

    const initializeProjectView = () => {
        if (generatedSolutions.length === 0) {
            return <></>
        }

        return <>
            <Group
                justify='flex-end'
            >
                <Button
                    disabled={initializeProjectMutation.isPending}
                    loading={initializeProjectMutation.isPending}
                    onClick={() => initializeProjectMutation.mutate()}
                >
                    Select Solution
                </Button>
            </Group>
        </>
    }

    return <>
        {
            showError
            ?   LoadingError()
            :   <></>
        }

        <Stack>
            <Title order={2}>Project Group Defaults</Title>

            <Alert variant='light' color='gray'>
                Any changes made will only be applied to this project.
                For permanent changes head over to <InternalLink
                    content='Project Group Defaults'
                    to={ProjectGroupDefaultsRoute.to}
                    params={{
                        projectGroupId: project.project_group.id,
                    }}
                    target='_blank'
                />
            </Alert>

            <Accordion chevronPosition="right" variant="contained">
                <Accordion.Item value='blacklist'>
                    <Accordion.Control>
                        <Text>Blacklist</Text>

                        <Text size="sm" c="dimmed" fw={400}>
                            Configure items that should not be build.
                        </Text>
                    </Accordion.Control>

                    <Accordion.Panel>
                        <ItemList
                            selected={selectedBlacklist}
                            onSelect={setSelectedBlacklist}
                            buildable
                            editable
                        />
                    </Accordion.Panel>
                </Accordion.Item>
                <Accordion.Item value='blueprint_overwrite'>
                    <Accordion.Control>
                        <Text>Blueprint Overwrite</Text>

                        <Text size="sm" c="dimmed" fw={400}>
                            Overwrite the default Material Efficiency for blueprints.
                        </Text>
                    </Accordion.Control>

                    <Accordion.Panel>
                        <BlueprintOverwriteList
                            selected={selectedBlueprintOverwrite}
                        />
                    </Accordion.Panel>
                </Accordion.Item>
                <Accordion.Item value='job_splitting'>
                    <Accordion.Control>
                        <Text>Job Splitting</Text>

                        <Text size="sm" c="dimmed" fw={400}>
                            Set the max runs for blueprints
                        </Text>
                    </Accordion.Control>

                    <Accordion.Panel>
                        <JobSplittingRunList
                            selected={selectedJobSplittingRun}
                        />
                    </Accordion.Panel>
                </Accordion.Item>
            </Accordion>
        </Stack>

        {productsSelect()}

        {showSolution()}

        {initializeProjectView()}
    </>
}

export type SolutionProps = {
    project: ProjectList;
}
