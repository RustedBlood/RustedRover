use phonenumber::PhoneNumber;
use serde::{Deserialize, Serialize};

use crate::osint_kernel::osint_utils::ip::IpInfo;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OsintInfo {
    pub number: Option<String>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub ip: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OsintResponse {
    pub num_info: Option<PhoneNumber>,
    pub ip_info: Option<IpInfo>,
}

impl OsintInfo {
    pub fn validate_info(&self) {}
}
