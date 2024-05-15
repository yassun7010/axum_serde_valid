use axum::{http::StatusCode, response::IntoResponse};

/// Rejection for [`crate::Json`].
#[derive(Debug)]
pub enum JsonRejection {
    /// A rejection returned by [`axum::Json`].
    Json(axum::extract::rejection::JsonRejection),
    /// A serde_valid validation error.
    SerdeValid(serde_valid::validation::Errors),
}

impl IntoResponse for JsonRejection {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Json(rejection) => rejection.into_response(),
            Self::SerdeValid(error) => {
                let mut response = axum::Json(error).into_response();
                *response.status_mut() = StatusCode::UNPROCESSABLE_ENTITY;
                response
            }
        }
    }
}

/// Rejection for [`crate::extract::Query`].
#[derive(Debug)]
pub enum QueryRejection {
    /// A rejection returned by [`axum::extract::Query`].
    Query(axum::extract::rejection::QueryRejection),
    /// A serde_valid validation error.
    SerdeValid(serde_valid::validation::Errors),
}

impl IntoResponse for QueryRejection {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Query(rejection) => rejection.into_response(),
            Self::SerdeValid(error) => {
                let mut response = axum::Json(error).into_response();
                *response.status_mut() = StatusCode::UNPROCESSABLE_ENTITY;
                response
            }
        }
    }
}
