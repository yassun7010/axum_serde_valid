# axum_serde_valid

[![Latest Version](https://img.shields.io/crates/v/axum_serde_valid.svg?color=green&style=flat-square)](https://crates.io/crates/axum_serde_valid)
[![Documentation](https://docs.rs/axum_serde_valid/badge.svg)](https://docs.rs/axum_serde_valid)
[![GitHub license](https://badgen.net/github/license/Naereen/Strapdown.js?style=flat-square)](https://github.com/Naereen/StrapDown.js/blob/master/LICENSE)

This crate is a Rust library for providing validation mechanism
to [axum](https://github.com/tokio-rs/axum) with [serde_valid](https://github.com/yassun7010/serde_valid) crate.

More information about this crate can be found in the [crate documentation](https://docs.rs/axum_serde_valid).

### Installation

This crate works with Cargo and can be found on [crates.io](https://crates.io/crates/axum_serde_valid) with a Cargo.toml like:

```toml
[dependencies]
axum = "0.6"
axum_serde_valid = "0.23.0"
serde = "^1.0"
serde_valid = "0.20"
```

### Example

```rust
use axum::{routing::post, Router};
use axum_serde_valid::Json;
use serde::Deserialize;
use serde_valid::Validate;

#[derive(Deserialize, Validate)]
struct User {
    #[validate(max_length = 3)]
    name: String,
}

let app = Router::<()>::new().route("/json", post(|user: Json<User>| async move { "hello" }));
```

License: MIT
