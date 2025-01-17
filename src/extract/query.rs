#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! A simple crate provides a drop-in replacement for [`axum::extract::Query`]
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

use std::ops::Deref;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use serde::de::DeserializeOwned;

/// Wrapper type over [`axum::extract::Query`] that validates
/// requests with a more helpful validation
/// message.
pub struct Query<T>(pub T);

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for Query<T> {
    fn from(data: T) -> Self {
        Query(data)
    }
}

impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned + serde_valid::Validate,
    S: Send + Sync,
{
    type Rejection = crate::extract::rejection::QueryRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let data: T = axum::extract::Query::from_request_parts(parts, _state)
            .await
            .map_err(crate::extract::rejection::QueryRejection::Query)?
            .0;

        data.validate()
            .map_err(crate::extract::rejection::QueryRejection::SerdeValid)?;

        Ok(Query(data))
    }
}
