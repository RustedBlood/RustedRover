use crate::{
    launcher::AppState,
    osint_kernel::{osint_builder::OsintInfo, osint_searcher::searcher},
};
use axum::{
    Json,
    extract::State,
    response::{Html, IntoResponse, Response},
};
use std::sync::Arc;
use tera::{Context, Tera};

pub async fn index(State(app_state): State<Arc<AppState>>) -> Response {
    let ctx = Context::new();

    let templates = app_state.tera_templates.clone();
    render_template(templates, ctx, "index.html").await
}

pub async fn search(Json(payload): Json<OsintInfo>) {
    println!("start");
    searcher(payload);
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
