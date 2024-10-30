mod config;
mod error;
mod event;
mod github;

use crate::config::{Config, SavedData};
use serenity::prelude::GatewayIntents;
use serenity::Client;
use std::env;

#[tokio::main]
async fn main() -> error::Result<()> {
    let config = config::load_file("config.toml", true, &Config::default()).await?;
    let saved_data = config::load_file("saved_data.toml", false, &SavedData::default()).await?;
    unsafe {
        config::SAVED_DATA = Some(saved_data);
        config::CONFIG = Some(config);
    }

    let token = env::var("DISCORD_TOKEN").expect("Cannot find discord token.");

    let intents = GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(&token, intents)
        .event_handler(event::Handler)
        .await?;

    client.start().await?;

    Ok(())
}
