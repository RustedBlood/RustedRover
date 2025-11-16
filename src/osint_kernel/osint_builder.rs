use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OsintInfo {
    pub number: Option<String>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub ip: Option<String>,
}

impl OsintInfo {}
