import { type Uuid } from '@/utils';
import axios from 'axios';

const AUTH_BASE_PATH: string = '/api/v1/auth';
const CHARACTER_BASE_PATH: string = '/api/v1/characters';
const CORPORATIONS_BASE_PATH: string = '/api/v1/corporations';

export class Service {
    private static character_req: { [key: number]: Promise<ICharacter> } = {};
    private static characters: { [key: number]: ICharacter } = {};
    private static corporation_req: {
        [key: number]: Promise<ICorporationInfo>;
    } = {};
    private static corporations: { [key: number]: ICorporationInfo } = {};

    public static add_character() {
        window.location.href = `${AUTH_BASE_PATH}/login/alt`;
    }

    public static add_corporation() {
        window.location.href = `${AUTH_BASE_PATH}/login/corporation`;
    }

    public static async whoami(): Promise<ICharacter> {
        return await axios.get(`${AUTH_BASE_PATH}/whoami`).then((x) => x.data);
    }

    public static async alts(): Promise<ICharacter[]> {
        let characters = (await axios.get(`${CHARACTER_BASE_PATH}/alts`)).data;
        characters.map((x: ICharacter) => this.cache_user(x));

        return characters;
    }

    public static async alt_corporations(): Promise<ICharacter[]> {
        let characters = (
            await axios.get(`${CHARACTER_BASE_PATH}/corporations`)
        ).data;
        characters.map((x: ICharacter) => this.cache_user(x));

        return characters;
    }

    public static async remove(characterId: number): Promise<void> {
        return (await axios.delete(`${CHARACTER_BASE_PATH}/${characterId}`))
            .data;
    }

    public static async refresh(characterId: number): Promise<void> {
        return await axios.get(`${CHARACTER_BASE_PATH}/${characterId}/refresh`);
    }

    public static async industry_slots(
        characterId: number,
    ): Promise<IIndustrySlots> {
        return await axios.get(
            `${CHARACTER_BASE_PATH}/${characterId}/industry/slots`,
        );
    }

    public static async info(characterId: number): Promise<ICharacter> {
        // Ignore what typescript says
        if (<any>this.character_req[characterId]) {
            return this.character_req[characterId];
        } else if (this.characters[characterId]) {
            return this.characters[characterId];
        }

        if (this.characters[characterId]) {
            return this.characters[characterId];
        } else {
            this.character_req[characterId] = axios
                .get(`${CHARACTER_BASE_PATH}/${characterId}`)
                .then((x) => x.data)
                .then((x) => {
                    this.characters[characterId] = x;
                    delete this.character_req[characterId];
                    return x;
                });
            return this.character_req[characterId];
        }
    }

    public static async corporation_info(
        characterId: number,
    ): Promise<ICorporationInfo> {
        // Ignore what typescript says
        if (<any>this.corporation_req[characterId]) {
            return this.corporation_req[characterId];
        }

        if (this.corporations[characterId]) {
            return this.corporations[characterId];
        } else {
            this.corporation_req[characterId] = axios
                .get(`${CORPORATIONS_BASE_PATH}/${characterId}/info`)
                .then((x) => x.data)
                .then((x) => {
                    this.corporations[characterId] = x;
                    delete this.corporation_req[characterId];
                    return x;
                });
            return this.corporation_req[characterId];
        }
    }

    private static cache_user(character: ICharacter) {
        this.characters[character.character_id] = character;
    }
}

export interface ICharacter {
    id: Uuid;
    character_name: string;
    character_id: number;
    alliance_name: string;
    alliance_id: number;
    corporation_name: string;
    corporation_id: number;
    credential_type: 'CORPORATION' | 'CHARACTER';

    open?: boolean;
    industry_slots?: IIndustrySlots;
}

export interface ICorporationInfo {
    alliance_id: number;
    name: string;
}

export interface IScope {
    key: string;
    name: string;
    reason: string;
    scopes: string[];
}

export interface IIndustrySlots {
    manufacturing: number;
    reactions: number;
    science: number;
}
