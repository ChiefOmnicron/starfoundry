import axios from "axios";
import { jwtDecode } from 'jwt-decode';

let jwtToken: string | null = null;

export const axiosClient = async () => {
    if (!jwtToken) {
        await refreshToken();
    }

    const decodedPayload = jwtDecode(jwtToken as string);
    if (decodedPayload.exp && Date.now() < decodedPayload.exp) {
        await refreshToken();
    }

    return axios
        .create({
            headers: {
                'Authorization': jwtToken,
            }
        })
}

export const refreshToken = async () => {
    return await axios
        .get('/api/auth/refresh-token')
        .then(x => {
            jwtToken = x.data;
        })
        .catch(e => console.error(e));
}
