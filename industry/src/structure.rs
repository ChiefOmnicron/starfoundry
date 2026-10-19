pub mod create;
pub mod delete;
pub mod fetch;
pub mod list;
pub mod permission;
pub mod update;

pub mod error;

use std::collections::HashMap;

use axum::middleware;
use serde::{Deserialize, Serialize};
use starfoundry_eve_gateway::asset::Item;
use starfoundry_eve_gateway::structure::{
    StructurePosition, StructureRigResponse, StructureServiceResponse, StructureType
};
use starfoundry_eve_gateway::system::System;
use starfoundry_lib_types::{CategoryId, GroupId, TypeId};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub use self::error::StructureError;
use crate::structure::permission::{assert_exists, assert_read, assert_write};
use crate::{AppState, StructureUuid};

pub fn routes(state: AppState) -> OpenApiRouter<AppState> {
    let list = OpenApiRouter::new().routes(routes!(list::api));

    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let create = OpenApiRouter::new().routes(routes!(create::api));

    let update = OpenApiRouter::new()
        .routes(routes!(update::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_write,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let delete = OpenApiRouter::new()
        .routes(routes!(delete::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_write,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    OpenApiRouter::new()
        .merge(list)
        .merge(fetch)
        .merge(create)
        .merge(update)
        .merge(delete)
}

#[cfg(test)]
pub async fn structure_test_routes(
    postgres: sqlx::PgPool,
    request: axum::http::Request<axum::body::Body>,
) -> axum::http::Response<axum::body::Body> {
    use std::sync::Arc;

    use tower::ServiceExt;

    use crate::metrics::Metric;

    let state: AppState = AppState {
        postgres: postgres.clone(),
        metric:   Arc::new(Metric::new()),
    };
    let (app, _) = crate::structure::routes(state.clone()).split_for_parts();
    let app = app.with_state(state.clone());

    app.clone()
        .oneshot(request)
        .await
        .unwrap()
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "15bd47e3-6b38-4cc1-887b-94924fff30a1",
        "name": "1DQ1-A - RIP",
        "structure_id": 1337,
        "structure_type": "KEEPSTAR",
        "system": {
            "constellation_id": 20000696,
            "constellation_name": "O-EIMK",
            "region_id": 10000060,
            "region_name": "Delve",
            "system_id": 30004759,
            "system_name": "1DQ1-A",
            "security": -0.38578233,
            "security_group": "NULLSEC",
        },
        "item": {
            "base_price": null,
            "category_id": 65,
            "group_id": 1657,
            "meta_group_id": 1,
            "name": "Keepstar",
            "repackaged": null,
            "type_id": 35834,
            "volume": 800000
        },
        "rigs": [{
            "item": {
                "type_id": 46496,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1939,
                    "category_id": 66,
                    "name": "Structure Reactor Rig L - Efficiency"
                },
                "volume": 20,
                "name": "Standup L-Set Reactor Efficiency I",
                "meta_group": 54,
                "repackaged": null
            },
            "excludes": [
                46497
            ],
            "material": 2,
            "time": 20,
            "categories": [],
            "groups": [
                {
                    "group_id": 428,
                    "category_id": 4,
                    "name": "Intermediate Materials"
                },
                {
                    "group_id": 429,
                    "category_id": 4,
                    "name": "Composite"
                },
                {
                    "group_id": 712,
                    "category_id": 4,
                    "name": "Biochemical Material"
                },
                {
                    "group_id": 974,
                    "category_id": 4,
                    "name": "Hybrid Polymers"
                },
                {
                    "group_id": 4096,
                    "category_id": 4,
                    "name": "Molecular-Forged Materials"
                },
                {
                    "group_id": 4932,
                    "category_id": 4,
                    "name": "Unrefined Mineral"
                }
            ]
        }],
        "service": [{
            "base_price": null,
            "category_id": 66,
            "group_id": 1321,
            "meta_group_id": 54,
            "name": "Standup Market Hub I",
            "repackaged": null,
            "type_id": 35892,
            "volume": 32000
        }],
        "taxes": {
            "35878": 1
        }
    })
)]
pub struct Structure {
    /// Internal id of the structure
    pub id:             StructureUuid,
    /// EVE Id of the structure
    pub structure_id:   i64,
    /// Name of the structure
    pub name:           String,
    /// Location of the structure
    pub system:         System,
    /// Type information
    pub item:           Item,
    /// List of all rigs that are in the structure
    pub rigs:           Vec<StructureRigResponse>,
    /// Id of the structure in-game
    pub services:       Vec<Item>,
    /// Position of the structure in the system
    pub position:       StructurePosition,
    /// Type of the structure
    pub structure_type: StructureType,
    /// Taxes by service type id
    pub taxes:          HashMap<TypeId, f32>,

    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installable_rigs:     Option<Vec<StructureRigResponse>>,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installable_services: Option<StructureServiceResponse>,
}

impl Structure {
    pub fn categories(&self) -> Vec<CategoryId> {
        self.rigs
            .iter()
            .flat_map(|y| {
                y.categories
                    .clone()
            })
            .map(|y| y.category_id)
            .collect::<Vec<_>>()
    }

    pub fn groups(&self) -> Vec<GroupId> {
        self.rigs
            .iter()
            .flat_map(|y| {
                y.groups
                    .clone()
            })
            .map(|y| y.group_id)
            .collect::<Vec<_>>()
    }

    pub fn joined_categories_groups(&self) -> Vec<i32> {
        let mut result = Vec::new();

        let categories = self
            .categories()
            .into_iter()
            .map(|x| *x);
        let groups = self
            .groups()
            .into_iter()
            .map(|x| *x);

        result.extend(categories);
        result.extend(groups);
        result
    }

    pub fn rig_bonus_by_security(&self) -> f32 {
        match (
            self.system
                .security_str
                .as_ref(),
            &self.structure_type,
        ) {
            // Refinery
            ("HIGHSEC", StructureType::Athanor)
            | ("HIGHSEC", StructureType::Tatara) => 0.0f32,
            ("LOWSEC", StructureType::Athanor)
            | ("LOWSEC", StructureType::Tatara) => 1.0f32,
            ("NULLSEC", StructureType::Athanor)
            | ("NULLSEC", StructureType::Tatara) => 1.1f32,

            // Engineering
            ("HIGHSEC", StructureType::Raitaru)
            | ("HIGHSEC", StructureType::Azbel)
            | ("HIGHSEC", StructureType::Sotiyo) => 1.0f32,
            ("LOWSEC", StructureType::Raitaru)
            | ("LOWSEC", StructureType::Azbel)
            | ("LOWSEC", StructureType::Sotiyo) => 1.9f32,
            ("NULLSEC", StructureType::Raitaru)
            | ("NULLSEC", StructureType::Azbel)
            | ("NULLSEC", StructureType::Sotiyo) => 2.1f32,

            // Invalid
            _ => 0.0f32,
        }
    }
}
