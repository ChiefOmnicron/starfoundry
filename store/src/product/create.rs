use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::config::ProductUuid;
use crate::product::error::{ProductError, Result};
use crate::product::util::resolve_items;

/// Create Product
/// 
/// - Alternative route: `/latest/products`
/// - Alternative route: `/v1/products`
/// 
/// ---
/// 
/// Creates a new product
/// 
/// ## Security
/// - authenticated
/// - admin
/// 
#[utoipa::path(
    post,
    path = "/",
    tag = "products",
    request_body = CreateProduct,
    responses(
        (
            body = CreateProductResponse,
            description = "Id of the new product",
            status = CREATED,
        ),
        BadRequest,
        Unauthorized,
        UnsupportedMediaType,
        UnprocessableEntity,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Json(info):   Json<CreateProduct>,
) -> Result<impl IntoResponse> {
    info.valid()?;

    let appraisal = resolve_items(info.content).await?;
    let product_uuid: Uuid = sqlx::query!("
            INSERT INTO product (
                category,
                name,
                price,
                image_type,
                image_type_id,
                description,
                tags,
                content,
                additional_products
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING uuid
        ",
            info.category,
            info.name,
            info.price,
            info.image_type,
            *info.image_type_id,
            info.description,
            &info.tags,
            &appraisal,
            &info.additional_options.iter().map(|x| *x.reference_id).collect::<Vec<_>>(),
        )
        .fetch_one(&state.postgres)
        .await
        .map(|x| x.uuid.into())
        .map_err(ProductError::GeneralSqlxError)?;

    Ok(
        (
            StatusCode::CREATED,
            Json(CreateProductResponse {
                id: product_uuid.into(),
            })
        )
    )
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateProductResponse {
    id: ProductUuid,
}

#[derive(Debug, Deserialize, ToSchema)]
#[cfg_attr(test, derive(serde::Serialize))]
#[schema(
    example = json!({
        "name": "Revelation Navy Issue",
        "price": 1000,
        "image_type": "render",
        "image_type_id": 73790,
    })
)]
pub struct CreateProduct {
    /// Maximum length 100
    pub name:               String,
    /// Price of the product
    pub price:              i64,

    /// Type of the icon that is inserted into the image server
    /// More info: https://developers.eveonline.com/docs/services/image-server/
    pub image_type:         String,
    /// TypeId of the image that should be shown
    pub image_type_id:      TypeId,

    /// Maximum length 10_000
    pub description:        Option<String>,
    /// Name of the category
    pub category:           Option<String>,
    /// List of filterable tags
    #[serde(default)]
    pub tags:               Vec<String>,

    /// Content of everything in the product
    pub content:            serde_json::Value,
    pub additional_options: Vec<AdditionalOption>
}

impl CreateProduct {
    pub fn valid(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self.name.trim().is_empty() {
                return Err(ProductError::ValidationError("Field 'name' must be set".into()));
            }
        } else {
            return Err(ProductError::ValidationError("Field 'name' is too long, max length: 100".into()));
        };

        if self.price < 0 {
            return Err(ProductError::ValidationError("Field 'price' must be at least 0".into()));
        };

        match &self.description {
            Some(x) => {
                if x.len() >= 10_000 {
                    return Err(ProductError::ValidationError("Field 'description' is too long, max length: 10_000".into()));
                }
                Some(x)
            },
            None => None,
        };

        Ok(true)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct AdditionalOption {
    //pub typ:            String,
    pub reference_id:   ProductUuid,
}
