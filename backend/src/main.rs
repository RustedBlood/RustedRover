mod api_keeper; // api хранитель
mod app;
mod osint_kernel; // Ядро osint утилиты
mod web_service; // Сервис создания веб интерфейса

fn main() {
    app::launch().unwrap_or_else(|e| {
        eprintln!("Возникла ошибка во время запуска программы!\nОшибка: {}", e);
        std::process::exit(1);
    })
}
