use std::num::ParseIntError;

use atrium_api::types::string::Datetime;
use bsky_sdk::{BskyAgent, rich_text::RichText};
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BskyPostingError {
    #[error("regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("parse int erro: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Link parse error")]
    LinkParseError,
    #[error("bsky error: {0}")]
    BskyError(#[from] bsky_sdk::Error),
    #[error("atrium rpc session error: {0}")]
    AtriumRPCSessionError(
        #[from] atrium_api::xrpc::Error<atrium_api::com::atproto::server::create_session::Error>,
    ),
    #[error("Did parsing error")]
    InvalidDid,
    #[error("rkey parsing error")]
    InvalidRKey,
}

pub async fn bsky_post(
    url: &str,
    bsky_usr: &str,
    bsky_key: &str,
) -> Result<(String, u32, String), BskyPostingError> {
    let regex = Regex::new(r"this-week-in-rust-(\d*)")?;

    let version = regex
        .captures(url)
        .ok_or(BskyPostingError::LinkParseError)?[1]
        .parse::<u32>()?;

    let msg = format!("This week in #rust {} {} #rustlang", version, url);

    let rt = RichText::new_with_detect_facets(msg.clone()).await?;

    let agent = {
        let agent = BskyAgent::builder().build().await?;
        agent.login(bsky_usr, bsky_key).await?;
        agent
    };

    let obj = agent
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

    let rkey = obj
        .uri
        .split("/")
        .last()
        .ok_or(BskyPostingError::InvalidRKey)?;

    let did = agent
        .did()
        .await
        .ok_or(BskyPostingError::InvalidDid)?
        .to_string();

    let url = format!("https://bsky.app/profile/{}/post/{}", did, rkey);

    Ok((msg, version, url))
}
