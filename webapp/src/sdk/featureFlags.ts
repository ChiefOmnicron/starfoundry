import axios from "axios";

const FEATURE_FLAG_PATH = '/api/v1/feature-flags'

export class FeatureFlagService {
    public static fetch(): Promise<string[]> {
        return axios
            .get(FEATURE_FLAG_PATH)
            .then(x => x.data);
    }
}
