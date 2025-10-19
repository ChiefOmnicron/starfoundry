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
    if (decodedPayload.exp && (decodedPayload.exp * 1000) < Date.now()) {
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
                'authorization': jwtToken,
            }
        })
}

export const refreshToken = async () => {
    return await axios
        .get('/api/auth/token')
        .then(x => {
            jwtToken = x.data.access_token;
        })
        .catch(e => {
            throw e;
        });
}

export const isAdmin = async (): Promise<boolean> => {
    if (!jwtToken) {
        try {
            await refreshToken();
        }
        catch(_) {
            throw 'invalid'
        }
    }

    const decodedPayload = jwtDecode<JwtPayload>(jwtToken as string);
    return decodedPayload.is_admin;
}

export const characterInfo = (): CharacterInfo => {
    if (!jwtToken) {
        throw 'invalid'
    }

    const decodedPayload = jwtDecode<JwtPayload>(jwtToken as string);
    return decodedPayload.character_info;
}

export type JwtPayload = {
    is_admin: boolean;
    character_info: CharacterInfo;
}

export type CharacterInfo = {
    alliance_id?: number,
    alliance_name?: string,
    character_id: number,
    character_name: string,
    corporation_id: number,
    corporation_name: string,
};
