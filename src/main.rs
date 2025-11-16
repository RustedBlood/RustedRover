mod api_keeper; // api хранитель
mod launcher;
mod osint_kernel; // Ядро osint утилиты
mod web_service; // Сервис создания веб интерфейса //Запускает чтение конфигов и веб сервер

#[tokio::main]
async fn main() {
    if let Err(e) = launcher::run().await {
        eprintln!("Критическая ошибка при запуске приложения:\n{}", e);
        std::process::exit(1);
    }
}
