import { jwtDecode } from 'jwt-decode';
import { Route as LoginRoute } from "@/routes/auth/login";
import axios from "axios";
import type { CharacterInfo } from "@/services/character";

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

    const instance = axios
        .create({
            headers: {
                'authorization': jwtToken,
            }
        });

    instance.interceptors.response.use(
        undefined,
        async (error) => {
            if (error.response?.status === 401) {
                window.location.href = LoginRoute.to;
            }
        }
    )

    return instance;
}

export const refreshToken = async () => {
    return await axios
        .get('/api/auth/token')
        .then(x => {
            jwtToken = x.data.access_token;
        })
        .catch(e => {
            if (e.response?.status === 401) {
                window.location.href = LoginRoute.to;
            }

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
