#![forbid(unsafe_code)]
mod links_file;

use anyhow::Result;
use chrono::{Duration, Utc};
use handlebars::Handlebars;
use humantime::format_duration;
use links_file::{LinksFile, URL_REGEX};
use regex::Regex;
use scrape_lobsters::LobstersEntry;
use scrape_reddit::RedditEntry;
use serde_json::json;
use serenity::{
    async_trait,
    builder::{CreateAttachment, CreateMessage, GetMessages},
    gateway::ActivityData,
    model::{
        channel::{Embed, Message, ReactionType},
        id::{ChannelId, MessageId},
        prelude::*,
    },
    prelude::*,
};
use std::{collections::HashSet, env, time::Instant};
use tokio::join;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

pub const GIT_HASH: &str = env!("GIT_HASH");

static HELP_MESSAGE: &str = "
**try**:
`!help`
`!collect` - will collect all new entries to add
`!ack` - will put checkboxes on all found entries
`!scrape` - will scrape reddit for the last 7 days

version: {{version}} - uptime: {{uptime}} - channel: {{channel}}
";

static ACK_MSG: &str = "
acknowledged: {{count}} entries
";

const CMD_COLLECT: &str = "!collect";
const CMD_SCRAPE: &str = "!scrape";
const CMD_ACK: &str = "!ack";
const HELP_COMMAND: &str = "!help";

const UNICODE_CHECKBOX: &str = "✅";
const UNICODE_DUPLICATE: &str = "🇩";

struct CollectEntry {
    pub title: Option<String>,
    pub url: String,
    msg_id: MessageId,
}

struct Handler {
    regex_url: Regex,
    ignore_emojis: HashSet<String>,
    start_time: Instant,
    links_file: Option<LinksFile>,
    channel_id: Option<ChannelId>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let is_target_channel = self
            .channel_id
            .map(|channel| msg.channel_id == channel)
            .unwrap_or_default();

        if !is_target_channel && !msg.is_private() {
            tracing::info!(
                "msg filtered! channel: {} - target channel: {:?} - private: {}",
                msg.channel_id,
                self.channel_id,
                msg.is_private()
            );
            return;
        };

        tracing::info!("msg channel ok! msg: '{}'", msg.content.as_str(),);

        ctx.set_activity(Some(ActivityData::playing(msg.content.as_str())));

        let res = match msg.content.as_str() {
            CMD_COLLECT => self.collect_cmd(&ctx, &msg).await,
            CMD_ACK => self.ack_cmd(&ctx, &msg).await,
            CMD_SCRAPE => self.scrape_cmd(&ctx, &msg).await,
            HELP_COMMAND => self.help_cmd(&ctx, &msg).await,
            _ => self.no_cmd(&ctx, &msg).await,
        };

        if let Err(err) = res {
            tracing::error!("cmd error: {}", err);

            let _ = self.cmd_error(&ctx, &msg, err).await;
        }

        ctx.reset_presence();
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!("bot connected: {}", ready.user.name);

        ctx.set_activity(Some(ActivityData::playing("ready".to_string())));

        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;

            ctx.reset_presence();
        });
    }
}

impl Handler {
    fn new(links_file: Option<LinksFile>) -> Self {
        let mut ignore_emojis = HashSet::new();
        ignore_emojis.insert(UNICODE_DUPLICATE.into());
        ignore_emojis.insert("\u{1F1EE}".into()); //🇮
        ignore_emojis.insert("🛑".into());

        let channel_id = env::var("CHANNEL_ID")
            .ok()
            .and_then(|channel| channel.parse::<ChannelId>().ok());

        tracing::info!("target channel: {:?}", channel_id);

        Self {
            regex_url: URL_REGEX.clone(),
            ignore_emojis,
            start_time: Instant::now(),
            links_file,
            channel_id,
        }
    }

    async fn help_cmd(&self, ctx: &Context, msg: &Message) -> Result<()> {
        let up_time = self.start_time.elapsed();
        let up_time = format_duration(up_time).to_string();
        let reg = Handlebars::new();
        let channel = self.channel_id.unwrap_or_default();
        let msg_string = reg.render_template(
            HELP_MESSAGE,
            &json!({ "version": GIT_HASH, "uptime": up_time, "channel": channel }),
        )?;

        msg.channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .content(msg_string)
                    .reference_message(msg),
            )
            .await?;

