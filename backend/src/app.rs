use crate::api_keeper::ApiKeeper;

pub fn launch() -> Result<(), Box<dyn std::error::Error>> {
    let keeper = ApiKeeper::new()?;
    if !keeper.is_api_none() {
        eprintln!(
            "Советуем добавить API разных сервисов в веб приложении, чтобы искать более подробную информацию!"
        )
    }
    Ok(())
}
