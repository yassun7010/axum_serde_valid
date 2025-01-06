#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! A simple crate provides a drop-in replacement for [`axum::Json`]
//! that uses [jsonschema](https://docs.rs/jsonschema/latest/jsonschema/) to validate requests schemas
//! generated via [schemars](https://docs.rs/schemars/latest/schemars/).
//!
//! You might want to do this in order to provide a better
//! experience for your clients and not leak serde's error messages.
//!
//! All schemas are cached in a thread-local storage for
//! the life of the application (or thread).
//!
//! # Features
//!
//! - aide: support for [aide](https://docs.rs/aide/latest/aide/)

use axum::extract::Request;
use axum::{extract::FromRequest, response::IntoResponse};
use serde::Serialize;
use std::ops::Deref;

/// Wrapper type over [`axum::Json`] that validates
/// requests and responds with a more helpful validation
/// message.
pub struct Json<T>(pub T);

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for Json<T> {
    fn from(data: T) -> Self {
        Json(data)
    }
}

impl<T, S> FromRequest<S> for Json<T>
where
    T: serde::de::DeserializeOwned + serde_valid::Validate + 'static,
    S: Send + Sync,
{
    type Rejection = crate::extract::rejection::JsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let data: T = axum::Json::from_request(req, state)
            .await
            .map_err(crate::extract::rejection::JsonRejection::Json)?
            .0;

        data.validate()
            .map_err(crate::extract::rejection::JsonRejection::SerdeValid)?;

        Ok(Json(data))
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

#[cfg(test)]
mod test {
    use crate::Json;
    use axum::http::StatusCode;
    use axum::{
        body::Body,
        http::{self, Request},
    };
    use serde::Deserialize;
    use serde_json::json;
    use serde_valid::Validate;
    use tower::ServiceExt;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_json() -> TestResult {
        use axum::{routing::post, Router};

        #[derive(Deserialize, Validate)]
        struct User {
            #[validate(max_length = 3)]
            name: String,
        }

        let app = Router::new().route("/json", post(|_user: Json<User>| async move { "hello" }));

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/json")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_vec(&json!({"name": "taro"}))?))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(
                &axum::body::to_bytes(response.into_body(), 1_000_000).await?,
            )?,
            json!({
                "errors": [],
                "properties": {
                    "name": {
                        "errors": ["The length of the value must be `<= 3`."]
                    }
                }
            })
        );

        Ok(())
    }
}
