import { Button, Flex, InputBase, Stack, Text, Title, UnstyledButton } from "@mantine/core";
import { useEffect, useState, type ReactElement } from "react";
import { listProjects, type ProjectListMinimal } from "../services/projects/list";
import { ProjectList } from "../project/ProjectList";
import { ModalWrapper } from "../wrapper/Modal";
import { useDebouncedCallback } from "@mantine/hooks";
import { LoadingAnimation } from "../misc/LoadingAnimation";

export function ProjectSelectorModal({
    opened,
    onClose,
    onSelect,

    selected,
}: ProjectSelectorModalProp): ReactElement {
    // all projects selected by the user
    const [selectedProjects, setSelectedProjects] = useState<ProjectListMinimal[]>([]);

    const [loading, setLoading] = useState(false);
    const [_, setSearch] = useState('');
    const [searchResults, setSearchResults] = useState<ProjectListMinimal[]>([]);

    useEffect(() => {
        setSelectedProjects(selected);
    }, [opened, selected]);

    const projectList = (projects: ProjectListMinimal[]) => {
        if (projects.length === 0) {
            return <Text>No projects</Text>
        }

        if (loading && projects.length === 0) {
            return LoadingAnimation();
        }

        return <ProjectList
            projects={projects}
            groupByProjectGroup={false}

            projectCardProps={{
                checkable: true,
                checked: selectedProjects,
                onChange: (event: 'checked' | 'unchecked', project: ProjectListMinimal) => {
                    setSelectedProjects(
                        event === 'checked'
                            ? [...selectedProjects, project]
                            : selectedProjects.filter((y) => y.id !== project.id)
                    );
                }
            }}
        />
    }

    const handleSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
        setSearch(event.currentTarget.value);
        searchProjects(event.currentTarget.value);
    };

    const searchProjects = useDebouncedCallback(async (query: string) => {
        setLoading(true);

        listProjects({
                name: query,
                limit: 10,
            })
            .then((response) => {
                setSearchResults(response);
                setLoading(false);
            });
    }, 200);

    return <ModalWrapper
        opened={opened}
        close={onClose}
        title="Structures"
    >
        <Stack>
            <InputBase
                name="Name"
                description='Search for the name of a projects'
                placeholder="My cool project"
                onChange={handleSearch}
                loading={loading}
            ></InputBase>

            <Title order={2}>Found projects</Title>

            { projectList(searchResults) }

            <Title order={2}>Selected projects</Title>

            { projectList(selectedProjects) }

            <Flex
                justify='flex-end'
                gap='xs'
            >
                <UnstyledButton
                    onClick={onClose}
                >
                    Close without change
                </UnstyledButton>
                <Button
                    onClick={() => onSelect(selectedProjects)}
                >
                    Select ({ selectedProjects.length }) projects
                </Button>
            </Flex>
        </Stack>
    </ModalWrapper>
}

export type ProjectSelectorModalProp = {
    // modal control
    opened: boolean;
    onSelect: (entry: ProjectListMinimal[]) => void;
    onClose: () => void;

    // list of values that are already selected
    selected:   ProjectListMinimal[],
}
