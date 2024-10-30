use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs::{read_to_string, try_exists, write};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub(crate) forum_channel_ids: Vec<u64>,
    pub(crate) tag_to_repo: HashMap<u64, String>,
    pub(crate) tag_to_label: HashMap<u64, String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SavedData {
    pub(crate) channel_id_to_issue: HashMap<u64, i32>,
}

pub static mut CONFIG: Option<Config> = None;
pub static mut SAVED_DATA: Option<SavedData> = None;

pub async fn load_file<T: ?Sized>(
    path: &str,
    has_to_exist: bool,
    default: &T,
) -> crate::error::Result<T>
where
    T: serde::ser::Serialize,
    T: serde::de::DeserializeOwned,
{
    let exists = try_exists(path).await?;
    if !exists {
        let default_string = toml::to_string_pretty(default)?;
        write(path, default_string).await?;
        if has_to_exist {
            return Err(crate::error::Error::NoConfigFound);
        }
    }

    let string = read_to_string(path).await?;
    let deserialized = toml::from_str(&string)?;
    Ok(deserialized)
}
