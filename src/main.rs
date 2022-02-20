#![forbid(unsafe_code)]

use anyhow::Result;
use regex::Regex;
use serenity::{
    async_trait,
    http::AttachmentType,
    model::{
        channel::{Embed, Message, ReactionType},
        gateway::Ready,
    },
    prelude::*,
};
use std::{collections::HashSet, env};

static HELP_MESSAGE: &str = "
try:
!help
!collect
";

const CMD_COLLECT: &str = "!collect";
const HELP_COMMAND: &str = "!help";

struct CollectEntry {
    pub title: Option<String>,
    pub url: String,
}

struct Handler {
    pub regex_url: Regex,
    ignore_emojis: HashSet<String>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let res = match msg.content.as_str() {
            CMD_COLLECT => self.collect_cmd(&ctx, &msg).await,
            HELP_COMMAND => self.help_cmd(&ctx, &msg).await,
            _ => Ok(()),
        };

        if let Err(err) = res {
            tracing::error!("cmd error: {}", err);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("bot connected: {}", ready.user.name);
    }
}

impl Handler {
    fn new() -> Self {
        let mut ignore_emojis = HashSet::new();
        ignore_emojis.insert("🇩".into());
        ignore_emojis.insert("\u{1F1EE}".into()); //🇮
        ignore_emojis.insert("🛑".into());

        Self {
            // slighly modified:
            // https://www.geeksforgeeks.org/python-check-url-string/
            regex_url: Regex::new(
                r#"(?i)\b((?:https?://|www\d{0,3}[.]|[a-z0-9.\-]+[.][a-z]{2,4}/)(?:[^\s()<>]+|\(([^\s()<>]+|(\([^\s()<>]+\)))*\))+(?:\(([^\s()<>]+|(\([^\s()<>]+\)))*\)|[^\s`!()\[\]{};:'.,<>?«»“”‘’]))"#,
            ).unwrap(),
            ignore_emojis
        }
    }

    async fn help_cmd(&self, ctx: &Context, msg: &Message) -> Result<()> {
        msg.channel_id.say(&ctx.http, HELP_MESSAGE).await?;

        Ok(())
    }

    async fn collect_cmd(&self, ctx: &Context, msg: &Message) -> Result<()> {
        let mut message_cursor = msg.id;
        let mut entries: Vec<CollectEntry> = Vec::new();
        let mut stop_msg_link = None;

        'outer: loop {
            let messages = msg
                .channel_id
                .messages(&ctx.http, |retriever| retriever.before(message_cursor))
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

                    // tracing::info!("match: '{}': {} ({})", m.author.name, m.link(), url);
                    // tracing::info!("match: {:?}", m);

                    let title = Self::get_link_title(&m.embeds);
                    entries.push(CollectEntry { title, url });
                }

                message_cursor = m.id;
            }
        }

        let (msg_content, attachement_content) =
            Self::create_collect_response(stop_msg_link, entries)?;

        let att: AttachmentType = (attachement_content.as_bytes(), "list.md").into();
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.content(msg_content);
                m.add_file(att);
                m.reference_message(msg);
                m
            })
            .await?;

        Ok(())
    }

    fn find_url(&self, msg: &str) -> Option<String> {
        self.regex_url
            .captures(msg)
            .and_then(|captures| captures.get(0))
            .map(|m| m.as_str().to_string())
    }

    fn is_stop_msg(msg: &Message) -> bool {
        msg.reactions
            .iter()
            .any(|reaction| reaction.reaction_type == ReactionType::Unicode(String::from("✅")))
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("bot starting");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let h = Handler::new();

    let mut client = Client::builder(&token)
        .event_handler(h)
        .await
        .expect("Err creating client");

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
        let h = Handler::new();

        let url = "https://nadim.computer/posts/2022-02-11-maccatalyst.html";
        assert_eq!(h.find_url(url).unwrap(), url);

        assert_eq!(h.find_url(&format!("foo bar {}", url)).unwrap(), url);
    }
}
