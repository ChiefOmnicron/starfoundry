mod error;
mod fetch_rig;
mod fetch_rig_blueprint_bonus;
mod fetch_service;
mod list_structure_rigs;
mod list_structure_services;
mod resolve_structure;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use starfoundry_gateway::ApiClient;
use starfoundry_lib_types::{StructureId, TypeId};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub use self::fetch_rig::StructureRigResponse;
pub use self::fetch_service::StructureServiceResponse;
use crate::state::AppState;
use crate::structure::error::Result;
use crate::structure::fetch_rig::StructureRigBlueprintBonus;
use crate::structure::resolve_structure::ResolveStructureResponse;

/// Exposes all routes that are under `/structures`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch_rig = OpenApiRouter::new().routes(routes!(fetch_rig::api));

    let fetch_rig_blueprints =
        OpenApiRouter::new().routes(routes!(fetch_rig_blueprint_bonus::api));

    let fetch_services =
        OpenApiRouter::new().routes(routes!(fetch_service::api));

    let list_structure_rigs =
        OpenApiRouter::new().routes(routes!(list_structure_rigs::api));

    let list_structure_services =
        OpenApiRouter::new().routes(routes!(list_structure_services::api));

    let resolve_structure =
        OpenApiRouter::new().routes(routes!(resolve_structure::api));

    OpenApiRouter::new()
        .merge(fetch_rig)
        .merge(fetch_rig_blueprints)
        .merge(fetch_services)
        .merge(list_structure_rigs)
        .merge(list_structure_services)
        .merge(resolve_structure)
}

