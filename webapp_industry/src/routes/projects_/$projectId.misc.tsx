import { createFileRoute } from '@tanstack/react-router'
import { LoadingAnimation } from '@/components/LoadingAnimation';
import { LoadingError } from '@/components/LoadingError';
import { ProjectMiscList } from './-components/MiscList'
import { useListProjectMisc, type ProjectMisc } from '@/services/projects/listMisc';
import { useEffect, useState } from 'react';
import { SaveDialog } from '@/components/SaveDialog';

export const Route = createFileRoute('/projects_/$projectId/misc')({
  component: RouteComponent,
})

function RouteComponent() {
    const { projectId } = Route.useParams();

    const [selectedProjectMiscOld, setSelectedProjectMiscOld] = useState<ProjectMisc[]>([]);
    const [selectedProjectMisc, setSelectedProjectMisc] = useState<ProjectMisc[]>([]);

    const {
        isError,
        isPending,
        data: projectMisc,
    } = useListProjectMisc(projectId);

    useEffect(() => {
        if (projectMisc) {
            setSelectedProjectMiscOld(projectMisc);
            setSelectedProjectMisc(projectMisc);
        }
    }, [projectMisc]);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return LoadingError();
    }

    const onSelect = (misc: ProjectMisc) => {
        console.log(misc)
        setSelectedProjectMisc([
            misc,
            ...selectedProjectMisc,
        ]);
    }

    const onDelete = (misc: ProjectMisc) => {
        const filtered = selectedProjectMisc
            .filter(x => x !== misc);
        setSelectedProjectMisc(filtered);
    }

    const resetChanges = () => {
        setSelectedProjectMisc(selectedProjectMiscOld);
    }

    const saveChanges = () => {

    }

    return <>
        <ProjectMiscList
            selected={selectedProjectMisc}
            onSelect={onSelect}
            onDelete={onDelete}
            editable
        />

        <SaveDialog
            onReset={resetChanges}
            onSave={saveChanges}
            show={ selectedProjectMisc !== selectedProjectMiscOld }
        />
    </>
}
