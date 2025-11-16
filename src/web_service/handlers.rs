use crate::osint_kernel::osint_builder::OsintInfo;
use axum::{
    Json,
    extract::State,
    response::{Html, IntoResponse, Response},
};
use std::sync::Arc;
use tera::{Context, Tera};

pub async fn index(State(tera): State<Arc<Tera>>) -> Response {
    let ctx = Context::new();
    render_template(tera, ctx, "index.html").await
}

pub async fn search(Json(payload): Json<OsintInfo>) -> Json<OsintInfo> {
    Json(payload)
}

async fn render_template(tera: Arc<Tera>, ctx: Context, template_name: &str) -> Response {
    match tera.render(template_name, &ctx) {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Возникла ошибка обработки шаблона: {}", e);
            Html("Ошибка сервера".to_string()).into_response()
        }
    }
}
