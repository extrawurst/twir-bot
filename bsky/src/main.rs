use std::env;

use atrium_api::types::string::Datetime;
use bsky_sdk::{BskyAgent, rich_text::RichText};
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = BskyAgent::builder().build().await?;
    agent
        .login(env::var("BSKY_USR").unwrap(), env::var("BSKY_KEY").unwrap())
        .await?;

    let url = env::args().nth(1).unwrap();

    println!("url provided: {}", url);

    let regex = Regex::new(r"this-week-in-rust-(\d*)").unwrap();

    let version = regex.captures(&url).unwrap()[1].parse::<u32>().unwrap();

    println!("version extracted: {}", version);

    let rt = RichText::new_with_detect_facets(format!(
        "This week in #rust {} {} #rustlang",
        version, url
    ))
    .await?;

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
    Ok(())
}
