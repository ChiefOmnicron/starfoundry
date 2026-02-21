import { Accordion, Alert, Button, Stack, Table, Text, Title } from '@mantine/core';
import { createColumnHelper, flexRender, getCoreRowModel, useReactTable } from '@tanstack/react-table';
import { useMutation } from '@tanstack/react-query';
import { useEffect, useState } from 'react';
import { Route as ProjectGroupDefaultsRoute } from '@/routes/project-groups_/$projectGroupId.defaults';
import { BlueprintOverwriteList } from '@/routes/project-groups_/-components/BlueprintOverwriteLIst';
import { JobSplittingRunList } from '@/routes/project-groups_/-components/JobSplittingRunList';
import { generateSolution, type GenerateSolutionResponse, type SolutionManufacturing, type SolutionMaterial } from '@starfoundry/components/services/projects/generateSolution';
import { EveIcon } from '@starfoundry/components/misc/EveIcon';
import { CopyText } from '@starfoundry/components/misc/CopyText';
import type { Item } from '@starfoundry/components/services/item/model';
import { useListProjectGroupDefaultBlueprintOverwrites, type BlueprintOverwrite } from '@starfoundry/components/services/project-group/listDefaultBlueprintOverwrites';
import { useListProjectGroupDefaultJobSplitting, type JobSplittingRun } from '@starfoundry/components/services/project-group/listDefaultJobSplitting';
import { useListProjectGroupDefaultBlacklist } from '@starfoundry/components/services/project-group/listDefaultBlacklist';
import { LoadingAnimation } from '@starfoundry/components/misc/LoadingAnimation';
import { LoadingError } from '@starfoundry/components/misc/LoadingError';
import { ItemList } from '@starfoundry/components/list/ItemList';
import { InternalLink } from '@starfoundry/components/links/InternalLink';
import type { Uuid } from '@starfoundry/components/services/utils';

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
        size: 50,
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
    //nextStep,
    //prevStep,

    projectGroupId,
}: SolutionProps) {
    const [solutionMaterial, setSolutionMaterial] = useState<SolutionMaterial[]>([]);
    const [solutionManufacturing, setSolutionManufacturing] = useState<SolutionManufacturing[]>([]);

    const [selectedBlacklist, setSelectedBlacklist] = useState<Item[]>([]);
    const [selectedBlueprintOverwrite, setSelectedBlueprintOverwrite] = useState<BlueprintOverwrite[]>([]);
    const [selectedJobSplittingRun, setSelectedJobSplittingRun] = useState<JobSplittingRun[]>([]);

    // tmp
    const generateSolutionMutation = useMutation({
        mutationFn: async () => {
            return await generateSolution(projectGroupId)
        },
        onSuccess: (data: GenerateSolutionResponse) => {
            setSolutionMaterial(data.material);
            setSolutionManufacturing(data.manufacturing);
        }
    });

    const {
        isPending: isPendingBlacklist,
        isError: isErrorBlacklist,
        data: projectGroupBlacklist,
    } = useListProjectGroupDefaultBlacklist(projectGroupId);

    const {
        isPending: isPendingBlueprintOverwrite,
        isError: isErrorBlueprintOverwrite,
        data: projectGroupBlueprintOverwrite,
    } = useListProjectGroupDefaultBlueprintOverwrites(projectGroupId);

    const {
        isPending: isPendingJobSplitting,
        isError: isErrorJobSplitting,
        data: projectGroupDefaultJobSplittings,
    } = useListProjectGroupDefaultJobSplitting(projectGroupId);

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
        data: solutionMaterial.sort((a, b) => a.item.name.localeCompare(b.item.name)),
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });
    const tableManufacturing = useReactTable<SolutionManufacturing>({
        columns: columnsManufacturing,
        data: solutionManufacturing.sort((a, b) => a.item.name.localeCompare(b.item.name)),
        autoResetPageIndex: false,
        getCoreRowModel: getCoreRowModel(),
    });

    if (isPendingBlacklist) {
        return LoadingAnimation();
    }
    if (isErrorBlacklist) {
        return LoadingError();
    }

    const material = () => {
        if (solutionMaterial.length === 0) {
            return <></>
        }

        return <>
            <Title order={2}>Material</Title>

            <Table.ScrollContainer minWidth={100} maxHeight={300}>
                <Table stickyHeader striped data-cy="data">
                    <Table.Thead>
                    {tableMaterials.getHeaderGroups().map(headerGroup => (
                        <Table.Tr key={headerGroup.id}>
                            {headerGroup.headers.map(header => (
                                <Table.Th
                                    key={header.id}
                                    style={{
                                        width: `${header.getSize()}%`
                                    }}
                                >
                                    {flexRender(
                                        header.column.columnDef.header,
                                        header.getContext()
                                    )}
                                </Table.Th>
                            ))}
                        </Table.Tr>
                    ))}
                    </Table.Thead>

                    <Table.Tbody>
                        {tableMaterials.getRowModel().rows.map(row => (
                            <Table.Tr key={row.id}>
                                {
                                    row.getVisibleCells().map(cell => (
                                        <Table.Td key={cell.id}>
                                            {
                                                flexRender(
                                                    cell.column.columnDef.cell,
                                                    cell.getContext()
                                                )
                                            }
                                        </Table.Td>
                                    ))
                                }
                            </Table.Tr>
                        ))}
                    </Table.Tbody>
                </Table>
            </Table.ScrollContainer>
        </>
    }

    const manufacturing = () => {
        if (solutionManufacturing.length === 0) {
            return <></>
        }

        return <>
            <Title order={2}>Manufacturing</Title>

            <Table.ScrollContainer minWidth={100} maxHeight={300}>
                <Table stickyHeader striped data-cy="data">
                    <Table.Thead>
                    {tableManufacturing.getHeaderGroups().map(headerGroup => (
                        <Table.Tr key={headerGroup.id}>
                            {headerGroup.headers.map(header => (
                                <Table.Th
                                    key={header.id}
                                    style={{
                                        width: `${header.getSize()}%`
                                    }}
                                >
                                    {flexRender(
                                        header.column.columnDef.header,
                                        header.getContext()
                                    )}
                                </Table.Th>
                            ))}
                        </Table.Tr>
                    ))}
                    </Table.Thead>

                    <Table.Tbody>
                        {tableManufacturing.getRowModel().rows.map(row => (
                            <Table.Tr key={row.id}>
                                {
                                    row.getVisibleCells().map(cell => (
                                        <Table.Td key={cell.id}>
                                            {
                                                flexRender(
                                                    cell.column.columnDef.cell,
                                                    cell.getContext()
                                                )
                                            }
                                        </Table.Td>
                                    ))
                                }
                            </Table.Tr>
                        ))}
                    </Table.Tbody>
                </Table>
            </Table.ScrollContainer>
        </>
    }

    const estimatedPrice = () => {
        if (solutionManufacturing.length === 0) {
            return <></>
        }

        return <>
            <Title order={2}>Estimated cost</Title>

            <Table variant="vertical" layout="fixed">
                <Table.Tbody>
                    <Table.Tr>
                        <Table.Th w={200}>Manufacturing</Table.Th>
                        <Table.Td><CopyText value={solutionManufacturing.map(x => x.build_tax).reduce((prev, curr) => prev += curr, 0)} number /></Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th w={200}>Market</Table.Th>
                        <Table.Td><CopyText value={0} number /></Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th w={200}>Hauling</Table.Th>
                        <Table.Td><CopyText value={0} number /></Table.Td>
                    </Table.Tr>
                    <Table.Tr>
                        <Table.Th w={200}>Total</Table.Th>
                        <Table.Td><CopyText value={0} number /></Table.Td>
                    </Table.Tr>
                </Table.Tbody>
            </Table>
        </>
    }

    return <>
        <Stack>
            <Title order={3}>Project Group Defaults</Title>

            <Accordion chevronPosition="right" variant="contained">
                <Accordion.Item value='blacklist'>
                    <Accordion.Control>
                        <Text>Blacklist</Text>

                        <Text size="sm" c="dimmed" fw={400}>
                            Configure items that should not be build.
                        </Text>
                    </Accordion.Control>

                    <Accordion.Panel>
                        <Alert variant='light' color='gray'>
                            Any changes made will only be applied to this project.
                            For permanent changes head over to <InternalLink
                                content='Project Group Defaults'
                                to={ProjectGroupDefaultsRoute.to}
                                params={{
                                    projectGroupId,
                                }}
                                target='_blank'
                            />
                        </Alert>
                        <ItemList
                            selected={selectedBlacklist}
                            onSelect={(tmpBlacklist) => {
                                setSelectedBlacklist(tmpBlacklist);
                            }}
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
                        <Alert variant='light' color='gray'>
                            Any changes made will only be applied to this project.
                            For permanent changes head over to <InternalLink
                                content='Project Group Defaults'
                                to={ProjectGroupDefaultsRoute.to}
                                params={{
                                    projectGroupId,
                                }}
                                target='_blank'
                            />
                        </Alert>
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
                        <Alert variant='light' color='gray'>
                            Any changes made will only be applied to this project.
                            For permanent changes head over to <InternalLink
                                content='Project Group Defaults'
                                to={ProjectGroupDefaultsRoute.to}
                                params={{
                                    projectGroupId,
                                }}
                                target='_blank'
                            />
                        </Alert>
                        <JobSplittingRunList
                            selected={selectedJobSplittingRun}
                        />
                    </Accordion.Panel>
                </Accordion.Item>
            </Accordion>
        </Stack>

        { material() }

        { manufacturing() }

        { estimatedPrice() }

        <Button
            data-cy="create"
            mt="sm"
            type="submit"
            onClick={() => generateSolutionMutation.mutate()}
        >
            Click mich du sau
        </Button>
    </>
}

export type SolutionProps = {
    nextStep: () => void;
    prevStep: () => void;

    projectGroupId: Uuid;
}
