use core::fmt;

use atrium_api::types::string::Datetime;
use bsky_sdk::{BskyAgent, rich_text::RichText};
use regex::Regex;

#[derive(Debug)]
pub struct BskyLinkParsingError;

impl std::error::Error for BskyLinkParsingError {}

impl fmt::Display for BskyLinkParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BskyLinkParsingError")
    }
}

pub async fn bsky_post(
    url: &str,
    bsky_usr: &str,
    bsky_key: &str,
) -> Result<(String, u32), Box<dyn std::error::Error>> {
    let regex = Regex::new(r"this-week-in-rust-(\d*)")?;

    let version = regex.captures(url).ok_or(Box::new(BskyLinkParsingError))?[1].parse::<u32>()?;

    let msg = format!("This week in #rust {} {} #rustlang", version, url);

    let rt = RichText::new_with_detect_facets(msg.clone()).await?;

    let agent = {
        let agent = BskyAgent::builder().build().await?;
        agent.login(bsky_usr, bsky_key).await?;
        agent
    };

    agent
        .create_record(atrium_api::app::bsky::feed::post::RecordData {
            created_at: Datetime::now(),
            embed: None,
            entities: None,
            facets: rt.facets,
            labels: None,
            langs: None,
            reply: None,
            tags: None,
            text: rt.text,
        })
        .await?;

    Ok((msg, version))
}
