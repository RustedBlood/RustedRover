use axum::{Router, routing::get};

use crate::web_service::handlers;

pub fn router_api() -> Router {
    Router::new().route("/search", get(handlers::search))
}

pub fn router_ui() -> Router {
    Router::new().route("search", get(handlers::search))
}
