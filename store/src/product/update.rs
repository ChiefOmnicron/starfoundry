use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::config::ProductUuid;
use crate::product::error::{ProductError, Result};
use crate::product::create::CreateProduct;
use crate::product::util::resolve_items;

/// Update Product
/// 
/// - Alternative route: `/latest/products/{ProductUuid}`
/// - Alternative route: `/v1/products/{ProductUuid}`
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
    put,
    path = "/{ProductUuid}",
    tag = "products",
    request_body = CreateProduct,
    responses(
        (
            description = "The item was successfully updated",
            status = NO_CONTENT,
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
    State(state):       State<AppState>,
    Path(product_uuid): Path<ProductUuid>,
    Json(info):         Json<CreateProduct>,
) -> Result<impl IntoResponse> {
    info.valid()?;

    let appraisal = resolve_items(info.content).await?;
    sqlx::query!("
            UPDATE product SET
                category = $2,
                name = $3,
                price = $4,
                image_type = $5,
                image_type_id = $6,
                description = $7,
                tags = $8,
                content = $9,
                additional_products = $10
            WHERE uuid = $1
        ",
            *product_uuid,
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
        .execute(&state.postgres)
        .await
        .map_err(ProductError::GeneralSqlxError)?;

    Ok(
        (
            StatusCode::NO_CONTENT,
            Json(())
        )
    )
}
