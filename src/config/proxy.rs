use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Proxy {
    pub enabled: bool,
    pub kind: String,
    pub address: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}