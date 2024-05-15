use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
};

/// Rejection for [`axum::Json`].
#[derive(Debug)]
pub enum Rejection {
    /// A rejection returned by [`axum::Json`].
    Json(JsonRejection),
    /// A rejection returned by [`axum::extract::Query`].
    Query(QueryRejection),
    /// A serde_valid validation error.
    SerdeValid(serde_valid::validation::Errors),
}

impl IntoResponse for Rejection {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Json(rejection) => rejection.into_response(),
            Self::Query(rejection) => rejection.into_response(),
            Self::SerdeValid(error) => {
                let mut response = axum::Json(error).into_response();
                *response.status_mut() = StatusCode::UNPROCESSABLE_ENTITY;
                response
            }
        }
    }
}
