use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Copy, Clone, Debug, Deserialize, ToSchema)]
pub enum OreReprocessingEfficiency {
    HsAthanorNoRig,
    HsAthanorT1,
    HsAthanorT2,
    HsTataraNoRig,
    HsTataraT1,
    HsTataraT2,

    LsAthanorNoRig,
    LsAthanorT1,
    LsAthanorT2,
    LsTataraNoRig,
    LsTataraT1,
    LsTataraT2,

    NsAthanorNoRig,
    NsAthanorT1,
    NsAthanorT2,
    NsTataraNoRig,
    NsTataraT1,
    NsTataraT2,
}

impl Default for OreReprocessingEfficiency {
    fn default() -> Self {
        Self::NsTataraT2
    }
}

impl OreReprocessingEfficiency {
    pub fn efficiency(&self) -> f64 {
        match self {
            Self::HsAthanorNoRig => 0.7381,
            Self::HsAthanorT1    => 0.7528,
            Self::HsAthanorT2    => 0.7823,
            Self::HsTataraNoRig  => 0.7634,
            Self::HsTataraT1     => 0.7786,
            Self::HsTataraT2     => 0.8092,

            Self::LsAthanorNoRig => 0.7381,
            Self::LsAthanorT1    => 0.7980,
            Self::LsAthanorT2    => 0.8293,
            Self::LsTataraNoRig  => 0.7634,
            Self::LsTataraT1     => 0.8254,
            Self::LsTataraT2     => 0.8577,

            Self::NsAthanorNoRig => 0.7381,
            Self::NsAthanorT1    => 0.8432,
            Self::NsAthanorT2    => 0.8762,
            Self::NsTataraNoRig  => 0.7634,
            Self::NsTataraT1     => 0.8721,
            Self::NsTataraT2     => 0.9063,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub enum GasReprocessingEfficiency {
    AthanorLvl0,
    AthanorLvl1,
    AthanorLvl2,
    AthanorLvl3,
    AthanorLvl4,
    AthanorLvl5,

    TataraLvl0,
    TataraLvl1,
    TataraLvl2,
    TataraLvl3,
    TataraLvl4,
    TataraLvl5,
}

impl Default for GasReprocessingEfficiency {
    fn default() -> Self {
        Self::TataraLvl5
    }
}

impl GasReprocessingEfficiency {
    const BASE_DECOMPRESSION: f64    = 80f64;
    const ATHANOR_DECOMPRESSION: f64 = 4f64;
    const TATARA_DECOMPRESSION: f64  = 10f64;

    pub fn efficiency(&self) -> f64 {
        match self {
            Self::AthanorLvl0 => Self::BASE_DECOMPRESSION + Self::ATHANOR_DECOMPRESSION + 0f64,
            Self::AthanorLvl1 => Self::BASE_DECOMPRESSION + Self::ATHANOR_DECOMPRESSION + 1f64,
            Self::AthanorLvl2 => Self::BASE_DECOMPRESSION + Self::ATHANOR_DECOMPRESSION + 2f64,
            Self::AthanorLvl3 => Self::BASE_DECOMPRESSION + Self::ATHANOR_DECOMPRESSION + 3f64,
            Self::AthanorLvl4 => Self::BASE_DECOMPRESSION + Self::ATHANOR_DECOMPRESSION + 4f64,
            Self::AthanorLvl5 => Self::BASE_DECOMPRESSION + Self::ATHANOR_DECOMPRESSION + 5f64,

            Self::TataraLvl0  => Self::BASE_DECOMPRESSION + Self::TATARA_DECOMPRESSION + 0f64,
            Self::TataraLvl1  => Self::BASE_DECOMPRESSION + Self::TATARA_DECOMPRESSION + 1f64,
            Self::TataraLvl2  => Self::BASE_DECOMPRESSION + Self::TATARA_DECOMPRESSION + 2f64,
            Self::TataraLvl3  => Self::BASE_DECOMPRESSION + Self::TATARA_DECOMPRESSION + 3f64,
            Self::TataraLvl4  => Self::BASE_DECOMPRESSION + Self::TATARA_DECOMPRESSION + 4f64,
            Self::TataraLvl5  => Self::BASE_DECOMPRESSION + Self::TATARA_DECOMPRESSION + 5f64,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub enum ScrapReprocessingEfficiency {
    Lvl0,
    Lvl1,
    Lvl2,
    Lvl3,
    Lvl4,
    Lvl5,
}

impl Default for ScrapReprocessingEfficiency {
    fn default() -> Self {
        Self::Lvl5
    }
}

impl ScrapReprocessingEfficiency {
    const BASE_EFFICIENCY: f64 = 50f64;

    pub fn efficiency(&self) -> f64 {
        match self {
            Self::Lvl0  => Self::BASE_EFFICIENCY + 0f64,
            Self::Lvl1  => Self::BASE_EFFICIENCY + 1f64,
            Self::Lvl2  => Self::BASE_EFFICIENCY + 2f64,
            Self::Lvl3  => Self::BASE_EFFICIENCY + 3f64,
            Self::Lvl4  => Self::BASE_EFFICIENCY + 4f64,
            Self::Lvl5  => Self::BASE_EFFICIENCY + 5f64,
        }
    }
}
