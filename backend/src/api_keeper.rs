use std::{fs, sync::Arc};

use serde::Deserialize;
use thiserror::Error;

const PATH_TO_APIS: &str = "/config/secrets.toml";

#[derive(Debug, Error)]
pub enum ApiKeeperErrors {
    #[error("Недостаточно прав для чтения и запись конфигурации")]
    PermissionDenied,

    #[error(
        "Ошибка десериализации TOML\nПроверьте целостность файла, либо удалите secrets.toml: {0}"
    )]
    InvalidToml(#[from] toml::de::Error), //Для обработки ошибки парсинга TOML

    #[error("Ошибка поиска конфига с API")]
    ConfigNotFound, //Для обработки ошибки при поиске конфига

    #[error("Ошибка ввода-вывода: {0}")]
    OtherIo(#[from] std::io::Error),
}

#[derive(Debug, Deserialize, Default)]
pub struct ApiKeys {
    pub shodan_api: Option<String>,
    pub hunter_api: Option<String>,
    pub numverify_api: Option<String>,
    pub vk_api: Option<String>,
    pub haveibeenpwned_api: Option<String>,
}

pub struct ApiKeeper {
    pub api_keeper: Arc<ApiKeys>,
}

impl ApiKeeper {
    pub fn new() -> Result<Self, ApiKeeperErrors> {
        let api_keys = Self::load_api_keys()?;

        let api_keeper = Arc::new(api_keys);
        Ok(Self { api_keeper })
    }
    fn load_api_keys() -> Result<ApiKeys, ApiKeeperErrors> {
        let keys: ApiKeys = if std::path::Path::new(PATH_TO_APIS).exists() {
            let content = fs::read_to_string(PATH_TO_APIS)?;
            toml::from_str(&content)?
        } else {
            ApiKeys::default()
        };

        Ok(keys)
    }
    pub fn is_api_none(&self) -> bool {
        self.api_keeper.haveibeenpwned_api.is_some()
            || self.api_keeper.hunter_api.is_some()
            || self.api_keeper.numverify_api.is_some()
            || self.api_keeper.vk_api.is_some()
    }
}
