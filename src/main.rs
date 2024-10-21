mod config;
mod error;
mod event;

use serenity::prelude::GatewayIntents;
use serenity::Client;
use std::env;

#[tokio::main]
async fn main() -> error::Result<()> {
    let config = config::load_config().await?;
    unsafe {
        config::CONFIG = Some(&config);
    }

    let token = env::var("DISCORD_TOKEN").expect("Cannot find discord token.");

    let intents = GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(&token, intents)
        .event_handler(event::Handler)
        .await?;

    client.start()?
}
