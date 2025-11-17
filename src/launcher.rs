use crate::web_service::handlers;
use crate::{api_keeper::ApiKeeper, web_service::web_routers};
use axum::routing::get;
use axum::{Router, routing};
use std::sync::Arc;
use tera::Tera;
use tower_http::services::ServeDir;

pub struct AppState {
    pub tera_templates: Arc<Tera>,
    pub api_config: Arc<ApiKeeper>,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let api_config = Arc::new(read_configs()?);
    let tera_templates = Arc::new(load_templates().await?);

    let app_state = AppState {
        tera_templates,
        api_config,
    };

    let app = Router::new()
        .nest_service("/api", web_routers::router_api())
        .nest_service("/ui", web_routers::router_ui())
        .route("/", get(handlers::index))
        .nest_service("/static", ServeDir::new("frontend/static"))
        .with_state(Arc::new(app_state));

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
