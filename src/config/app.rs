use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub pretty_print: bool,
    pub sort_by: String,
}


pub fn load_app_config(filename: &str) -> Result<AppConfig, String> {
    let content = std::fs::read_to_string(filename).map_err(|e| e.to_string())?;
    toml::from_str(&content).map_err(|e| e.to_string())
}
