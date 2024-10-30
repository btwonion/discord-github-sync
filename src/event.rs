use crate::config;
use crate::config::{Config, SavedData};
use serenity::all::{Channel, GuildChannel, Message, Ready};
use serenity::async_trait;
use serenity::prelude::{Context, EventHandler};
use crate::github::{process_channel_update, process_message};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn channel_create(&self, _ctx: Context, channel: GuildChannel) {
        process_child_channel(channel).await;
    }

    async fn channel_update(&self, _ctx: Context, _: Option<GuildChannel>, new: GuildChannel) {
        if let Some(result) = process_child_channel(new).await {
            process_channel_update(result).await;
        }
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        let channel_result = new_message.channel_id.to_channel(&ctx).await;
        if channel_result.is_err() {
            return;
        }
        let channel = channel_result.unwrap();
        match channel {
            Channel::Private(_) => return,
            Channel::Guild(channel) => {
                if let Some(result) = process_child_channel(channel).await {
                    process_message(result, new_message).await;
                }
            }
            _ => {}
        }
    }

    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!(
            "The bot successfully connected as {}.",
            data_about_bot.user.name
        )
    }
}

pub struct ForumIssueResult {
    pub post_channel: GuildChannel,
    pub forum_id: u64,
    pub config: Config,
    pub saved_data: SavedData,
    pub repo: String,
    pub issue: Option<i32>,
}

async fn process_child_channel(channel: GuildChannel) -> Option<ForumIssueResult> {
    let config = unsafe { config::CONFIG.clone().unwrap() };
    if !validate_channel(&channel, &config).await {
        return None;
    }
    let parent_id = channel.parent_id.unwrap().get();

    let forum_tags = channel
        .applied_tags
        .iter()
        .map(|t| t.get())
        .collect::<Vec<_>>();

    let repo = identify_repo(&forum_tags, &config).await;
    if repo.is_none() {
        return None;
    }
    let saved_data = unsafe { config::SAVED_DATA.clone().unwrap() };
    let issue = get_issue(&parent_id, &saved_data).await;
    Some(ForumIssueResult {
        post_channel: channel,
        forum_id: parent_id,
        config,
        saved_data,
        repo: repo.unwrap(),
        issue,
    })
}

async fn validate_channel(channel: &GuildChannel, config: &Config) -> bool {
    if let Some(parent_id) = channel.parent_id {
        if !config.forum_channel_ids.contains(&parent_id.get()) {
            return false;
        }
        println!(
            "Triggered update in {}({}), child of {}.",
            channel.name,
            channel.id,
            parent_id.get()
        );

        return true;
    }
    false
}

async fn identify_repo(tags: &Vec<u64>, config: &Config) -> Option<String> {
    let mut associated_repo = None;
    for tag in tags {
        if let Some(repo) = config.tag_to_repo.get(&tag) {
            associated_repo = Some(repo.to_string());
            break;
        }
    }
    associated_repo
}

async fn get_issue(channel_id: &u64, saved_data: &SavedData) -> Option<i32> {
    saved_data.channel_id_to_issue.get(channel_id).cloned()
}