        Ok(())
    }

    async fn cmd_error(&self, ctx: &Context, msg: &Message, err: anyhow::Error) -> Result<()> {
        msg.channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .content(format!("cmd error: {}", err))
                    .reference_message(msg),
            )
            .await?;

        Ok(())
    }

    async fn ack_cmd(&self, ctx: &Context, msg: &Message) -> Result<()> {
        let (entries, _) = self.gather_entries(msg, ctx).await?;

        for e in &entries {
            msg.channel_id.broadcast_typing(&ctx.http).await?;

            msg.channel_id
                .create_reaction(
                    &ctx.http,
                    e.msg_id,
                    ReactionType::Unicode(String::from(UNICODE_CHECKBOX)),
                )
                .await?;
        }

        let reg = Handlebars::new();
        let msg_string = reg.render_template(ACK_MSG, &json!({ "count": entries.len() }))?;

        msg.channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .content(msg_string)
                    .reference_message(msg),
            )
            .await?;

        Ok(())
    }

    async fn no_cmd(&self, ctx: &Context, msg: &Message) -> Result<()> {
        tracing::info!("no_cmd handler: {}", msg.link());

        if self.ignore_msg(msg) {
            tracing::info!("ignore msg: {}", msg.link());

            return Ok(());
        }

        if let Some(capture) = self.find_url(&msg.content) {
            if let Some(links_file) = &self.links_file {
                if links_file.contains(capture).await {
                    msg.channel_id
                        .create_reaction(
                            &ctx.http,
                            msg.id,
                            ReactionType::Unicode(String::from(UNICODE_DUPLICATE)),
                        )
                        .await?;
                }
            }
        }

        Ok(())
    }

    async fn collect_cmd(&self, ctx: &Context, msg: &Message) -> Result<()> {
        msg.channel_id.broadcast_typing(&ctx.http).await?;

        let (entries, stop_msg_link) = self.gather_entries(msg, ctx).await?;

        let (msg_content, attachement_content) =
            Self::create_collect_response(stop_msg_link, entries)?;

        msg.channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .content(msg_content)
                    .add_file(CreateAttachment::bytes(
                        attachement_content.as_bytes(),
                        "list.md",
                    ))
                    .reference_message(msg),
            )
            .await?;

        Ok(())
    }

    async fn scrape_cmd(&self, ctx: &Context, msg: &Message) -> Result<()> {
        msg.channel_id.broadcast_typing(&ctx.http).await?;

        let until = Utc::now() - Duration::days(7);
        let (reddit, lobsters): (Vec<RedditEntry>, Vec<LobstersEntry>) = join!(
            scrape_reddit::scrape_reddit(until),
            scrape_lobsters::scrape_lobsters(until)
        );

        msg.channel_id.broadcast_typing(&ctx.http).await?;

        let reddit_file = reddit
            .iter()
            .map(|entry| format!("[{}] {}", entry.time, entry.url))
            .collect::<Vec<_>>()
            .join("\n");
        let lobsters_file = lobsters
            .iter()
            .map(|entry| format!("[{}] {}", entry.time, entry.url))
            .collect::<Vec<_>>()
            .join("\n");

        msg.channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .content(format!(
                        "scrape results: {}/{}",
                        reddit.len(),
                        lobsters.len()
                    ))
                    .add_file(CreateAttachment::bytes(reddit_file.as_bytes(), "reddit.md"))
                    .add_file(CreateAttachment::bytes(
                        lobsters_file.as_bytes(),
                        "lobsters.md",
                    ))
                    .reference_message(msg),
            )
            .await?;

        Ok(())
    }

    async fn gather_entries(
        &self,
        msg: &Message,
        ctx: &Context,
    ) -> Result<(Vec<CollectEntry>, Option<String>)> {
        let mut message_cursor = msg.id;
        let mut entries: Vec<CollectEntry> = Vec::new();
        let mut stop_msg_link = None;

        'outer: loop {
            let messages = msg
                .channel_id
                .messages(&ctx.http, GetMessages::new().before(message_cursor))
                .await?;

            if messages.is_empty() {
                break;
            }

            for m in messages {
                if Self::is_stop_msg(&m) {
                    tracing::info!("stop found: {}", m.link());
                    stop_msg_link = Some(m.link());
                    break 'outer;
                }

                if self.ignore_msg(&m) {
                    tracing::info!("ignored msg: {}", m.link());
                    continue;
                }

                if let Some(capture) = self.find_url(&m.content) {
                    let url = capture.as_str().to_string();

                    let title = Self::get_link_title(&m.embeds);
                    entries.push(CollectEntry {
                        title,
                        url,
                        msg_id: m.id,
                    });
                }

                message_cursor = m.id;
            }
        }

        Ok((entries, stop_msg_link))
    }

    fn find_url(&self, msg: &str) -> Option<String> {
        self.regex_url
            .captures(msg)
            .and_then(|captures| captures.get(0))
            .map(|m| m.as_str().to_string())
    }

    fn is_stop_msg(msg: &Message) -> bool {
        msg.reactions.iter().any(|reaction| {
            reaction.reaction_type == ReactionType::Unicode(String::from(UNICODE_CHECKBOX))
        })
    }

    fn get_link_title(embeds: &[Embed]) -> Option<String> {
        embeds.iter().next().and_then(|embed| embed.title.clone())
    }

    fn ignore_msg(&self, msg: &Message) -> bool {
        let ignore_reaction = msg.reactions.iter().any(|reaction| {
            if let ReactionType::Unicode(emoji) = &reaction.reaction_type {
                self.ignore_emojis.contains(emoji)
            } else {
                false
            }
        });

        msg.author.bot || ignore_reaction
    }

    fn create_collect_response(
        stop_msg_link: Option<String>,
        entries: Vec<CollectEntry>,
    ) -> Result<(String, String)> {
        // let reg = Handlebars::new();
        let meta_string = format!(
            "entries: {}\nstop msg: {}",
            entries.len(),
            stop_msg_link.unwrap_or_default()
        );

        let content_string = entries
            .into_iter()
            .map(|entry| {
                format!(
                    "* [{}]({})",
                    entry.title.unwrap_or_else(|| String::from("TODO")),
                    entry.url
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        Ok((meta_string, content_string))
    }
}

#[cfg(not(debug_assertions))]
#[must_use]
pub const fn is_debug() -> bool {
    false
}

#[cfg(debug_assertions)]
#[must_use]
pub const fn is_debug() -> bool {
    true
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_ansi(is_debug()))
        .init();

    tracing::info!("bot starting");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    tracing::info!("bot starting: parse link file");

    let links_file = LinksFile::new()
        .await
        .expect("Expected links file to download");

    tracing::info!("bot starting: init handler");

    let h = Handler::new(Some(links_file));

    tracing::info!("bot starting: init discord client");

    let mut client = Client::builder(
        &token,
        GatewayIntents::default().union(GatewayIntents::MESSAGE_CONTENT),
    )
    .event_handler(h)
    .await
    .expect("Err creating client");

    tracing::info!("bot starting: starting discord client");

    if let Err(why) = client.start().await {
        tracing::error!("bot error: {:?}", why);
    }
}

#[cfg(test)]
mod test {
    use crate::Handler;
    use serde_json::json;
    use serenity::model::channel::Embed;

    #[test]
    fn test_embed_title() {
        let embed = serde_json::from_value::<Embed>(json!({
            "title": "test-title".to_string(),
            "type": "rich"
        }))
        .unwrap();

        let res = Handler::get_link_title(&[embed]);
        assert_eq!(&res.unwrap(), "test-title");
    }

    #[test]
    fn test_url_filter() {
        let h = Handler::new(None);

        let url = "https://nadim.computer/posts/2022-02-11-maccatalyst.html";
        assert_eq!(h.find_url(url).unwrap(), url);

        assert_eq!(h.find_url(&format!("foo bar {}", url)).unwrap(), url);
    }
}
