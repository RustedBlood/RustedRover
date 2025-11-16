use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IpInfo {
    pub ip: String,
    pub city: String,
    pub location: String,
}

pub async fn get_ip_info(ip: &str) -> Result<IpInfo, Box<dyn std::error::Error>> {
    let url = format!("https://ipinfo.io/{}/json", ip);
    let resp = reqwest::get(&url).await?.json::<IpInfo>().await?;
    Ok(resp)
}
