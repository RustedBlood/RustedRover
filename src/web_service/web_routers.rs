use axum::{Router, routing::get, routing::post};

use crate::web_service::handlers;

pub fn router_api() -> Router {
    Router::new().route("/search", post(handlers::search))
}

pub fn router_ui() -> Router {
    Router::new().route("/finded", get(handlers::search))
}
