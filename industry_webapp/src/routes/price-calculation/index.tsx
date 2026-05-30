import { Accordion, Button, Grid, Group, Stack, Table, Text, Textarea, Title } from '@mantine/core';
import { CopyText } from '@starfoundry/components/misc';
import { createColumnHelper, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { createFileRoute } from '@tanstack/react-router';
import { EveIcon } from '@starfoundry/components/misc/EveIcon';
import { generateSolution, type GenerateSolutionResponse, type SolutionManufacturing, type SolutionMaterial } from '@starfoundry/components/services/projects/generateSolution';
import { IndustryHubList } from '@starfoundry/components/list/IndustryHubList';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { ProjectGroupSelector } from '@starfoundry/components/selectors/ProjectGroupSelector';
import { TableWrapper } from '@starfoundry/components/wrapper/Table';
import { TempProjectGroupConfiguration } from '@starfoundry/components/projectGroup/TempConfiguration';
import { useEffect, useState } from 'react';
import { useListProjectGroup, type ProjectGroupMinimal } from '@starfoundry/components/services/project-group/list';
import { useListProjectGroupDefaultBlacklist } from '@starfoundry/components/services/project-group/listDefaultBlacklist';
import { useListProjectGroupDefaultBlueprintOverwrites, type BlueprintOverwrite } from '@starfoundry/components/services/project-group/listDefaultBlueprintOverwrites';
import { useListProjectGroupDefaultJobSplitting, type JobSplittingRun } from '@starfoundry/components/services/project-group/listDefaultJobSplitting';
import { useListProjectGroupDefaultMarkets } from '@starfoundry/components/services/project-group/listDefaultMarket';
import { useMutation } from '@tanstack/react-query';
import type { IndustryHub } from '@starfoundry/components/services/industry-hub/list';
import type { Item } from '@starfoundry/components/services/item/model';
import type { Structure } from '@starfoundry/components/services/structure/list';

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
        size: 40,
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
        size: 10,
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
    columnHelperMaterial.display({
        id: 'price',
        cell: props => <CopyText
            value={props.row.original.needed * (props.row.original.price || 0)}
            number
        />,
        header: () => 'Price (Needed)',
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

export const Route = createFileRoute('/price-calculation/')({
    component: RouteComponent,})

function RouteComponent() {
    const [selectedProjectGroup, setSelectedProjectGroup] = useState<ProjectGroupMinimal | null>(null);
    const [_showError, setShowError] = useState<boolean>(false);

    const [selectedBlacklist, setSelectedBlacklist] = useState<Item[]>([]);
    const [selectedBlueprintOverwrite, setSelectedBlueprintOverwrite] = useState<BlueprintOverwrite[]>([]);
    const [selectedJobSplittingRun, setSelectedJobSplittingRun] = useState<JobSplittingRun[]>([]);
    const [selectedMarkets, setSelectedMarkets] = useState<Structure[]>([]);

    const [products, setProducts] = useState<string>('');
    const [additionalProducts, setAdditionalProducts] = useState<string>('');
    const [stocks, setStocks] = useState<string>('');

    const [generatedSolutions, setGeneratedSolutions] = useState<GenerateSolutionResponse[]>([]);
    const [selectedSolution, setSelectedSolution] = useState<GenerateSolutionResponse | undefined>(undefined);

    const {
        data: projectGroups,
    } = useListProjectGroup({
        archived: false,
    });

    const {
        isPending: isPendingBlacklist,
        isError: isErrorBlacklist,
        data: projectGroupBlacklist,
    } = useListProjectGroupDefaultBlacklist((selectedProjectGroup || { id: '' }).id, {
        enabled: !!selectedProjectGroup
    });

    const {
        isPending: isPendingBlueprintOverwrite,
        isError: isErrorBlueprintOverwrite,
        data: projectGroupBlueprintOverwrite,
    } = useListProjectGroupDefaultBlueprintOverwrites((selectedProjectGroup || { id: '' }).id, {
        enabled: !!selectedProjectGroup
    });

    const {
        isPending: isPendingJobSplitting,
        isError: isErrorJobSplitting,
        data: projectGroupDefaultJobSplittings,
    } = useListProjectGroupDefaultJobSplitting((selectedProjectGroup || { id: '' }).id, {
        enabled: !!selectedProjectGroup
    });

    const {
        isPending: isPendingMarket,
        isError: isErrorMarket,
        data: projectGroupDefaultMarkets,
    } = useListProjectGroupDefaultMarkets((selectedProjectGroup || { id: '' }).id, {
        enabled: !!selectedProjectGroup
    });

    const generateSolutionMutation = useMutation({
        mutationFn: async () => {
            return await generateSolution({
                project_group_id: (selectedProjectGroup || { id: '' }).id,
                products_str: products,
                additional_products_str: additionalProducts,
                stocks_str: stocks,
                calculate_market_cost: true,

                blacklist:              (selectedBlacklist || [])
                                            .map(x => x.type_id),
                blueprint_overwrite:    (selectedBlueprintOverwrite || [])
                                            .map(x => {
                                                return {
                                                    material_efficiency: x.material_efficiency,
                                                    type_id:             x.item.type_id,
                                                }
                                            }),
                job_splitting:          (selectedJobSplittingRun || [])
                                            .map(x => {
                                                return {
                                                    runs:       x.max_runs,
                                                    type_id:    x.item.type_id,
                                                }
                                            }),
                markets:                (selectedMarkets || [])
                                            .map(x => x.structure_id),
            })
        },
        onSuccess: (data: GenerateSolutionResponse[]) => {
            setShowError(false);
            setGeneratedSolutions(data);
            setSelectedSolution(data[0]);
        },
        onError: (e) => {
            console.error(e);
            setShowError(true);
        },
    });

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
        if (isPendingMarket || isErrorMarket) {
            return;
        }

        setSelectedBlacklist(projectGroupBlacklist);
        setSelectedBlueprintOverwrite(projectGroupBlueprintOverwrite);
        setSelectedJobSplittingRun(projectGroupDefaultJobSplittings.runs);
        setSelectedMarkets(projectGroupDefaultMarkets)
    }, [projectGroupBlacklist, projectGroupDefaultMarkets, projectGroupDefaultJobSplittings, projectGroupDefaultMarkets]);

    const showProjectGroupConfiguration = () => {
        if (!!!selectedProjectGroup) {
            return <></>
        }

        return <TempProjectGroupConfiguration
            showBlacklist
            blacklist={selectedBlacklist}
            onBlacklistSelect={() => {}}

            showBlueprintOverwrites
            blueprintOverwrites={selectedBlueprintOverwrite}
            onBlueprintOverwriteSelect={() => {}}

            showJobSplitting
            jobSplitting={selectedJobSplittingRun}
            onJobSplittingSelect={() => {}}

            showMarket
            markets={selectedMarkets}
        />
    }

    const showProductConfiguration = () => {
        if (!!!selectedProjectGroup) {
            return <></>
        }

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
                            onChange={(event) => setAdditionalProducts(event.currentTarget.value)}
                            value={additionalProducts}
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

    const showSolution = () => {
        if (generatedSolutions.length === 0) {
            return <></>;
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

        const manufacturingCost = () =>  {
            if (!selectedSolution) {
                return 0;
            }

            return selectedSolution
                .manufacturing
                .map(x => x.build_tax)
                .reduce((prev, curr) => prev += curr, 0);
        }

        const marketCost = () => {
            if (!selectedSolution) {
                return 0;
            }

            return selectedSolution
                .material
                .map(x => (x.price || 0) * x.needed)
                .reduce((prev, curr) => prev += curr, 0);
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

            <Stack>
                <Text>Market: { Math.ceil(marketCost()) }</Text>
                <Text>Taxes: { Math.ceil(manufacturingCost()) }</Text>
                <Text>Total: { Math.ceil(marketCost() + manufacturingCost()) }</Text>
                <Text>Total (7.5%): { Math.ceil((marketCost() + manufacturingCost()) * 1.075) }</Text>
                <Text>Total (10%): { Math.ceil((marketCost() + manufacturingCost()) * 1.1) }</Text>
            </Stack>

            <Table variant="vertical" layout="fixed" withTableBorder>
                <Table.Tbody>
                    <Table.Tr>
                        <Table.Th w={200}>Materials</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={ Math.ceil(marketCost()) }
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Manufacturing taxes</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={ Math.ceil(manufacturingCost()) }
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Total</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={ Math.ceil(marketCost() + manufacturingCost()) }
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Total (5%)</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={ Math.ceil((marketCost() + manufacturingCost()) * 1.05) }
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Total (7.5%)</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={ Math.ceil((marketCost() + manufacturingCost()) * 1.075) }
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th>Total (10%)</Table.Th>
                        <Table.Td>
                            <CopyText
                                value={ Math.ceil((marketCost() + manufacturingCost()) * 1.1) }
                                number
                                withUnit
                            />
                        </Table.Td>
                    </Table.Tr>
                </Table.Tbody>
            </Table>
        </>
    }

    return <>
        <Stack>
            <ProjectGroupSelector
                projectGroups={projectGroups}
                onSelect={setSelectedProjectGroup}
            />

            {showProjectGroupConfiguration()}

            {showProductConfiguration()}

            {showSolution()}
        </Stack>
    </>
}
