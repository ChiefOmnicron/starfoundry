import type { JobId, ProjectUuid, StructureId, Uuid } from '@/sdk/utils';
import axios from 'axios';

const JOB_DETECTION_PATH = '/api/v1/job-detection';

export class JobDetectionService {
    public static updateJobAdd(
        jobId: JobId,
        update: IUpdateJobAdd,
    ): Promise<void> {
        return axios.put(`${JOB_DETECTION_PATH}/${jobId}/add`, update);
    }

    public static updateJobDelete(
        jobId: JobId,
        update: IUpdateJobDelete,
    ): Promise<void> {
        return axios.put(`${JOB_DETECTION_PATH}/${jobId}/delete`, update);
    }

    public static updateJobReplace(
        jobId: JobId,
        update: IUpdateJobReplace,
    ): Promise<void> {
        return axios.put(`${JOB_DETECTION_PATH}/${jobId}/replace`, update);
    }
}

export interface IUpdateJobAdd {
    delete_from_source: boolean;
    structure_id: StructureId;
    target_project_uuid: ProjectUuid;
}

export interface IUpdateJobDelete {
    delete_from_source: boolean;
    ignore: boolean;
}

export interface IUpdateJobReplace {
    delete_from_source: boolean;
    job_uuids: Uuid[];
    structure_id: StructureId;
    target_project_uuid: ProjectUuid;
}
