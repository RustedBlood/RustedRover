mod api_keeper; // api хранитель
mod launcher;
mod osint_kernel {
    pub mod osint_builder;
    pub mod osint_searcher;
    pub mod osint_utils;
} // Ядро osint утилиты
pub mod web_service;
#[tokio::main]
async fn main() {
    if let Err(e) = launcher::run().await {
        eprintln!("Критическая ошибка при запуске приложения:\n{}", e);
        std::process::exit(1);
    }
}
