use crate::api_keeper::ApiKeeper;
use crate::web_service;
use axum::{Router, routing};
use std::sync::Arc;
use tera::Tera;
use tower_http::services::ServeDir;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let api_config = read_configs()?;
    let tera_templates = load_templates().await?;

    let app = Router::new()
        .route("/", routing::get(web_service::handlers::index))
        .nest_service("/static", ServeDir::new("frontend/static"))
        .with_state(Arc::new(tera_templates))
        .with_state(Arc::new(api_config));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5354").await?;
    println!("--------\nСервер запущен на http://127.0.0.1:5354\n--------");
    axum::serve(listener, app).await?;

    Ok(())
}
fn read_configs() -> Result<ApiKeeper, Box<dyn std::error::Error>> {
    let keeper = ApiKeeper::new()?;
    if !keeper.is_api_none() {
        eprintln!(
            "Советуем добавить API разных сервисов в веб приложении, чтобы искать более подробную информацию!"
        )
    }
    Ok(keeper)
}

async fn load_templates() -> Result<Tera, Box<dyn std::error::Error>> {
    let template_dir = "frontend/templates/*";
    let mut tera = Tera::new(template_dir)?;
    tera.autoescape_on(vec!["html", ".html", ".htm"]);
    Ok(tera)
}
