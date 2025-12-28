use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_types::CharacterId;
use utoipa::{IntoParams, ToSchema};

use crate::project_group::ProjectGroupUuid;
use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;

pub async fn list(
    pool:                   &PgPool,
    character_id:           CharacterId,
    filter:                 ProjectFilter,
    eve_gateway_api_client: &impl EveGatewayApiClient,
) -> Result<Vec<ProjectList>> {
    let project_groups = if let Some(x) = filter.project_group {
        vec![*x]
    } else {
        crate::project_group::service::list(
            pool,
            character_id,
            eve_gateway_api_client,
            Default::default()
        )
            .await?
            .into_iter()
            .map(|x| *x.id)
            .collect::<Vec<_>>()
    };

    let filter_status: Vec<String> = if let Some(x) = filter.status.clone() {
        x
            .split(",")
            .map(|x| x.into())
            .collect::<Vec<_>>()
    } else {
        vec![
            "PREPARING".into(),
            "IN_PROGRESS".into(),
            "PAUSED".into(),
            "DONE".into(),
        ]
    };

    let entries = sqlx::query!(r#"
            SELECT
                id,
                name,
                status AS "status: ProjectStatus",
                orderer,
                sell_price
            FROM project
            WHERE
                (
                    NOT (LOWER(name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                    NOT (LOWER(orderer) LIKE '%' || LOWER($5) || '%') IS FALSE AND
                    NOT (status = ANY($3::PROJECT_STATUS[])) IS FALSE
                )
                AND
                (
                    -- check if the character is in the group
                    (
                        NOT (project_group_id = ANY($4::UUID[])) IS FALSE
                    )
                    OR
                    -- as a fallback check if the character is the owner
                    (
                        owner = $1
                    )
                )
            ORDER BY name
        "#,
            *character_id,
            filter.name,
            &filter_status as _,
            &project_groups,
            filter.orderer,
        )
        .fetch_all(pool)
        .await
        .map_err(ProjectError::ListProjects)?;

    let mut project_groups = Vec::new();
    for entry in entries {
        let project_group = ProjectList {
            id:         entry.id.into(),
            name:       entry.name,
            status:     entry.status,
            orderer:    entry.orderer,
            sell_price: entry.sell_price,
        };
        project_groups.push(project_group);
    }

    Ok(project_groups)
}

#[derive(Debug, Default, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProjectFilter {
    #[serde(default)]
    #[param(
        example = json!("Project 1337"),
        required = false,
    )]
    pub name:   Option<String>,

    #[param(
        default = json!("PREPARING,IN_PROGRESS,PAUSED,DONE"),
        required = false,
    )]
    #[serde(default = "default_status")]
    pub status: Option<String>,

    #[serde(default)]
    #[param(
        example = json!("019b5d76-0ebd-77f4-80b0-12daf86501b6"),
        required = false,
    )]
    pub project_group: Option<ProjectGroupUuid>,

    #[serde(default)]
    #[param(
        example = json!("Eistonen Kodan Sasen"),
        required = false,
    )]
    pub orderer:       Option<String>,
}

fn default_status() -> Option<String> {
    Some("PREPARING,IN_PROGRESS,PAUSED,DONE".into())
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "b034c3a9-2f4d-487d-95bb-c66fc20148b3",
        "name": "My cool project",
        "status": "IN_PROGRESS",
        "orderer": "Me Myself and I",
        "sell_price": 1337
    })
)]
pub struct ProjectList {
    pub id:         ProjectUuid,
    pub name:       String,
    pub status:     ProjectStatus,
    pub orderer:    String,

    pub sell_price: Option<f64>,
}

/// Different states of the project
/// 
/// A newly created project will always be in the status `Preparing`.
/// When the projects switches into `InProgress` the job detection gets active
/// for that project.
/// Afterwards the project is either `Done` or `Closed`. Job detection then gets
/// deactivated again.
/// 
#[derive(
    Clone, Debug, Copy, Hash,
    PartialEq, Eq, PartialOrd, Ord,
    sqlx::Type, Deserialize, Serialize, ToSchema,
)]
#[sqlx(type_name = "PROJECT_STATUS")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectStatus {
    /// the project has not started yet, but materials are gathered
    /// job detection not active
    Preparing,
    /// the project is currently in progress, and job detection is active
    InProgress,
    /// the project is currently paused, job detection not active
    Paused,
    /// the project is finished, industry job detection is no longer active
    Done,
}


#[cfg(test)]
mod list_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;

    use crate::project::service::list::ProjectFilter;
    use crate::test_util::EveGatewayTestApiClient;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base")
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let gateway_client = EveGatewayTestApiClient::new();
        let result = super::list(
                &pool,
                CharacterId(1),
                ProjectFilter::default(),
                &gateway_client,
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 4);

        let result = super::list(
                &pool,
                CharacterId(1),
                ProjectFilter {
                    name: Some(String::from("Filter")),
                    ..Default::default()
                },
                &gateway_client,
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);

        let result = super::list(
                &pool,
                CharacterId(1),
                ProjectFilter {
                    name: Some(String::from("SomeGibberish")),
                    ..Default::default()
                },
                &gateway_client,
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
