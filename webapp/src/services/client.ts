import axios from "axios";
import { jwtDecode } from 'jwt-decode';

let jwtToken: string | null = null;

export const axiosClient = async () => {
    if (!jwtToken) {
        try {
            await refreshToken();
        }
        catch(_) {
            throw 'invalid'
        }
    }

    const decodedPayload = jwtDecode(jwtToken as string);
    if (decodedPayload.exp && Date.now() < decodedPayload.exp) {
        try {
            await refreshToken();
        }
        catch(_) {
            throw 'invalid'
        }
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
        .get('/api/auth/token')
        .then(x => {
            jwtToken = x.data.access_token;
        })
        .catch(e => console.error(e));
}
