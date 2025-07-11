use axum::response::{IntoResponse, Response};

pub struct JsonWrapper<T>(pub T);

impl<T> IntoResponse for JsonWrapper<T>
    where
        axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}
