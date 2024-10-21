use crate::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs::{read_to_string, try_exists, write};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub(crate) forum_channel_ids: Vec<String>,
    pub(crate) tag_to_repo: HashMap<String, String>,
    pub(crate) tag_to_label: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            forum_channel_ids: vec![],
            tag_to_repo: HashMap::new(),
            tag_to_label: HashMap::new(),
        }
    }
}

pub static mut CONFIG: Option<&Config> = None;

pub async fn load_config() -> error::Result<Config> {
    let config_path = "config.toml";

    let config_exists = try_exists(config_path).await?;
    if !config_exists {
        let default_config_string = toml::to_string_pretty(&Config::default())?;
        write(config_path, default_config_string).await?;
        return Err(error::Error::NoConfigFound);
    }

    let config_string = read_to_string("config.toml").await?;
    let config = toml::from_str(&config_string)?;
    Ok(config)
}
