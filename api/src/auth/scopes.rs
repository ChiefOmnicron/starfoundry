const ESI_PUBLIC_DATA: &str                    = "publicData";

const ESI_UNIVERSE_STRUCTURES: &str            = "esi-universe.read_structures.v1";

const ESI_READ_STRUCTURE_MARKETS: &str         = "esi-markets.structure_markets.v1";

const ESI_READ_SKILLS: &str                    = "esi-skills.read_skills.v1";

const ESI_READ_BLUEPRINTS: &str                = "esi-characters.read_blueprints.v1";
const ESI_READ_CORPORATION_BLUEPRINTS: &str    = "esi-corporations.read_blueprints.v1";

const ESI_READ_ASSETS: &str                    = "esi-assets.read_assets.v1";
const ESI_READ_CORPORATION_ASSETS: &str        = "esi-assets.read_corporation_assets.v1";

const ESI_READ_INDUSTRY_JOBS: &str             = "esi-industry.read_character_jobs.v1";
const ESI_READ_CORPORATION_INDUSTRY_JOBS: &str = "esi-industry.read_corporation_jobs.v1";

pub const ESI_CHARACTER: &[&str] = &[
    ESI_PUBLIC_DATA,

    ESI_READ_ASSETS,
    ESI_READ_BLUEPRINTS,

    ESI_READ_INDUSTRY_JOBS,
    ESI_READ_STRUCTURE_MARKETS,

    ESI_UNIVERSE_STRUCTURES,
    ESI_READ_SKILLS,
];

pub const ESI_CORPORATION: &[&str] = &[
    ESI_READ_CORPORATION_ASSETS,
    ESI_READ_CORPORATION_BLUEPRINTS,
    ESI_READ_CORPORATION_INDUSTRY_JOBS,
];
