import { InternalLink } from "../links/InternalLink";
import { LoadingAnimation } from "../misc/LoadingAnimation";
import { useFetchProject } from "../services/projects/fetch";
import type { Uuid } from "../services/utils"

export function ProjectSubProjectList({
    projects,
    projectLink,
}: ProjectSubProjectListProps) {
    return projects
        .map(x => ProjectSubProjectLink({
            projectId: x,
            projectLink: projectLink,
        }))
}

function ProjectSubProjectLink({
    projectId,
    projectLink,
}: ProjectSubProjectLinkProps) {
    const {
        isPending,
        isError,
        data,
    } = useFetchProject(projectId);

    if (isPending) {
        return LoadingAnimation();
    }

    if (isError) {
        return <></>;
    }

    return <InternalLink
        to={projectLink}
        content={data.name}
        params={{
            projectId,
        }}
        target="_blank"
    />
}

export type ProjectSubProjectListProps = {
    projects:       Uuid[];

    projectLink:    any;
}

export type ProjectSubProjectLinkProps = {
    projectId:      Uuid;

    projectLink:    any;
}