pub trait EveGatewayApiClientStructure: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn resolve_structure(
        &self,
        structure_id: StructureId,
    ) -> Result<Option<ResolveStructureResponse>> {
        self.fetch(&format!("structures/{}", *structure_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_rig(
        &self,
        rig_type_id: TypeId,
    ) -> Result<Option<StructureRigResponse>> {
        self.fetch(&format!("structures/rigs/{}", *rig_type_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_service(
        &self,
        service_type_id: TypeId,
    ) -> Result<Option<StructureServiceResponse>> {
        self.fetch(&format!("structures/services/{}", *service_type_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_structure_rigs(
        &self,
        structure_type_id: TypeId,
    ) -> Result<Vec<StructureRigResponse>> {
        self.fetch(&format!("structures/{}/rigs", *structure_type_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_structure_services(
        &self,
        structure_type_id: TypeId,
    ) -> Result<StructureServiceResponse> {
        self.fetch(&format!("structures/{}/services", *structure_type_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_rig_blueprints(
        &self,
        rig_type_ids: Vec<TypeId>,
    ) -> Result<Vec<StructureRigBlueprintBonus>> {
        self.post("structures/rigs", rig_type_ids)
            .await
            .map_err(Into::into)
    }
}

/// Coordinates of a structure within the system
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct StructurePosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum BonusVariations {
    Time(f32),
    Material(f32),
    Isk(f32),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum StructureType {
    /// https://everef.net/type/35835
    Athanor,
    /// https://everef.net/type/35836
    Tatara,

    /// https://everef.net/type/35825
    Raitaru,
    /// https://everef.net/type/35826
    Azbel,
    /// https://everef.net/type/35827
    Sotiyo,

    /// https://everef.net/type/35832
    Astrahus,
    /// https://everef.net/type/35833
    Fortizar,
    /// https://everef.net/type/35834
    Keepstar,

    /// https://everef.net/type/47512
    MoreauFortizar,
    /// https://everef.net/type/47513
    DraccousFortizar,
    /// https://everef.net/type/47514
    HorizonFortizar,
    /// https://everef.net/type/47515
    MarginisFortizar,
    /// https://everef.net/type/47516
    PrometheusFortizar,
    /// https://everef.net/type/40340
    PalatineKeepstar,

    NpcAmarr,
    NpcJita,

    Invalid,
}

impl StructureType {
    /// Bonuses applied by the structure itself
    pub fn bonus(&self) -> Vec<BonusVariations> {
        match self {
            // Refinery
            Self::Athanor => Vec::new(),
            Self::Tatara => vec![BonusVariations::Time(25.00f32)],

            // Engineering
            Self::Raitaru => {
                vec![
                    BonusVariations::Time(15.00f32),
                    BonusVariations::Material(1.00f32),
                    BonusVariations::Isk(3.00f32),
                ]
            },
            Self::Azbel => {
                vec![
                    BonusVariations::Time(20.00f32),
                    BonusVariations::Material(1.00f32),
                    BonusVariations::Isk(4.00f32),
                ]
            },
            Self::Sotiyo => {
                vec![
                    BonusVariations::Time(30.00f32),
                    BonusVariations::Material(1.00f32),
                    BonusVariations::Isk(5.00f32),
                ]
            },
            Self::Astrahus
            | Self::Fortizar
            | Self::Keepstar
            | Self::MoreauFortizar
            | Self::DraccousFortizar
            | Self::HorizonFortizar
            | Self::MarginisFortizar
            | Self::PrometheusFortizar
            | Self::PalatineKeepstar
            | Self::NpcAmarr
            | Self::NpcJita
            | Self::Invalid => Vec::new(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: i32 = Deserialize::deserialize(deserializer)?;
        Ok(StructureType::from(v))
    }

    pub fn serialize<S>(
        value: &Self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(value.into())
    }

    pub fn into_i32(&self) -> i32 {
        self.into()
    }
}

impl From<i32> for StructureType {
    fn from(x: i32) -> Self {
        match x {
            35835 => Self::Athanor,
            35836 => Self::Tatara,

            35825 => Self::Raitaru,
            35826 => Self::Azbel,
            35827 => Self::Sotiyo,

            35832 => Self::Astrahus,
            35833 => Self::Fortizar,
            35834 => Self::Keepstar,

            47512 => Self::MoreauFortizar,
            47513 => Self::DraccousFortizar,
            47514 => Self::HorizonFortizar,
            47515 => Self::MarginisFortizar,
            47516 => Self::PrometheusFortizar,
            40340 => Self::PalatineKeepstar,

            1932 => Self::NpcAmarr,
            52678 => Self::NpcJita,

            _ => Self::Invalid,
        }
    }
}

impl From<TypeId> for StructureType {
    fn from(x: TypeId) -> Self {
        Self::from(*x)
    }
}

impl From<StructureType> for i32 {
    fn from(x: StructureType) -> Self {
        Self::from(&x)
    }
}

impl From<StructureType> for TypeId {
    fn from(x: StructureType) -> Self {
        Self::from(&x)
    }
}

impl From<&StructureType> for i32 {
    fn from(x: &StructureType) -> Self {
        match *x {
            StructureType::Athanor => 35835,
            StructureType::Tatara => 35836,

            StructureType::Raitaru => 35825,
            StructureType::Azbel => 35826,
            StructureType::Sotiyo => 35827,

            StructureType::Astrahus => 35832,
            StructureType::Fortizar => 35833,
            StructureType::Keepstar => 35834,

            StructureType::MoreauFortizar => 47512,
            StructureType::DraccousFortizar => 47513,
            StructureType::HorizonFortizar => 47514,
            StructureType::MarginisFortizar => 47515,
            StructureType::PrometheusFortizar => 47516,
            StructureType::PalatineKeepstar => 40340,

            StructureType::NpcAmarr => 1932,
            StructureType::NpcJita => 52678,

            StructureType::Invalid => 0,
        }
    }
}

impl From<&StructureType> for TypeId {
    fn from(x: &StructureType) -> Self {
        match *x {
            StructureType::Athanor => 35835,
            StructureType::Tatara => 35836,

            StructureType::Raitaru => 35825,
            StructureType::Azbel => 35826,
            StructureType::Sotiyo => 35827,

            StructureType::Astrahus => 35832,
            StructureType::Fortizar => 35833,
            StructureType::Keepstar => 35834,

            StructureType::MoreauFortizar => 47512,
            StructureType::DraccousFortizar => 47513,
            StructureType::HorizonFortizar => 47514,
            StructureType::MarginisFortizar => 47515,
            StructureType::PrometheusFortizar => 47516,
            StructureType::PalatineKeepstar => 40340,

            StructureType::NpcAmarr => 1932,
            StructureType::NpcJita => 52678,

            StructureType::Invalid => 0,
        }
        .into()
    }
}
