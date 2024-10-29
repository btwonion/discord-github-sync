use crate::config;
use crate::config::Config;
use serenity::all::{Channel, GuildChannel, Message, Ready};
use serenity::async_trait;
use serenity::prelude::{Context, EventHandler};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn channel_create(&self, ctx: Context, channel: GuildChannel) {
        process_child_channel(&ctx, channel).await;
    }

    async fn channel_update(&self, ctx: Context, _: Option<GuildChannel>, new: GuildChannel) {
        process_child_channel(&ctx, new).await
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        let channel_result = new_message.channel_id.to_channel(&ctx).await;
        if channel_result.is_err() {
            return;
        }
        let channel = channel_result.unwrap();
        match channel {
            Channel::Private(_) => return,
            Channel::Guild(channel) => process_child_channel(&ctx, channel).await,
            _ => {}
        }
    }

    async fn ready(&self, _: Context, data_about_bot: Ready) {
        println!(
            "The bot successfully connected as {}.",
            data_about_bot.user.name
        )
    }
}

async fn process_child_channel(ctx: &Context, channel: GuildChannel) {
    if let Some(config) = validate_channel(channel) {
        let tags = channel
            .applied_tags
            .iter()
            .map(|t| t.get())
            .collect::<Vec<_>>();
        if tags.is_empty() {
            return;
        }
        let mut associated_repo = None;
        for tag in tags {
            if let Some(repo) = config.tag_to_repo.get(&tag) {
                associated_repo = Some(repo);
                break;
            }
        }
        if associated_repo.is_none() {
            return;
        }
        let saved_data = unsafe { config::SAVED_DATA.clone().unwrap() };
    }
}

async fn validate_channel(channel: GuildChannel) -> Option<Config> {
    if let Some(parent_id) = channel.parent_id {
        let config = unsafe { config::CONFIG.clone().unwrap() };
        if !config.forum_channel_ids.contains(&parent_id.get()) {
            return None;
        }
        println!(
            "Triggered update in {}({}), child of {}.",
            channel.name,
            channel.id,
            parent_id.get()
        );

        return Some(config);
    }
    None
}
